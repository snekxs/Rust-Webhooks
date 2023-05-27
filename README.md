# **rust_webhook**

The **'rust_webhook'** crate is a Rust library that provides functionality for sending webhooks using the **'reqwest'** and **'serde_json'** libraries. It allows you to easily send POST requests with JSON payloads to webhook endpoints.


# **How to Use**

To use the **'rust_webhook'** crate in your Rust project, follow these steps:

1. Add the crate as a dependency in your **Cargo.toml** file:

```toml
[dependencies]
rust_webhook = { version = "0.1" }
reqwest = "0.11"
serde_json = "1.0"
```
2. Import the `send_hook` function from the `rust_webhook` crate:

```rust
use rust_webhook::send_hook;
```
3. Use the send_hook function to send a webhook. Provide the webhook URL and content as arguments. The function returns a Result<(), Box<dyn StdError>>, indicating success or failure.

```rust
#[tokio::main]
async fn main() {
    let hook_url = "https://your-webhook-url";
    let content = "Hello, webhook!";
  
    match send_hook(hook_url, content).await {
        Ok(()) => {
            println!("Webhook sent successfully!");
        }
        Err(err) => {
            eprintln!("Failed to send webhook: {}", err);
        }
    }
}
```
Replace "https://your-webhook-url" with the actual URL of your webhook endpoint.

4. Run your Rust program. If the webhook is sent successfully, it will print a success message. If an error occurs, it will print the error message.

That's it! You can now use the rust_webhook crate to send webhooks in your Rust project.