use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
pub struct User {
    pub email: String,
}

#[derive(Deserialize)]
pub struct Profile {
    pub emailAddress: String,
    pub messagesTotal: i32,
    pub threadsTotal: i32,
    pub historyId: String,
}

#[derive(Deserialize)]
pub struct MessageHeader {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct MessagePartBody {
    pub size: i32,
    pub data: Option<String>,
    pub attachmentId: Option<String>,
}

#[derive(Deserialize)]
pub struct MessagePart {
    pub partId: String,
    pub mimeType: String,
    pub filename: String,
    pub headers: Vec<MessageHeader>,
    pub body: MessagePartBody,
    pub parts: Option<Vec<MessagePart>>,
}

#[derive(Deserialize)]
pub struct Message {
    pub id: String,
    pub threadId: String,
    pub labelIds: Vec<String>,
    pub snippet: String,
    pub historyId: String,
    pub internalDate: String,
    pub payload: Option<MessagePart>,
    pub sizeEstimate: i32,
    pub raw: Option<String>,
}

#[derive(Deserialize)]
pub struct MessageListEntry {
    pub id: String,
    pub threadId: String,
}

#[derive(Deserialize)]
pub struct MessagesList {
    pub messages: Vec<MessageListEntry>,
    pub nextPageToken: Option<String>,
    pub resultSizeEstimate: Option<i32>,
}

/// Retrieves a list of messages from the user's Gmail account
/// by calling the users.messages.list method of the Gmail API.
///
/// # Arguments
/// * `token` - The access token for Gmail API authentication.
/// * `max_results` - The maximum number of messages to return.
/// * `page_token` - Optional token for pagination.
///
/// # Returns
/// A Result containing MessagesList on success or an error on failure.
pub async fn messages_list(
    token: &str,
    max_results: u32,
    page_token: Option<&str>,
) -> Result<MessagesList, ()> {
    let client = reqwest::Client::new();
    let mut request = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("maxResults", max_results)]);

    if let Some(token) = page_token {
        request = request.query(&[("pageToken", token)]);
    }

    let result = request.send().await;
    match result {
        Ok(response) => match response.json::<MessagesList>().await {
            Ok(message_list) => Ok(message_list),
            Err(e) => {
                println!("json parsing error: {}", e);
                Err(())
            }
        },
        Err(e) => {
            println!("request error: {}", e);
            Err(())
        }
    }
}

/// Retrieves a specific message from the user's Gmail account
/// by calling the users.messages.get method of the Gmail API.
///
/// # Arguments
/// * `token` - The access token for Gmail API authentication.
/// * `id` - The ID of the message to retrieve.
///
/// # Returns
/// A Result containing Message on success or an error on failure.
pub async fn message_get(token: &str, id: &str) -> Result<Message, ()> {
    let client = reqwest::Client::new();
    let result = client
        .get(format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
            id
        ))
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
        Err(_) => Err(()),
    }
}

pub async fn user_get(token: &str) -> Result<User, ()> {
    let client = reqwest::Client::new();
    let result = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/profile")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    match result {
        Ok(response) => {
            if let Ok(profile) = response.json::<Profile>().await {
                Ok(User {
                    email: profile.emailAddress,
                })
            } else {
                Err(())
            }
        }
        Err(_) => Err(()),
    }
}
