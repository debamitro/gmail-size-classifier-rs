use gpui::{
    div, px, rgb, size, App, AppContext, Application, Bounds, Context, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions,
};
use rocket::{get, routes, Error, serde::json::Json};
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;

#[derive(Serialize)]
struct SearchResult {
    title: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/api/summary?<max>")]
fn summary(max: String) -> Json<Vec<SearchResult>> {
    // This is a mock implementation - replace with actual API call
    let results = vec![
        SearchResult { title: format!("Result 1 for: ") },
        SearchResult { title: format!("Result 2 for: ") },
        SearchResult { title: format!("Result 3 for: ") },
    ];
    Json(results)
}

struct AppState {
    status: String,
    server_handle: Arc<Mutex<Option<rocket::Shutdown>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            status: "press start to begin".to_string(),
            server_handle: Arc::new(Mutex::new(None)),
        }
    }

    fn start(&mut self) {
        let server_handle = self.server_handle.clone();
        // Start Rocket in a separate thread
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let rocket = rocket::build()
                    .mount("/", routes![index, summary])
                    .attach(Template::fairing())
                    .ignite()
                    .await
                    .unwrap();
                let shutdown = rocket.shutdown();
                *server_handle.lock().unwrap() = Some(shutdown.clone());
                let _ = rocket.launch().await;
            });
        });
        self.status = "started at http://127.0.0.1:8000".to_string();
    }

    fn stop(&mut self) {
        if let Some(shutdown) = self.server_handle.lock().unwrap().take() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                shutdown.notify();
            });
        }
        self.status = "press start to begin".to_string();
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
                    .child(format!("{}", self.status)),
                div().flex().gap_2().children([
                    div()
                        .bg(rgb(0x4CAF50))
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _win, cx| {
                                this.start();
                                cx.notify();
                            }),
                        )
                        .child("start"),
                    div()
                        .bg(rgb(0x4CAF50))
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _win, cx| {
                                this.stop();
                                cx.notify();
                            }),
                        )
                        .child("stop"),
                ]),
            ])
    }
}

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
        let _ = cx.on_window_closed(|cx: &mut App| {
            cx.quit();
        });
    });
}
