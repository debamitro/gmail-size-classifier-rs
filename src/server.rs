use rocket::http::{Cookie, CookieJar};
use rocket::{get, response::Redirect, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};
use urlencoding;
use reqwest;

#[derive(Serialize)]
pub struct SearchResult {
    title: String,
    size: i32,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
}

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Redirect {
    match cookies.get_private("token") {
        Some(_cookie) => {
            Redirect::to("/home")
        }
        None => {
            Redirect::to("/login")
        }
    }
}

#[get("/home")]
pub fn home(cookies: &CookieJar<'_>) -> Template {
    match cookies.get_private("token") {
        Some(_cookie) => Template::render("index", &context! {}),
        None => Template::render("/error", context! { 
            error: "Not logged in",
            redirect: "/login"
        }),
    }
}

#[get("/login")]
pub fn login(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("token");
    
    let scope = urlencoding::encode("https://www.googleapis.com/auth/gmail.readonly");
    let redirect_uri = urlencoding::encode("http://127.0.0.1:5000/oauth2callback");
    let client_id = "";
    
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?scope={}&redirect_uri={}&response_type=code&client_id={}", 
        scope,
        redirect_uri,
        client_id
    );
    
    Redirect::to(auth_url)
}

#[get("/oauth2callback?<code>&<state>&<scope>&<authuser>&<prompt>")]
pub async fn oauth2_callback(
    code: Option<String>,
    state: Option<String>,
    scope: Option<String>,
    authuser: Option<String>,
    prompt: Option<String>,
    cookies: &CookieJar<'_>
) -> Redirect {
    println!("OAuth2 callback received with params:");
    println!("  code: {:?}", code);
    println!("  state: {:?}", state);
    println!("  scope: {:?}", scope);
    println!("  authuser: {:?}", authuser);
    println!("  prompt: {:?}", prompt);

    match code {
        Some(code) => {
            let client = reqwest::Client::new();
            let params = [
                ("client_id", ""),
                ("client_secret", ""),
                ("code", &code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", "http://127.0.0.1:5000/oauth2callback")
            ];

            match client.post("https://oauth2.googleapis.com/token")
                .form(&params)
                .send()
                .await {
                    Ok(response) => {
                        if let Ok(text) = response.text().await {
                            println!("Token response: {:?}", &text);
                            match serde_json::from_str::<TokenResponse>(&text) {
                                Ok(token_data) => {
                                    // Store the access token in a cookie
                                    cookies.add_private(Cookie::new("token", token_data.access_token));
                                    Redirect::to("/")
                                }
                                Err(e) => {
                                    println!("json parsing error: {}", e);
                                    Redirect::to("/error")
                                }
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
        None => {
            Redirect::to("/error")       
        }
    }
}

#[get("/error")]
pub fn error() -> Template {
    Template::render("error", context! { 
        error: "Not logged in",
        redirect: "/login"
    })
}

#[derive(Deserialize)]
struct MessagePart {
    partId: String
}

#[derive(Deserialize)]
struct Message {
    id : String,
    threadId : String,
    labelIds : Vec<String>,
    snippet : String,
    historyId : String,
    internalDate: String,
    payload: Option<MessagePart>,
    sizeEstimate: i32,
    raw: Option<String>
}

#[derive(Deserialize)]
struct MessageListEntry {
    id: String,
    threadId: String
}

#[derive(Deserialize)]
struct MessagesList {
    messages: Vec<MessageListEntry>,
    nextPageToken: Option<String>,
    resultSizeEstimate: Option<i32>,
}

async fn messages_list (token: &str, _max_results: u32) -> Result<Vec<MessageListEntry>, ()>{
    let client = reqwest::Client::new();
    let result = client
    .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
    .header("Authorization", format!("Bearer {}", token))
    .send()
    .await;
    match result {
        Ok(response) => {
            match response.json::<MessagesList>().await {
                Ok(message_list) => {
                    Ok(message_list.messages)
                },
                Err(e) => {
                    println!("json parsing error: {}", e);
                    Err(())
                }
            }
        },
        Err(e) => {
            println!("request error: {}", e);
            Err(())
        }
    }
}
async fn message_get (token: &str, id: &str) -> Result<Message, ()> {
    let client = reqwest::Client::new();
    let result = client
        .get(format!("https://gmail.googleapis.com/gmail/v1/users/me/messages/{}", id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    match result {
        Ok(response) => {
            if let Ok(body) = response.text().await {
                if let Ok(message) = serde_json::from_str(&body) {
                    Ok(message)
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
        Err(_) => {
            Err(())
        }
    }
}

#[get("/api/summary?<max>")]
pub async fn summary(max: String, cookies: &CookieJar<'_>) -> Json<Vec<SearchResult>> {
    match cookies.get_private("token") {
        Some(token) => {
            let max_results: u32 = max.parse().unwrap_or(10);
            
            match messages_list (token.value(), max_results).await {
                Ok(messages) => {
                    let mut results = Vec::new();
                    for message in messages {
                        if let Ok(msg) = message_get (token.value(),&message.id).await {
                            results.push(SearchResult {
                                    title: msg.snippet,
                                    size: msg.sizeEstimate
                            });
                        }
                    }
                    Json(results)
                },
                Err(_) => {
                    Json(vec![])
                }
            }
        },
        None => {
            Json(vec![])
        }
    }
}
