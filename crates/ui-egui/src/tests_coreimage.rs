//! macOS has RGB from Core Image; sensor tools must not offer model setup or mutate edits.
use crate::{LightcraftApp, Services};
use lightcraft_catalog::{MediaKind, Op, Photo, PhotoId, Source};
use serde_json::json;

#[test]
fn coreimage_sensor_tools_fail_without_opening_setup_or_changing_settings() {
    if !lightcraft_engine::raw_backend::enabled() {
        assert!(lightcraft_engine::raw_backend::require_sensor().is_ok());
        return;
    }
    let mut session = lightcraft_engine::Session::new();
    let id = PhotoId(1);
    let mut photo = Photo::new(id, Source::File { path: "not-read.ORF".into() }, "test.ORF", "ORF", 5184, 3888, "2026-10-21");
    photo.kind = MediaKind::Raw;
    photo.raw_backend = Some("coreimage-linear-rec2020-v1:8:test".into());
    session.commit("fixture", Op::AddPhoto { photo: Box::new(photo) }).unwrap();
    session.execute("library.select", &json!({"ids":[id.0],"active":id.0})).unwrap();
    let before = session.develop_of(id).unwrap().clone();
    let mut app = LightcraftApp::new(session, Services::default());
    for command in ["denoise.toggle", "denoise.queue", "photo.convertToDng"] {
        let error = app.run(command, json!({"id":id.0,"ids":[id.0],"enabled":true})).unwrap_err();
        assert!(error.contains("Core Image"), "{command}: {error}");
        assert_eq!(app.session.develop_of(id).unwrap(), before);
        assert!(!crate::model_setup::denoise_requested(&app, id));
        assert!(app.ui.dialog.is_none());
    }
    assert!(matches!(app.session.denoise_photo_state(id), lightcraft_engine::denoise::PhotoState::Failed { unsupported: true, .. }));
}
