use gpui::{
    actions, px, size, App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem,
    SharedString, TitlebarOptions, WindowBounds, WindowOptions,
};

mod about;
mod app;
mod server;
use about::AboutWindow;
use app::AppState;

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
            |_, cx| cx.new(|_| AppState::new()),
        )
        .unwrap();
    });
}

actions!(set_menus, [Quit, About]);

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

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
