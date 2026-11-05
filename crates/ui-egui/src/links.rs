//! Community and project links.
use serde_json::{Value, json};

use crate::PhotocraftApp;

pub const DISCORD: &str = "";
pub const ARTCRAFT_WEBSITE: &str = "";
pub const APP_PAGE: &str = "";
pub const GITHUB: &str = "";
pub const ISSUES: &str = "";

/// Help-menu link commands: (id, url). Labels live in `menus::UI_COMMANDS`.
pub const COMMANDS: &[(&str, &str)] = &[];

pub fn url_for(id: &str) -> Option<&'static str> {
    COMMANDS.iter().find(|c| c.0 == id).map(|c| c.1)
}

/// Open `url` in the system browser (a new tab on the web) and note it in the status bar. Prefers
/// the platform `open_url` service (reliable on Windows/macOS/Linux); falls back to `ctx.open_url`.
pub fn open(app: &mut PhotocraftApp, ctx: &egui::Context, url: &str) -> Value {
    let opened = app.services.open_url.as_ref().map(|f| f(url).is_ok()).unwrap_or(false);
    if !opened {
        ctx.open_url(egui::OpenUrl::new_tab(url));
    }
    let template = "Opened {url}";
    app.ui.status = crate::i18n::fmt(tl!(template), &[("url", url)]);
    json!({"url": url})
}

/// Stubbed out for TSPhoto.
pub fn discord_button(_app: &mut PhotocraftApp, _ui: &mut egui::Ui, _min_width: f32) -> egui::Response {
    _ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
}

/// Stubbed out for TSPhoto.
pub fn link_row(_app: &mut PhotocraftApp, _ui: &mut egui::Ui) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_menu_lists_report_issue_then_separator_then_system_info_and_about() {
        let app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        let help: Vec<String> = crate::menus::menu_items(&app).into_iter().filter(|i| i.path == ["Help"]).map(|i| i.id).collect();
        assert_eq!(
            help,
            [
                "help.reportIssue",
                "help.checkForUpdates",
                "---",
                "help.systemInfo",
                "help.about"
            ]
        );
    }
}
