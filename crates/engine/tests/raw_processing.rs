// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! A real process restart freezes a saved choice, refreshes existing photos and isolates caches.
#![cfg(target_os = "macos")]
use lightcraft_engine::{Session, raw_backend};
use serde_json::json;
use std::path::PathBuf;

#[test]
fn processor_choice_survives_real_restarts_in_both_directions() {
    let samples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../_orfs");
    if !samples.join("_7190788.ORF").is_file() {
        lightcraft_testkit::skip!(Corpus, "private RAW processor switching samples unavailable");
        return;
    }
    let dir = std::env::temp_dir().join(format!("lc-raw-restarts-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("config")).unwrap();
    for phase in ["coreimage", "native", "coreimage-again"] {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--ignored", "--exact", "restart_child", "--nocapture"])
            .env("LIGHTCRAFT_RAW_RESTART_TEST", &dir)
            .env("LIGHTCRAFT_RAW_RESTART_PHASE", phase)
            .env("LIGHTCRAFT_CONFIG_DIR", dir.join("config"));
        if phase == "native" {
            command.env("LIGHTCRAFT_RAW_WORKER", dir.join("no-helper-here"));
        }
        let status = command.status().unwrap();
        assert!(status.success(), "{phase}");
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
#[ignore = "child process driven by processor_choice_survives_real_restarts_in_both_directions"]
fn restart_child() {
    let dir = PathBuf::from(std::env::var_os("LIGHTCRAFT_RAW_RESTART_TEST").unwrap());
    let phase = std::env::var("LIGHTCRAFT_RAW_RESTART_PHASE").unwrap();
    let native = phase == "native";
    assert_eq!(lightcraft_engine::library::DEFAULT_NAME, "LightCraft BW Core Image Library");
    assert_eq!(raw_backend::enabled(), !native);
    assert_eq!(raw_backend::require_sensor().is_ok(), native);
    if native {
        let mut fresh = Session::new().with_fs();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../_orfs/_7190788.ORF");
        fresh.execute("library.import", &json!({"paths":[path.to_string_lossy()]})).unwrap();
        let p = fresh.catalog.photos().next().unwrap();
        assert_eq!(p.develop.detail.sharpen_amount, 15.0);
        assert!(p.raw_backend.is_none());
    }
    let mut s = Session::new().with_fs();
    s.open_library(dir.join("library"), false).unwrap();
    if phase == "coreimage" {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../_orfs");
        let paths: Vec<_> = ["B6010001.ORF", "P6130523.ORF", "_1014216.RW2", "_7190788.ORF", "_C031132.ORF", "_C031132.ORI", "_X220064.ORF"]
            .into_iter()
            .map(|name| root.join(name).to_string_lossy().to_string())
            .collect();
        s.execute("library.import", &json!({"paths":paths,"mode":"add"})).unwrap();
        let ids: Vec<_> = s.catalog.photos().map(|p| p.id.0).collect();
        s.execute("library.select", &json!({"ids":ids,"active":ids[0]})).unwrap();
        s.execute("preset.apply", &json!({"id":"lc.bw-trix400-push2"})).unwrap();
        s.execute("develop.set", &json!({"ids":ids,"control":"light.exposure","value":0.25})).unwrap();
    }
    assert_eq!(s.catalog.photos().count(), 7);
    let before: Vec<_> = s.catalog.photos().map(|p| (p.id, p.develop.clone(), p.history.clone(), p.import_look.clone())).collect();
    s.execute("library.recheckPreviewOnly", &json!({})).unwrap();
    for (id, develop, history, reset) in before {
        let p = s.catalog.photo(id).unwrap();
        assert_eq!(p.raw_backend.is_none(), native, "{}", p.file_name);
        assert_eq!(p.develop, develop);
        assert_eq!(p.history, history);
        assert_eq!(p.import_look, reset);
        assert_eq!(p.develop.detail.sharpen_amount, 15.0);
        let key = lightcraft_engine::media::content_key(p);
        assert!(key.contains(if native { "native-v1" } else { "coreimage-linear" }));
        let rendered = s.render_now(id, 256, 256).unwrap();
        assert!(rendered.image.width > 0 && rendered.image.height > 0);
    }
    let state = s.execute("app.rawProcessing", &json!({})).unwrap();
    assert_eq!(state["restartRequired"], false);
    let next = if native { "coreimage" } else { "native" };
    let pending = s.execute("app.rawProcessing", &json!({"backend":next})).unwrap();
    assert_eq!(pending["restartRequired"], true);
    assert_eq!(pending["active"], state["active"]);
    assert_eq!(raw_backend::enabled(), !native, "saving never switches active processing");
    s.persist().unwrap();
}
