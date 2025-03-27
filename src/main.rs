use gpui::{div, Window, Context, Render, IntoElement, px, 
    rgb, Application, App, Bounds, size, WindowOptions, WindowBounds, 
    AppContext, Styled, ParentElement, InteractiveElement, MouseButton};
use rocket::{get, routes, Error};
use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket!"
}

#[rocket::main]
async fn rocket() -> Result<(), rocket::Error> {
    rocket::build().mount("/", routes![index])
    .launch().await?;
    Ok(())
}

struct AppState {
    counter: i32,
}

impl AppState {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .size(px(300.0))
            .justify_center()
            .items_center()
            .text_xl()
            .children([
                div()
                    .text_color(rgb(0xff0000))
                    .child(format!("Counter: {}", self.counter)),
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .bg(rgb(0x4CAF50))
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left,cx.listener(|this, _event, _win, cx| {
                                this.increment();
                                cx.notify();
                            }))
                            .child("Increment"),
                    ),
            ])
    }
}

fn main() {
    // Start Rocket in a separate thread
    thread::spawn(|| {
        rocket();
    });

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
