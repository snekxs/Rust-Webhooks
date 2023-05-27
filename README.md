# **rust_webhook**

The **'rust_webhook'** crate is a Rust library that provides functionality for sending webhooks using the **'reqwest'** and **'serde_json'** libraries. It allows you to easily send POST requests with JSON payloads to webhook endpoints.


# Installation



Add the crate as a dependency in your **Cargo.toml** file:

```toml
[dependencies]
rust_webhook = { version = "0.1.5" }
reqwest = "0.11"
serde_json = "1.0"
```


# Quick Start



###  Sending Messages 

```rust
use rust_webhook::DiscordWebhook;

#[tokio::main]
async fn main() {
    let webhook = DiscordWebhook::new("YOUR_WEBHOOK_URL");
    let content = "Hello, webhook!";

    webhook.send(content).await;
}
```
