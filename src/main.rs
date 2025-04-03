use google_gmail1::{hyper, hyper_rustls, oauth2, Error as GmailError, Gmail};
use gpui::{
    div, px, rgb, size, App, AppContext, Application, Bounds, Context, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use rocket::{get, routes, serde::json::Json, Config, Error};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;

mod app;
use app::AppState;

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
                            if let Ok((_, msg)) = hub.users().messages_get("me", &id).doit().await {
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
