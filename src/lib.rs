use reqwest::{Client, Response};
use std::error::Error as StdError;
use serde_json::{json, Map, Value};



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

    pub async fn send(&self, content: &str) -> Result<(), Box<dyn StdError>> {
        let payload = json!({
            "content": content,
        });
        let response = self.post_json(&payload).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Unexpected response: {:?}", response).into())
        }
    }

    pub async fn send_embed(&self, embed: Value) -> Result<(), Box<dyn StdError>> {
        let payload = json!({
        "embeds": [embed]
    });

        let response = self.post_json(&payload).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Unexpected response: {:?}", response).into())
        }
    }

    pub fn create_embed(fields: &[(&str, &str)], color: Option<u32>) -> Value {
        let mut embed = Map::new();

        for (field, value) in fields {
            embed.insert(field.to_string(), Value::String(value.to_string()));
        }

        if let Some(color) = color {
            embed.insert("color".to_string(), Value::Number(color.into()));
        }

        Value::Object(embed)
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



