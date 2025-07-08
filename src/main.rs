mod video_snapshot;
mod feishu_notify;

use axum::{
    routing::{get, post},
    Router,
    response::{IntoResponse, Response},
    extract::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use video_snapshot::VideoSnapshotService;
use serde_json;
use crate::feishu_notify::send_feishu_webhook;

// 新增：请求体结构体
#[derive(Deserialize)]
struct SnapshotRequest {
    url: String,
    timestamp: Option<f64>, // 可选的时间戳，单位秒
}

#[derive(Deserialize)]
struct ClipRequest {
    url: String,
    start: Option<f64>,
    duration: f64,
    return_url: Option<bool>, // 新增
}

#[derive(Serialize)]
struct ClipResponse {
    video_url: String,
}

// 新增：返回图片二进制流
async fn take_snapshot(Json(payload): Json<SnapshotRequest>) -> Response {
    tracing::info!("Received snapshot request for URL: {}", payload.url);
    
    // 默认在视频开始处截图，如果指定了时间戳则使用指定时间
    let timestamp = payload.timestamp.unwrap_or(0.0);
    tracing::info!("Taking snapshot at timestamp: {} seconds", timestamp);
    
    let service = VideoSnapshotService::new();
    match service.capture_frame(&payload.url, timestamp).await {
        Ok(image_data) => {
            tracing::info!("Successfully captured frame, size: {} bytes", image_data.len());
            // 飞书通知：截图成功
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            let msg = format!(
                "【截图成功】\nURL: {}\n时间戳: {} 秒\n图片大小: {} 字节",
                payload.url, timestamp, image_data.len()
            );
            let webhook_url_clone = webhook_url.clone();
            let msg_clone = msg.clone();
            tokio::spawn(async move {
                let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
            });
            (
                [("Content-Type", "image/png")],
                image_data
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to capture frame: {}", e);
            // 飞书通知：截图失败
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            let msg = format!(
                "【截图失败】\nURL: {}\n时间戳: {} 秒\n错误: {}",
                payload.url, timestamp, e
            );
            let webhook_url_clone = webhook_url.clone();
            let msg_clone = msg.clone();
            tokio::spawn(async move {
                let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
            });
            // 返回错误图片或错误信息
            let error_img = create_error_image(&format!("Error: {}", e));
            (
                [("Content-Type", "image/png")],
                error_img
            ).into_response()
        }
    }
}

// 新增：视频流截取接口
async fn clip_video(Json(payload): Json<ClipRequest>) -> impl IntoResponse {
    let start = payload.start.unwrap_or(0.0);
    let duration = payload.duration;
    let return_url = payload.return_url.unwrap_or(true); // 默认true
    let service = VideoSnapshotService::new();
    match service.clip_video(&payload.url, start, duration).await {
        Ok(filename) => {
            let video_path = format!("clips/{}", filename);
            // 飞书通知：截视频成功
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            let msg = format!(
                "【视频截取成功】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n文件: {}",
                payload.url, start, duration, filename
            );
            let webhook_url_clone = webhook_url.clone();
            let msg_clone = msg.clone();
            tokio::spawn(async move {
                let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
            });
            if return_url {
                // 返回地址
                let video_url = format!("/clips/{}", filename);
                (StatusCode::OK, Json(serde_json::json!({ "video_url": video_url }))).into_response()
            } else {
                // 直接返回视频流
                match std::fs::read(&video_path) {
                    Ok(data) => (
                        [("Content-Type", "video/mp4")],
                        data
                    ).into_response(),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("读取视频文件失败: {}", e)
                    ).into_response(),
                }
            }
        }
        Err(e) => {
            // 飞书通知：截视频失败
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            let msg = format!(
                "【视频截取失败】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n错误: {}",
                payload.url, start, duration, e
            );
            let webhook_url_clone = webhook_url.clone();
            let msg_clone = msg.clone();
            tokio::spawn(async move {
                let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
            });
            let err = serde_json::json!({"error": format!("视频截取失败: {}", e)});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
        }
    }
}

fn create_error_image(error_msg: &str) -> Vec<u8> {
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    std::fs::create_dir_all("clips").ok();
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 配置 CORS
    let cors = CorsLayer::permissive();

    // 构建路由
    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello from API!" }))
        .route("/api/snapshot", post(take_snapshot))
        .route("/api/clip", post(clip_video))
        .fallback_service(ServeDir::new("frontend/vue-project/dist"))
        .nest_service("/clips", ServeDir::new("clips"))
        .layer(cors);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}:{}", addr.ip(), addr.port());
    tracing::info!("Server started successfully");

    // 运行服务
    axum::serve(listener, app)
        .await
        .unwrap();
}