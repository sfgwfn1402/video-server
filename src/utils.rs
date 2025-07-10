use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::sync::atomic::Ordering;
use crate::feishu_notify::send_feishu_webhook;
use crate::models::AppState;

pub fn create_error_image(_error_msg: &str) -> Vec<u8> {
    // 创建一个错误提示图片
    let imgx = 400;
    let imgy = 200;
    let mut img = image::RgbImage::new(imgx, imgy);
    
    // 填充红色背景
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([255, 200, 200]);
    }
    
    // 编码为 PNG
    let mut buf = Vec::new();
    {
        let encoder = PngEncoder::new(&mut buf);
        encoder.write_image(
            img.as_raw(),
            imgx,
            imgy,
            image::ExtendedColorType::Rgb8,
        ).unwrap();
    }
    
    buf
}

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