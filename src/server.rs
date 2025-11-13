use handlebars::Handlebars;
use reqwest;
use rocket::http::{Cookie, CookieJar, SameSite, ContentType};
use rocket::response::content::{RawHtml, RawJavaScript};
use rocket::serde::json::serde_json;
use rocket::{get, response::Redirect, serde::json::Json, State};
use rocket::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::json;
use urlencoding;
use crate::gmail_client::*;

#[derive(Serialize)]
pub struct SearchResult {
    title: String,
    size: i32,
    thread_id: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
}

#[derive(Deserialize)]
struct CredentialsWeb {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
    javascript_origins: Vec<String>,
}

#[derive(Deserialize)]
struct Credentials {
    web: CredentialsWeb,
}

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Redirect {
    match cookies.get_private("token") {
        Some(_cookie) => Redirect::to("/home"),
        None => Redirect::to("/login"),
    }
}

#[get("/home")]
pub fn home(cookies: &CookieJar<'_>, hbs: &State<Handlebars<'static>>) -> RawHtml<String> {
    match cookies.get_private("token") {
        Some(_cookie) => {
            let html = hbs.render("index", &json!({})).unwrap_or_else(|e| {
                format!("Template error: {}", e)
            });
            RawHtml(html)
        }
        None => {
            let html = hbs
                .render(
                    "error",
                    &json!({
                        "error": "Not logged in",
                        "redirect": "/login"
                    }),
                )
                .unwrap_or_else(|e| format!("Template error: {}", e));
            RawHtml(html)
        }
    }
}

#[get("/js/main.js")]
pub fn mainjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/main.js");
    RawJavaScript(js.to_string())
}

#[get("/js/app.js")]
pub fn appjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/app.js");
    RawJavaScript(js.to_string())
}

#[get("/js/chart_section.js")]
pub fn chart_sectionjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/chart_section.js");
    RawJavaScript(js.to_string())
}

#[get("/js/header.js")]
pub fn headerjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/header.js");
    RawJavaScript(js.to_string())
}

#[get("/js/search.js")]
pub fn searchjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/search.js");
    RawJavaScript(js.to_string())
}

#[get("/js/tab_section.js")]
pub fn tab_sectionjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/tab_section.js");
    RawJavaScript(js.to_string())
}

#[get("/js/utils.js")]
pub fn utilsjs() -> RawJavaScript<String> {
    let js = include_str!("../static/js/utils.js");
    RawJavaScript(js.to_string())
}

#[get("/logo_062725.png")]
pub fn logo() -> (ContentType, &'static [u8]) {
    let img = include_bytes!("../static/logo_062725.png");
    (ContentType::PNG, img.as_slice())
}

#[get("/login")]
pub fn login(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("token");
    cookies.remove_private("page_token");
    let credentials_file = include_str!("../credentials.json");
    match serde_json::from_str::<Credentials>(credentials_file) {
        Ok(credentials) => {
            let scope = urlencoding::encode("https://www.googleapis.com/auth/gmail.readonly");
            let redirect_uri = urlencoding::encode(&credentials.web.redirect_uris[0]);
            let client_id = credentials.web.client_id;

            let auth_url = format!(
                "https://accounts.google.com/o/oauth2/v2/auth?scope={}&redirect_uri={}&response_type=code&client_id={}", 
                scope,
                redirect_uri,
                client_id
            );

            Redirect::to(auth_url)
        }
        Err(_) => Redirect::to("/error"),
    }
}

#[get("/oauth2callback?<code>&<_state>&<scope>&<_authuser>&<_prompt>")]
pub async fn oauth2_callback(
    code: Option<String>,
    _state: Option<String>,
    scope: Option<String>,
    _authuser: Option<String>,
    _prompt: Option<String>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    match code {
        Some(code) => {
            let credentials_file = include_str!("../credentials.json");
            match serde_json::from_str::<Credentials>(credentials_file) {
                Ok(credentials) => {
                    let client = reqwest::Client::new();
                    let params = [
                        ("client_id", credentials.web.client_id.as_str()),
                        ("client_secret", credentials.web.client_secret.as_str()),
                        ("code", &code),
                        ("grant_type", "authorization_code"),
                        ("redirect_uri", &credentials.web.redirect_uris[0]),
                    ];

                    match client
                        .post(&credentials.web.token_uri)
                        .form(&params)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if let Ok(text) = response.text().await {
                                match serde_json::from_str::<TokenResponse>(&text) {
                                    Ok(token_data) => {
                                        let cookie =
                                            Cookie::build(("token", token_data.access_token))
                                                .max_age(Duration::new(token_data.expires_in as i64, 0))
                                                .same_site(SameSite::Lax);
                                        cookies.add_private(cookie);
                                        Redirect::to("/home")
                                    }
                                    Err(e) => Redirect::to("/error"),
                                }
                            } else {
                                println!("Failed to get response text");
                                Redirect::to("/error")
                            }
                        }
                        Err(e) => {
                            println!("token error: {}", e);
                            Redirect::to("/error")
                        }
                    }
                }
                Err(_) => Redirect::to("/error"),
            }
        }
        None => Redirect::to("/error"),
    }
}

#[get("/error")]
pub fn error(hbs: &State<Handlebars<'static>>) -> RawHtml<String> {
    let html = hbs
        .render(
            "error",
            &json!({
                "error": "Not logged in",
                "redirect": "/login"
            }),
        )
        .unwrap_or_else(|e| format!("Template error: {}", e));
    RawHtml(html)
}

#[get("/api/summary?<max>")]
pub async fn summary(max: String, cookies: &CookieJar<'_>) -> Json<Vec<SearchResult>> {
    match cookies.get_private("token") {
        Some(token) => {
            let max_results: u32 = max.parse().unwrap_or(10);
            let page_token = cookies.get_private("page_token").map(|c| c.value().to_string());
            match messages_list(token.value(), max_results, page_token.as_deref()).await {
                Ok(res) => {
                    let mut results = Vec::new();
                    for message in res.messages {
                        if let Ok(msg) = message_get(token.value(), &message.id).await {
                            results.push(SearchResult {
                                title: msg
                                    .payload
                                    .unwrap()
                                    .headers
                                    .into_iter()
                                    .find(|h| h.name == "Subject")
                                    .unwrap()
                                    .value,
                                size: msg.sizeEstimate,
                                thread_id: msg.threadId,
                            });
                        }
                    }
                    // Set the next page token as a private cookie
                    if let Some(next_token) = res.nextPageToken {
                        let page_cookie = Cookie::build(("page_token", next_token))
                            .same_site(SameSite::Lax);
                        cookies.add_private(page_cookie);
                    } else {
                        // If no next page, remove the cookie
                        cookies.remove_private("page_token");
                    }
                    Json(results)
                }
                Err(_) => Json(vec![]),
            }
        }
        None => Json(vec![]),
    }
}

#[get("/api/profile")]
pub async fn profile(cookies: &CookieJar<'_>) -> Json<User> {
    match cookies.get_private("token") {
        Some(token) => match user_get(token.value()).await {
            Ok(user) => Json(user),
            Err(_) => Json(User {
                email: String::new(),
            }),
        },
        None => Json(User {
            email: String::new(),
        }),
    }
}
