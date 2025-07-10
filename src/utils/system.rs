use std::sync::atomic::Ordering;
use crate::models::AppState;
use crate::services::notification::send_feishu_webhook;

pub async fn send_feishu_notification(
    state: &AppState,
    message: &str,
) {
    let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
    if !webhook_url.is_empty() {
        let current_requests = state.concurrent_requests.load(Ordering::SeqCst);
        let full_message = format!("{}\n当前并发请求: {}", message, current_requests);
        
        let webhook_url_clone = webhook_url.clone();
        let msg_clone = full_message.clone();
        tokio::spawn(async move {
            let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
        });
    }
} 