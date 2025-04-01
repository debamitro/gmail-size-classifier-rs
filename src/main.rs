use gpui::{
    div, px, rgb, size, App, AppContext, Application, Bounds, Context, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions,
};
use rocket::{get, routes, Error, serde::json::Json, Config};
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;
use google_gmail1::{Gmail, oauth2, hyper, hyper_rustls, 
    Error as GmailError};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

#[derive(Serialize)]
struct SearchResult {
    title: String,
    size: i32,
}

async fn setup_gmail_client() -> Result<Gmail<HttpsConnector<HttpConnector>>, GmailError> {
    let secret = oauth2::read_application_secret("credentials.json")
        .await
        .expect("credentials.json not found");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("gmail_token.json")
    .build()
    .await?;

    let hub = Gmail::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    Ok(hub)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/api/summary?<max>")]
async fn summary(max: String) -> Json<Vec<SearchResult>> {
    let max_results: u32 = max.parse().unwrap_or(10);
    
    // This would need to be passed from the AppState - for now using a new client
    if let Ok(hub) = setup_gmail_client().await {
        let result = hub
            .users()
            .messages_list("me")
            .max_results(max_results)
            .doit()
            .await;

        match result {
            Ok((_, message_list)) => {
                let mut results = Vec::new();
                if let Some(messages) = message_list.messages {
                    for message in messages {
                        if let Some(id) = message.id {
                            if let Ok((_, msg)) = hub.users().messages_get("me",&id).doit().await {
                                let size = msg.size_estimate.unwrap_or(0);
                                results.push(SearchResult {
                                    title: msg.snippet.unwrap_or_else(|| "No subject".to_string()),
                                    size,
                                });
                            }
                        }
                    }
                }
                Json(results)
            }
            Err(e) => {
                eprintln!("Error fetching messages: {}", e);
                Json(vec![])
            }
        }
    } else {
        Json(vec![])
    }
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
                let config = Config {
                    port: 5000,
                    ..Config::debug_default()};
                let rocket = rocket::custom(&config)
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
                    .px_2()
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
                    div()
                        .bg(rgb(0x4CAF50))
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|_, _event, _win, cx| {
                                cx.quit();
                            }),
                        )
                        .child("quit"),
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
    });
}
