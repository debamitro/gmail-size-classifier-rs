use gpui::{
    div, px, rgb, size, App, AppContext, Application, Bounds, Context, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions,
};

mod app;
mod server;
use app::AppState;

fn main() {
    // Start GPUI application
    Application::new().run(|cx: &mut App| {
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
