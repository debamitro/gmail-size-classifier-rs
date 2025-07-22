#[cfg(feature = "gpui_ui")]
use gpui::{
    actions, px, size, App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem,
    SharedString, TitlebarOptions, WindowBounds, WindowOptions,
};

#[cfg(feature = "iced_ui")]
use iced::{Application, Command, Element, Settings, Theme};

use native_dialog::{MessageDialog, MessageType};

#[cfg(feature = "gpui_ui")]
mod app_gpui_ui;

mod app;
mod server;

#[cfg(feature = "iced_ui")]
mod app_iced_ui;

#[cfg(feature = "iced_ui")]
fn main() -> iced::Result {
    app_iced_ui::GmailCleanerApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[cfg(feature = "gpui_ui")]
fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.bind_keys(vec![KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(quit);
        cx.on_action(about);
        cx.set_menus(vec![Menu {
            name: "set_menus".into(),
            items: vec![
                MenuItem::action("About", About),
                MenuItem::action("Quit", Quit),
            ],
        }]);
        let bounds = Bounds::centered(None, size(px(400.0), px(160.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::new_static("Gmail Cleaner")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| app_gpui_ui::GmailCleanerApp::new()),
        )
        .unwrap();
    });
}

#[cfg(feature = "gpui_ui")]
actions!(set_menus, [Quit, About]);

#[cfg(feature = "gpui_ui")]
fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

#[cfg(feature = "gpui_ui")]
fn about(_: &About, cx: &mut App) {
    // Show about dialog in a separate system window
    let about_text = "Gmail Cleaner is a desktop app which helps you find out which emails are taking up storage space in your Gmail account. This app runs on your desktop and does not send your email to any server. Therefore it is the most secure way of cleaning up your Gmail account. You don't need to give any permissions to this app, neither do you need to provide credentials. When you start it, a browser window opens up with Gmail's login page. Once Gmail authorizes you the app visually shows you what you can delete. The app does not read your email or cannot modify your Gmail account in any way.";

    let _ = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("About Gmail Cleaner")
        .set_text(&format!("Gmail Cleaner v0.1.0-beta\n\n{}", about_text))
        .show_alert();
}
