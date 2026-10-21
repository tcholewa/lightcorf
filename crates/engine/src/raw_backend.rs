//! Platform RAW policy: strict Core Image on macOS, Rust elsewhere. No sensor fallback.
use lightcraft_pipeline::SourceInfo;
use lightcraft_raster::{
    Rgb32f,
    resample::{Filter, fit},
};
use lightcraft_raw_backend::{BackendError, Decoded, Info, Operation};

pub const SENSOR_UNAVAILABLE: &str = "This Core Image build receives developed RGB, not sensor samples. AI RAW denoise, sensor-DNG conversion and camera calibration are unavailable; RGB editing and ordinary exports remain available.";
// The engine's existing unit fixtures exercise Rust sensor algorithms on every host.
// macOS production policy is tested through integration tests (which link the normal library)
// and real app/CLI binaries. There is deliberately no runtime Rust override in the product.
pub const fn enabled() -> bool {
    cfg!(all(target_os = "macos", not(test)))
}
pub fn require_sensor() -> Result<(), String> {
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
pub fn runtime_stamp() -> &'static str {
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
pub fn stale(stamp: Option<&str>) -> bool {
    if !enabled() {
        return false;
    }
    let build = runtime_stamp().rsplit(':').next().unwrap_or("");
    !stamp.is_some_and(|s| s.starts_with(lightcraft_raw_backend::RECIPE) && s.contains(build))
}
pub fn diagnostics() -> serde_json::Value {
    serde_json::json!({"backend": if enabled(){"coreimage"}else{"rust"},"runtime":runtime_stamp(),
        "sensorAccess":!enabled(),"strict":enabled(),"recipe":if enabled(){Some(lightcraft_raw_backend::RECIPE)}else{None}})
}

/// Never save/relink a generated DNG that the active decoder cannot reopen.
pub fn validate_generated_dng(bytes: &[u8]) -> Result<(), String> {
    if enabled() {
        request(Operation::Decode, bytes).map_err(|e| format!("Generated DNG cannot be reopened by Core Image: {e}; no file was saved"))?;
    }
    Ok(())
}
