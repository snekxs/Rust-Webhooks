use reqwest::{Client, Response};
use serde::{Serialize, Serializer};
use std::error::Error as StdError;
use serde_json::json;

pub struct DiscordWebhook {
    client: Client,
    hook_url: String,
}

impl DiscordWebhook {
    pub fn new(hook_url: &str) -> Self {
        DiscordWebhook {
            client: Client::new(),
            hook_url: hook_url.to_owned(),
        }
    }

    pub async fn send(&self, content: &str, embed: Option<Embed>) -> Result<(), Box<dyn StdError>> {
        let payload = json!({
            "content": content,
            "embeds": match embed {
                Some(e) => vec![e],
                None => vec![],
            }
        });
        let response = self.post_json(&payload).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Unexpected response: {:?}", response).into())
        }
    }

    pub async fn send_embed(&self, embed: Embed) -> Result<(), Box<dyn StdError>> {
        let payload = json!({
            "embeds": vec![embed],
        });
        let response = self.post_json(&payload).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Unexpected response: {:?}", response).into())
        }
    }

    async fn post_json(&self, payload: &serde_json::Value) -> Result<Response, Box<dyn StdError>> {
        let body = serde_json::to_string(payload)?;

        let response = self
            .client
            .post(&self.hook_url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        Ok(response)
    }
}

#[derive(Serialize)]
struct Embed {
    description: String,
    color: i32,
    timestamp: String,
}

impl Serialize for Embed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer
            .serialize_struct("Embed", 3)?
            .serialize_field("description", &self.description)?
            .serialize_field("color", &self.color)?
            .serialize_field("timestamp", &self.timestamp)?
            .end()
    }
}
