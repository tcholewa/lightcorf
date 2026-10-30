// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! This source-only fork has no binary update feed. Never install an upstream LightCraft release
//! over Lightcorf: it does not contain the fork's RAW processors and saved-look behavior.

use serde_json::Value;

use crate::LightcraftApp;

#[derive(Default)]
pub(crate) struct Runtime {
    pub(crate) manual_window_open: bool,
    pub(crate) status_message: Option<String>,
}

pub(crate) fn blocks_input(_app: &LightcraftApp) -> bool {
    false
}

pub(crate) fn check(app: &mut LightcraftApp) -> Result<Value, String> {
    app.updates_rt.manual_window_open = true;
    app.updates_rt.status_message = Some(format!(
        "Lightcorf {} is built from source. Build updates from the Lightcorf project; official LightCraft releases do not include this fork's changes.",
        env!("CARGO_PKG_VERSION")
    ));
    Ok(Value::Null)
}

pub(crate) fn tick(_app: &mut LightcraftApp, _ctx: &egui::Context) {}

pub(crate) fn show(app: &mut LightcraftApp, ctx: &egui::Context) {
    if app.updates_rt.manual_window_open {
        let mut open = true;
        egui::Window::new("Lightcorf Updates")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.hyperlink_to("Lightcorf source and build instructions", crate::links::GITHUB);
                ui.add_space(8.0);
                if ui.button(crate::i18n::tr("OK")).clicked() {
                    app.updates_rt.manual_window_open = false;
                }
            });
        if !open {
            app.updates_rt.manual_window_open = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{check, show};
    use crate::LightcraftApp;
    use lightcraft_engine::Session;

    #[test]
    fn update_check_explains_source_builds_without_offering_an_upstream_installer() {
        let mut app = LightcraftApp::new(Session::with_demo(), Default::default());
        let revision = app.session.catalog.revision;
        assert!(!app.updates_rt.manual_window_open);
        check(&mut app).unwrap();
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx()));
        output.textures_delta.clear();
        assert!(app.updates_rt.manual_window_open);
        assert!(app.updates_rt.status_message.as_deref().unwrap().contains("built from source"));
        assert_eq!(crate::links::GITHUB, "https://github.com/tcholewa/lightcorf");
        assert_eq!(app.session.catalog.revision, revision);
    }
}
