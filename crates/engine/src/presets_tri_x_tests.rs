//! The film look owns creative settings, never photographic corrections.
use super::*;
use lightcraft_develop::{DevelopSettings, Treatment};

fn preset() -> Preset {
    builtin().into_iter().find(|p| p.id == "lc.bw-trix400-push2").unwrap()
}

#[test]
fn tri_x_replaces_creative_edits_and_preserves_corrections() {
    let base = DevelopSettings::default()
        .merged(&json!({
            "process": 1,
            "profile": {"id": "lc.bw.sepia", "amount": 130.0, "camera_local_tone": true},
            "light": {"exposure": 1.25, "contrast": -20.0, "highlights": 50.0},
            "wb": {"mode": "custom", "temp": 5200.0, "tint": 12.0},
            "hdr": {"enabled": true},
            "curve": {"red": [{"x": 0.0, "y": 0.1}, {"x": 1.0, "y": 0.9}], "split_mid": 65.0},
            "color": {"saturation": 35.0, "vibrance": -20.0},
            "mixer": {"blue": {"hue": 50.0, "sat": -40.0, "lum": 60.0}},
            "point_colors": [{"hue": 30.0, "lum_shift": 25.0}],
            "grading": {"global": {"hue": 35.0, "sat": 40.0, "lum": 15.0}},
            "bw_mix": {"magenta": 70.0},
            "effects": {"dehaze": 45.0},
            "grain": {"amount": 90.0, "seed": 987654},
            "detail": {"sharpen_amount": 65.0, "nr_luminance": 25.0},
            "optics": {"distortion": 15.0},
            "calibration": {"red_hue": 25.0},
            "vignette": {"amount": -30.0},
            "geometry": {"rotate": 2.0},
            "masks": [{"id": 7, "name": "Retain local edit", "adjust": {"exposure": 0.8}}],
            "disabled_sections": ["effects", "color"]
        }))
        .unwrap();
    let out = preset().apply(&base, 1.0);
    assert_eq!(out.treatment, Treatment::Bw);
    assert_eq!(out.profile.id, "lc.color");
    assert_eq!(out.profile.amount, 100.0);
    assert_eq!((out.light.contrast, out.light.highlights, out.light.shadows, out.light.whites, out.light.blacks), (45.0, -20.0, -25.0, 10.0, -50.0));
    assert_eq!((out.grain.amount, out.grain.size, out.grain.roughness), (50.0, 50.0, 70.0));
    assert!(out.grain.advanced);
    assert_eq!(out.bw_mix.bands(), [10.0, 8.0, 0.0, -5.0, -10.0, -15.0, -5.0, 0.0]);
    assert!(out.mixer.is_neutral() && out.point_colors.is_empty() && out.grading.is_neutral());
    assert_eq!(out.color, Default::default());
    assert!(out.curve.master.is_empty() && out.curve.red.is_empty() && out.curve.green.is_empty() && out.curve.blue.is_empty());
    assert_eq!(out.curve.split_mid, 50.0);
    assert!(!out.section_enabled("effects"));
    assert_eq!(out.profile.camera_local_tone, base.profile.camera_local_tone);
    assert_eq!(out.light.exposure, base.light.exposure);
    assert_eq!(out.grain.seed, base.grain.seed);
    let mut untouched = out.to_json();
    let mut before = base.to_json();
    for key in ["profile", "treatment", "light", "curve", "color", "mixer", "point_colors", "grading", "bw_mix", "effects", "grain"] {
        untouched.as_object_mut().unwrap().remove(key);
        before.as_object_mut().unwrap().remove(key);
    }
    assert_eq!(untouched, before, "all settings outside the creative look are preserved");
    assert_eq!(preset().apply(&out, 1.0), out, "repeat application is idempotent");
    assert_eq!(DevelopSettings::from_json(&out.to_json()).unwrap(), out);
}

#[test]
fn tri_x_amount_and_batch_undo() {
    let p = preset();
    let base = DevelopSettings::default();
    assert_eq!(p.apply(&base, 0.0), base);
    for (amount, bw) in [(0.49, false), (0.5, true), (1.0, true), (2.0, true)] {
        let s = p.apply(&base, amount);
        assert_eq!(s.treatment == Treatment::Bw, bw);
        assert_eq!(s.grain.advanced, bw);
        assert_eq!(s.grain.amount, 50.0 * amount);
        assert_eq!(s.grain.roughness, (50.0 + 20.0 * amount).min(100.0));
        assert_eq!(s.light.exposure, 0.0, "push naming must not add exposure");
    }
    let mut s = Session::with_demo();
    let ids: Vec<_> = s.visible_cloned().into_iter().take(2).collect();
    s.execute("library.select", &json!({"ids": ids})).unwrap();
    s.execute("develop.set", &json!({"control": "light.exposure", "value": 0.75})).unwrap();
    let before: Vec<_> = ids.iter().map(|id| s.develop_of(*id).unwrap()).collect();
    let steps = s.undo.len();
    s.execute("preset.apply", &json!({"id": p.id})).unwrap();
    assert_eq!(s.undo.len(), steps + 1);
    for (id, old) in ids.iter().zip(&before) {
        assert_eq!(*s.develop_of(*id).unwrap(), p.apply(old, 1.0));
    }
    s.execute("edit.undo", &json!({})).unwrap();
    for (id, old) in ids.iter().zip(&before) {
        assert_eq!(&s.develop_of(*id).unwrap(), old);
    }
    s.execute("edit.redo", &json!({})).unwrap();
    for id in ids {
        assert!(s.develop_of(id).unwrap().grain.advanced);
    }
}

#[test]
fn tri_x_neutral_output_and_gpu_windows() {
    use lightcraft_pipeline::{PixelWindow, RenderRequest, SourceInfo, render};
    let src = std::sync::Arc::new(lightcraft_raster::Rgb32f::from_fn(320, 240, |x, y| {
        let v = x as f32 / 319.0;
        if y < 120 { [v; 3] } else { [v, v * 0.6, v * 0.3] }
    }));
    let info = SourceInfo::default();
    let s = preset().apply(&DevelopSettings::default(), 1.0);
    let req = RenderRequest::fit(320, 240);
    let cpu = render(&src, &info, &s, &req);
    for pixel in cpu.image.as_bytes().chunks_exact(4) {
        assert!(pixel[0].abs_diff(pixel[1]) <= 1 && pixel[1].abs_diff(pixel[2]) <= 1, "neutral B&W");
    }
    let mut clean = s.clone();
    clean.grain.amount = 0.0;
    assert_ne!(cpu.image.as_bytes(), render(&src, &info, &clean, &req).image.as_bytes(), "grain is visible");
    if !crate::gpu::available() {
        eprintln!("Tri-X GPU check skipped: {:?}", crate::gpu::unavailable_reason());
        return;
    }
    for req in [req, RenderRequest { window: Some(PixelWindow { x: 53, y: 41, w: 107, h: 83 }), ..req }] {
        let cpu = render(&src, &info, &s, &req);
        let gpu = crate::gpu::render(&src, &info, &s, &req, None).expect("GPU render must not fall back");
        let diffs: Vec<_> = cpu.image.as_bytes().iter().zip(gpu.image.as_bytes()).map(|(a, b)| a.abs_diff(b)).collect();
        let max = diffs.iter().copied().max().unwrap_or(0);
        let mean = diffs.iter().map(|v| f64::from(*v)).sum::<f64>() / diffs.len() as f64;
        eprintln!("Tri-X CPU/GPU: mean {mean}, max {max} LSB");
        assert!(max <= 3 && mean < 0.5);
    }
}
