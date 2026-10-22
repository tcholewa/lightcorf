//! Restart-only RAW policy: a saved macOS processor choice, native Rust elsewhere. No decoder fallback.
use lightcraft_pipeline::SourceInfo;
use lightcraft_raster::{
    Rgb32f,
    resample::{Filter, fit},
};
use lightcraft_raw_backend::{BackendError, Decoded, Info, Operation};

pub const SENSOR_UNAVAILABLE: &str = "The active Core Image processor receives developed RGB, not sensor samples. AI RAW denoise, sensor-DNG conversion and camera calibration are unavailable; RGB editing and ordinary exports remain available.";
/// Product identity is independent of the selected decoder.
pub const fn coreimage_edition() -> bool {
    cfg!(all(target_os = "macos", not(test)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RawBackend {
    CoreImage,
    Native,
}

impl RawBackend {
    pub const fn platform_default() -> Self {
        if cfg!(target_os = "macos") { Self::CoreImage } else { Self::Native }
    }

    pub fn available(self) -> bool {
        self == Self::Native || cfg!(target_os = "macos")
    }

    pub fn stale(self, stamp: Option<&str>) -> bool {
        match self {
            Self::Native => stamp.is_some(),
            Self::CoreImage => {
                let build = core_stamp().rsplit(':').next().unwrap_or("");
                !stamp.is_some_and(|s| {
                    s.strip_prefix(lightcraft_raw_backend::RECIPE)
                        .is_some_and(|rest| rest.starts_with(':') && rest.split(|c: char| !c.is_ascii_alphanumeric()).any(|part| part == build))
                })
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Preference {
    backend: RawBackend,
}

/// One immutable backend for the entire host process, shared by all sessions and worker jobs.
/// Saving a preference changes only the next launch; no running loader can change decoder.
struct Runtime {
    active: RawBackend,
    selected: std::sync::Mutex<RawBackend>,
    path: Option<std::path::PathBuf>,
    error: Option<String>,
}

impl Runtime {
    fn load(path: Option<std::path::PathBuf>, default: RawBackend) -> Self {
        let result = path.as_deref().map_or(Ok(default), |path| {
            use std::io::Read;
            let file = match std::fs::File::open(path) {
                Ok(file) => file,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(default),
                Err(e) => return Err(format!("Cannot read RAW processing settings: {e}")),
            };
            let mut bytes = Vec::new();
            file.take(4097).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
            if bytes.len() > 4096 {
                return Err("RAW processing settings exceed 4 KiB".into());
            }
            let p: Preference = serde_json::from_slice(&bytes).map_err(|e| format!("Invalid RAW processing settings: {e}"))?;
            if !p.backend.available() {
                return Err("Apple Core Image requires macOS. Select LightCraft Native in Settings and restart.".into());
            }
            Ok(p.backend)
        });
        let (active, error) = match result {
            Ok(value) => (value, None),
            Err(error) => (default, Some(format!("{error}. Choose a RAW processor in Settings and restart; RAW processing is paused."))),
        };
        Self { active, selected: std::sync::Mutex::new(active), path, error }
    }

    fn selected(&self) -> RawBackend {
        *self.selected.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn save(&self, backend: RawBackend) -> Result<(), String> {
        if !backend.available() {
            return Err("Apple Core Image requires macOS".into());
        }
        let path = self.path.as_deref().ok_or("RAW processor preferences are unavailable without a configuration directory")?;
        let bytes = serde_json::to_vec_pretty(&Preference { backend }).map_err(|e| e.to_string())?;
        let mut selected = self.selected.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        lightcraft_catalog::safe_file::write_atomic(path, &bytes).map_err(|e| format!("Cannot save RAW processor choice: {e}"))?;
        *selected = backend;
        Ok(())
    }
}

fn runtime() -> &'static Runtime {
    static RUNTIME: std::sync::OnceLock<Runtime> = std::sync::OnceLock::new();
    RUNTIME.get_or_init(|| {
        // Existing unit fixtures explicitly use the native reference engine, without personal settings.
        // Production policy and restart behavior are tested in isolated child processes.
        if cfg!(test) {
            Runtime::load(None, RawBackend::Native)
        } else {
            Runtime::load(crate::config::config_dir().map(|d| d.join("raw-processing.json")), RawBackend::platform_default())
        }
    })
}

pub fn active() -> RawBackend {
    runtime().active
}
pub fn enabled() -> bool {
    active() == RawBackend::CoreImage
}
pub fn startup_error() -> Option<&'static str> {
    runtime().error.as_deref()
}
pub fn ensure_ready() -> Result<(), String> {
    startup_error().map_or(Ok(()), |e| Err(e.into()))
}

pub fn preferences() -> serde_json::Value {
    let r = runtime();
    let selected = r.selected();
    serde_json::json!({"active": r.active, "selected": selected,
        "restartRequired": selected != r.active || r.error.is_some(),
        "persistent": r.path.is_some(), "error": r.error,
        "available": if cfg!(target_os="macos") { vec![RawBackend::CoreImage, RawBackend::Native] } else { vec![RawBackend::Native] },
        "capabilities": diagnostics()})
}

pub fn set_preference(backend: RawBackend) -> Result<(), String> {
    runtime().save(backend)
}

pub fn require_sensor() -> Result<(), String> {
    ensure_ready()?;
    if enabled() { Err(SENSOR_UNAVAILABLE.into()) } else { Ok(()) }
}

pub fn request(operation: Operation, bytes: &[u8]) -> Result<Decoded, BackendError> {
    if bytes.is_empty() || bytes.len() > lightcraft_raw_backend::MAX_INPUT {
        return Err(BackendError::decode("RAW input is empty or exceeds 1 GiB"));
    }
    #[cfg(target_os = "macos")]
    {
        static CLIENT: std::sync::OnceLock<Result<lightcraft_raw_backend::Client, String>> = std::sync::OnceLock::new();
        let client = CLIENT.get_or_init(lightcraft_raw_backend::Client::installed).as_ref().map_err(|e| BackendError::decode(e.clone()))?;
        client.request(operation, std::sync::Arc::from(bytes))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (operation, bytes);
        Err(BackendError::unsupported("Core Image requires macOS"))
    }
}
pub fn probe(bytes: &[u8]) -> Result<Info, BackendError> {
    request(Operation::Probe, bytes).map(|d| d.info)
}
pub fn white_balance(info: &Info) -> (f64, f64) {
    let [x, y] = info.neutral_xy;
    lightcraft_color::cct::xy_to_temp_tint(lightcraft_color::Xy::new(x, y))
}
pub fn decode(bytes: &[u8], max_edge: usize) -> Result<(Rgb32f, SourceInfo), BackendError> {
    let decoded = request(Operation::Decode, bytes)?;
    let (temp, tint) = white_balance(&decoded.info);
    let source = SourceInfo {
        raw: true,
        as_shot_temp: temp,
        as_shot_tint: tint,
        native_long: Some(decoded.info.width.max(decoded.info.height)),
        raw_backend: Some(decoded.info.stamp()),
        ..Default::default()
    };
    let image = decoded.image.ok_or_else(|| BackendError::decode("Core Image returned no RAW pixels"))?;
    let image = if image.width.max(image.height) > max_edge.max(1) { fit(&image, max_edge.max(1), max_edge.max(1), Filter::Box) } else { image };
    Ok((image, source))
}
/// The OS build is part of every RAW cache key, including older catalog records. Fetch once.
fn core_stamp() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        static STAMP: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        STAMP.get_or_init(|| {
            let build = std::process::Command::new("/usr/bin/sw_vers")
                .arg("-buildVersion")
                .output()
                .ok()
                .filter(|r| r.status.success())
                .and_then(|r| String::from_utf8(r.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| format!("unknown-os-{}", std::process::id()));
            format!("{}:{build}", lightcraft_raw_backend::RECIPE)
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        "rust"
    }
}
pub fn runtime_stamp() -> &'static str {
    if enabled() { core_stamp() } else { "native-v1" }
}

pub fn stale(stamp: Option<&str>) -> bool {
    active().stale(stamp)
}

pub fn photo_stale(p: &lightcraft_catalog::Photo) -> bool {
    p.kind == lightcraft_catalog::MediaKind::Raw
        && matches!(p.source, lightcraft_catalog::Source::File { .. })
        && (startup_error().is_some() || stale(p.raw_backend.as_deref()))
}

pub fn diagnostics() -> serde_json::Value {
    serde_json::json!({"backend": if enabled(){"coreimage"}else{"native"},"runtime":runtime_stamp(),
        "sensorAccess":!enabled(),"strict":enabled(),"recipe":if enabled(){Some(lightcraft_raw_backend::RECIPE)}else{None}})
}

/// Never save/relink a generated DNG that the active decoder cannot reopen.
pub fn validate_generated_dng(bytes: &[u8]) -> Result<(), String> {
    ensure_ready()?;
    if enabled() {
        request(Operation::Decode, bytes).map_err(|e| format!("Generated DNG cannot be reopened by Core Image: {e}; no file was saved"))?;
    }
    Ok(())
}

#[cfg(test)]
mod preference_tests {
    use super::*;

    fn path(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("lc-raw-choice-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("raw-processing.json")
    }

    #[test]
    fn saved_choice_only_changes_the_next_runtime_and_can_be_cancelled() {
        let path = path("restart");
        let _ = std::fs::remove_file(&path);
        let first = Runtime::load(Some(path.clone()), RawBackend::Native);
        assert_eq!(first.active, RawBackend::Native);
        let other = if cfg!(target_os = "macos") { RawBackend::CoreImage } else { RawBackend::Native };
        first.save(other).unwrap();
        assert_eq!(first.active, RawBackend::Native);
        assert_eq!(first.selected(), other);
        let second = Runtime::load(Some(path.clone()), RawBackend::Native);
        assert_eq!(second.active, other);
        first.save(first.active).unwrap();
        assert_eq!(first.active, first.selected());
        assert_eq!(Runtime::load(Some(path.clone()), other).active, RawBackend::Native);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn invalid_preferences_are_preserved_and_reported() {
        let path = path("invalid");
        for bytes in [b"{broken".to_vec(), br#"{"backend":"automatic"}"#.to_vec(), vec![b' '; 4097]] {
            std::fs::write(&path, &bytes).unwrap();
            let runtime = Runtime::load(Some(path.clone()), RawBackend::Native);
            assert!(runtime.error.is_some());
            assert_eq!(std::fs::read(&path).unwrap(), bytes);
            runtime.save(RawBackend::Native).unwrap();
            assert!(runtime.error.is_some(), "repair still requires restart");
            assert!(Runtime::load(Some(path.clone()), RawBackend::Native).error.is_none());
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn failed_save_keeps_selection_and_disabled_storage_writes_nothing() {
        let path = path("failure");
        std::fs::write(&path, b"parent is a file").unwrap();
        let runtime = Runtime::load(Some(path.join("child")), RawBackend::Native);
        let old = runtime.selected();
        assert!(runtime.save(RawBackend::Native).is_err());
        assert_eq!(runtime.selected(), old);
        assert_eq!(std::fs::read(&path).unwrap(), b"parent is a file");
        assert!(Runtime::load(None, RawBackend::Native).save(RawBackend::Native).is_err());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn backend_staleness_is_bidirectional_and_recipe_sensitive() {
        assert!(!RawBackend::Native.stale(None));
        assert!(RawBackend::CoreImage.stale(None));
        assert!(RawBackend::Native.stale(Some("coreimage-linear-rec2020-v1:8:old-os")));
        assert!(RawBackend::CoreImage.stale(Some("coreimage-linear-rec2020-v0:8:old-os")));
        if cfg!(target_os = "macos") {
            assert!(!RawBackend::CoreImage.stale(Some(core_stamp())));
            let build = core_stamp().rsplit(':').next().unwrap();
            let worker = format!("{}:8:Version 27.0.1 (Build {build})", lightcraft_raw_backend::RECIPE);
            assert!(!RawBackend::CoreImage.stale(Some(&worker)), "worker OS description and sw_vers build identify the same runtime");
            assert!(RawBackend::CoreImage.stale(Some(&worker.replace(build, &format!("{build}0")))));
        }
    }
}
