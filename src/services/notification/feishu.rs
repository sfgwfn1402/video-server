use reqwest::Client;
use serde_json::json;

pub async fn send_feishu_webhook(webhook_url: &str, content: &str) -> Result<(), String> {
    let client = Client::new();
    let payload = json!({
        "msg_type": "text",
        "content": { "text": content }
    });
    let resp = client.post(webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send webhook: {}", e))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Webhook failed: {}", resp.status()))
    }
} 