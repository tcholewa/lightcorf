// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Community and project links (Help menu, About dialog, top-bar Discord button).

/// Public fork repository; legacy command ids stay compatible with saved keymaps.
pub const GITHUB: &str = "https://github.com/tcholewa/lightcorf";
pub const APP_PAGE: &str = GITHUB;
/// Retained upstream credit, explicitly labelled in the UI.
pub const WEBSITE: &str = "https://github.com/storytold/lightcraft";
/// Legacy Discord command now opens the fork's support page.
pub const DISCORD: &str = "https://github.com/tcholewa/lightcorf/issues";
pub const HELP: &str = "https://github.com/tcholewa/lightcorf/tree/lightcorf/docs";
pub const FEEDBACK: &str = "https://github.com/tcholewa/lightcorf/issues/new";

/// (Stable UI command id, menu label, URL), in Help-menu order.
pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "Lightcorf Help", HELP),
    ("app.discord", "Lightcorf Support…", DISCORD),
    ("app.website", "Lightcorf Project", APP_PAGE),
    ("app.github", "Lightcorf on GitHub", GITHUB),
    ("app.artcraft", "Upstream LightCraft", WEBSITE),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

/// The URL behind a link command id.
pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_https_and_name_this_app() {
        for (id, label, url) in LINKS {
            assert!(url.starts_with("https://"), "{id}");
            assert!(!label.is_empty());
            assert_eq!(url_of(id), Some(*url));
        }
        assert_eq!(url_of("app.github"), Some("https://github.com/tcholewa/lightcorf"));
        assert_eq!(url_of("app.help"), Some("https://github.com/tcholewa/lightcorf/tree/lightcorf/docs"));
        assert_eq!(url_of("app.discord"), Some("https://github.com/tcholewa/lightcorf/issues"));
        assert_eq!(url_of("app.feedback"), Some("https://github.com/tcholewa/lightcorf/issues/new"));
        assert_eq!(url_of("app.artcraft"), Some("https://github.com/storytold/lightcraft"));
        assert_eq!(url_of("unknown"), None);
    }
}
