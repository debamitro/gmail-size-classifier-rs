use gpui::{
    actions, px, size, App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem,
    WindowBounds, WindowOptions,
};

mod app;
mod server;
use app::AppState;

fn main() {
    // Start GPUI application
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.bind_keys(vec![KeyBinding::new("cmd-q", Quit, None)]);
        cx.on_action(quit);
        cx.set_menus(vec![Menu {
            name: "set_menus".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        let bounds = Bounds::centered(None, size(px(300.0), px(200.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AppState::new()),
        )
        .unwrap();
    });
}

actions!(set_menus, [Quit]);

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}
