use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::error::Error as StdError;

pub async fn send_hook(hook_url: &str, content: &str) -> Result<(), Box<dyn StdError>> {
    let client = Client::new();
    let payload = json!({"content": content});

    let response = post_json(&client, hook_url, &payload).await?;

    // Handle the response as needed
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Unexpected response: {:?}", response).into())
    }
}

async fn post_json(
    client: &Client,
    url: &str,
    payload: &Value,
) -> Result<Response, Box<dyn StdError>> {
    let body = serde_json::to_string(payload)?;

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    Ok(response)
}
