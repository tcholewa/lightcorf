// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Production macOS backend integration. Existing engine unit fixtures retain the Rust reference path.
#![cfg(target_os = "macos")]
use lightcraft_engine::{files, raw_backend};
use std::path::PathBuf;

#[test]
fn supplied_cameras_use_apple_rgb_and_preserve_hdr_and_orientation() {
    if isolated_coreimage("supplied_cameras_use_apple_rgb_and_preserve_hdr_and_orientation") {
        return;
    }
    assert!(raw_backend::enabled(), "integration must link the production backend policy");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../_orfs");
    if !root.is_dir() {
        lightcraft_testkit::skip!(Corpus, "personal _orfs acceptance samples unavailable");
        return;
    }
    for (file, size) in [
        ("B6010001.ORF", (4608, 3456)),
        ("P6130523.ORF", (3456, 4608)),
        ("_1014216.RW2", (5184, 3888)),
        ("_7190788.ORF", (3888, 5184)),
        ("_C031132.ORF", (8160, 6120)),
        ("_C031132.ORI", (5184, 3888)),
        ("_X220064.ORF", (3888, 5184)),
    ] {
        let bytes = std::fs::read(root.join(file)).unwrap();
        let probe = files::probe_bytes(file, &bytes).unwrap();
        assert_eq!((probe.width, probe.height), size, "{file}");
        assert!(probe.preview_only.is_none(), "{file}");
        assert!(probe.raw_backend.as_deref().unwrap().starts_with("coreimage-linear-rec2020-v1:"));
        let (full, source) = files::load_bytes(&bytes, usize::MAX).unwrap();
        assert_eq!((full.width, full.height), (size.0 as usize, size.1 as usize));
        assert!(source.raw && source.camera_color.is_none() && source.camera_tone.is_none());
        assert!(!source.relative_wb && !source.legacy_relative_wb);
        assert_eq!(source.raw_backend, probe.raw_backend);
        assert!(full.data.iter().flatten().all(|v| v.is_finite()));
        if file == "_7190788.ORF" {
            assert!(full.data.iter().flatten().any(|&v| v > 1.0));
        }
        let expected = lightcraft_raster::resample::fit(&full, 512, 512, lightcraft_raster::resample::Filter::Box);
        drop(full);
        // Upstream thumbnails may use DNG mipmaps. Apple's full RAW decode must keep
        // its provenance and must not be marked as a reduced embedded source.
        let (preview, preview_source, reduced) = files::load_vec_reduced(bytes, 512).unwrap();
        assert!(!reduced, "{file}: Apple decodes the full RAW before resizing");
        assert_eq!(preview_source.raw_backend, source.raw_backend);
        assert_eq!((preview.width, preview.height), (expected.width, expected.height));
        let error = preview.data.iter().flatten().zip(expected.data.iter().flatten()).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
        assert!(error < 0.0001, "{file}: preview differs from full-resolution downsample by {error}");
    }
}

#[test]
fn sensor_tools_and_bad_input_cannot_fall_back_to_rust() {
    if isolated_coreimage("sensor_tools_and_bad_input_cannot_fall_back_to_rust") {
        return;
    }
    assert!(raw_backend::enabled());
    assert!(raw_backend::require_sensor().is_err());
    assert_eq!(lightcraft_engine::library::DEFAULT_NAME, "LightCraft BW Core Image Library");
    let mut session = lightcraft_engine::Session::new();
    let info = session.commands().into_iter().find(|c| c.id == "photo.convertToDng");
    // The same policy is enforced below the command layer for CLI/import paths.
    assert!(info.is_some());
    assert!(session.execute("photo.convertToDng", &serde_json::json!({})).is_err());
    assert!(raw_backend::probe(b"not a RAW").is_err());
}

#[test]
fn original_em10_qualification_is_explicitly_pending_without_its_fixture() {
    if isolated_coreimage("original_em10_qualification_is_explicitly_pending_without_its_fixture") {
        return;
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../corpus/raw/orf-olympus-em10.orf");
    if !path.is_file() {
        lightcraft_testkit::skip!(Corpus, "original E-M10 Core Image qualification pending: corpus/raw/orf-olympus-em10.orf is missing");
        return;
    }
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(lightcraft_meta::extract(&bytes).model.as_deref().map(str::trim), Some("E-M10"));
    let probe = files::probe_bytes("original-em10.orf", &bytes).unwrap();
    assert!(probe.preview_only.is_none());
    let (image, source) = files::load_bytes(&bytes, usize::MAX).unwrap();
    assert!(source.raw && source.raw_backend.is_some());
    assert_eq!(image.width * image.height, 4608 * 3456);
}

// Production selection is persisted per user. These tests must never read that profile.
fn isolated_coreimage(name: &str) -> bool {
    if std::env::var("LIGHTCRAFT_COREIMAGE_TEST_CHILD").as_deref() == Ok(name) {
        return false;
    }
    let dir = std::env::temp_dir().join(format!("lc-apple-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("raw-processing.json"), br#"{"backend":"coreimage"}"#).unwrap();
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture"])
        .env("LIGHTCRAFT_COREIMAGE_TEST_CHILD", name)
        .env("LIGHTCRAFT_CONFIG_DIR", &dir)
        .status()
        .unwrap();
    std::fs::remove_dir_all(dir).unwrap();
    assert!(result.success(), "isolated Core Image test failed");
    true
}
