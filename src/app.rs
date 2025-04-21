use gpui::{
    div, px, rgb, size, Context, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, Styled, Window,
};
use rocket::{routes, Config};
use rocket_dyn_templates::Template;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;

use crate::server::{error, home, index, login, oauth2_callback, profile, summary};

pub struct AppState {
    pub status: String,
    pub server_handle: Arc<Mutex<Option<rocket::Shutdown>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status: "Idle".to_string(),
            server_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&mut self) {
        let server_handle = self.server_handle.clone();
        // Start Rocket in a separate thread
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let config = Config {
                    port: 5000,
                    ..Config::debug_default()
                };
                let rocket = rocket::custom(&config)
                    .mount(
                        "/",
                        routes![index, summary, oauth2_callback, login, error, home, profile],
                    )
                    .attach(Template::fairing())
                    .ignite()
                    .await
                    .unwrap();
                let shutdown = rocket.shutdown();
                *server_handle.lock().unwrap() = Some(shutdown.clone());
                let _ = rocket.launch().await;
            });
        });
        self.status = "Serving at http://127.0.0.1:5000".to_string();
    }

    pub fn stop(&mut self) {
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
            .bg(rgb(0xffffff))
            .flex()
            .flex_col()
            .gap_3()
            .w_full()
            .h_full()
            .justify_center()
            .px_2()
            .children([
                div().justify_center().text_xl().child("Gmail Cleaner"),
                div().flex().gap_2().children([div()
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
                            cx.open_url("http://127.0.0.1:5000/");
                        }),
                    )
                    .child("start")]),
                div()
                    .px_2()
                    .text_color(rgb(0xff0000))
                    .text_sm()
                    .child(format!("{}", self.status)),
            ])
    }
}
