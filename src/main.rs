#[cfg(feature = "gpui_ui")]
use gpui::{
    actions, px, size, App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem,
    SharedString, TitlebarOptions, WindowBounds, WindowOptions,
};

#[cfg(feature = "iced_ui")]
use iced::{Application, Command, Element, Settings, Theme};

#[cfg(feature = "gpui_ui")]
mod about;

#[cfg(feature = "gpui_ui")]
use about::AboutWindow;

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
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(600.0), px(400.0)),
                cx,
            ))),
            ..Default::default()
        },
        |_, cx| cx.new(|_| AboutWindow {}),
    )
    .unwrap();
}
