use iced::{Application, Command, Element, Settings, Theme};

mod app;
mod server;
use app::AppState;
mod app_iced_ui;
use app_iced_ui::GmailCleanerApp;

#[cfg(feature = "iced_ui")]
fn main() -> iced::Result {
    GmailCleanerApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
