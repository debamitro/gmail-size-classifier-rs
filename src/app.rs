use gpui::{
    div, rgb, Context, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    Styled, Window,
};
use rocket::{routes, Config};
use rocket_dyn_templates::Template;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio;

use crate::server::{error, home, index, login, oauth2_callback, profile, summary};

#[cfg(target_os = "macos")]
fn get_app_data_dir() -> Result<String, ()> {
    let exe_path = std::env::current_exe().unwrap();
    if let Some(parent_path) = exe_path.parent().unwrap().parent() {
        return Ok(parent_path.join("Resources").to_str().unwrap().to_string());
    }

    Err(())
}

#[cfg(not(target_os = "macos"))]
fn get_app_data_dir() -> Result<String, ()> {
    Ok(std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string())
}

fn get_templates_dir() -> Result<String, ()> {
    if let Ok(app_data_dir) = get_app_data_dir() {
        let expected_path = Path::new(&app_data_dir).join("templates");
        if let Ok(true) = expected_path.try_exists() {
            return Ok(expected_path.to_str().unwrap().to_string());
        }
    }

    let default_path = Path::new("./templates");
    if let Ok(true) = default_path.try_exists() {
        return Ok(default_path.to_str().unwrap().to_string());
    }

    Err(())
}

pub struct AppState {
    pub status: Arc<Mutex<String>>,
    pub server_handle: Arc<Mutex<Option<rocket::Shutdown>>>,
    pub templates_dir: Arc<String>,
}

impl AppState {
    pub fn new() -> Self {
        if let Ok(templates_dir) = get_templates_dir() {
            Self {
                status: Arc::new(Mutex::new("Idle".to_string())),
                server_handle: Arc::new(Mutex::new(None)),
                templates_dir: Arc::new(templates_dir),
            }
        } else {
            Self {
                status: Arc::new(Mutex::new("Error!".to_string())),
                server_handle: Arc::new(Mutex::new(None)),
                templates_dir: Arc::new("".to_string()),
            }
        }
    }

    pub fn start(&mut self) {
        let server_handle = self.server_handle.clone();
        let templates_dir = self.templates_dir.clone();
        let status = self.status.clone();
        // Start Rocket in a separate thread
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let figment = Config::figment()
                    .merge(("port", 5000))
                    .merge(("template_dir", templates_dir.as_str()));
                let rocket = rocket::custom(figment)
                    .mount(
                        "/",
                        routes![index, summary, oauth2_callback, login, error, home, profile],
                    )
                    .attach(Template::fairing())
                    .ignite();
                match rocket.await {
                    Ok(ignited_rocket) => {
                        let shutdown = ignited_rocket.shutdown();
                        *server_handle.lock().unwrap() = Some(shutdown);
                        let _ = ignited_rocket.launch().await;
                        *status.lock().unwrap() =
                            "Open http://127.0.0.1:5000 in your web browser".to_string();
                    }
                    Err(e) => {
                        println!("Rocket error: {}", e);
                    }
                }
            });
        });
    }

    pub fn stop(&mut self) {
        if let Some(shutdown) = self.server_handle.lock().unwrap().take() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                shutdown.notify();
            });
        }
        //self.status = "press start to begin".to_string();
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
            .px_2()
            .children([
                div().text_center().text_3xl().child("Gmail Cleaner"),
                div().justify_center().children([div()
                    .bg(rgb(0x4CAF50))
                    .px_4()
                    .py_2()
                    .w_48()
                    .rounded_md()
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _win, cx| {
                            this.start();
                            thread::sleep(time::Duration::from_secs(5));
                            *this.status.lock().unwrap() =
                                "Open http://127.0.0.1:5000 in your web browser".to_string();
                            cx.notify();
                            cx.open_url("http://127.0.0.1:5000/");
                        }),
                    )
                    .text_center()
                    .child("start")]),
                div()
                    .border_t_1()
                    .border_color(Hsla::black())
                    .bg(rgb(0xe0e0e0))
                    .px_2()
                    .text_color(rgb(0x000000))
                    .text_sm()
                    .child(format!("{}", self.status.lock().unwrap().as_str())),
            ])
    }
}
