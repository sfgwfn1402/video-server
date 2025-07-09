mod video_snapshot;
mod feishu_notify;

use axum::{
    routing::{get, post},
    Router,
    response::{IntoResponse, Response},
    extract::{Json, State},
    http::StatusCode,
    middleware::{self, Next},
    http::Request,
    body::Body,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpListener;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use video_snapshot::VideoSnapshotService;
use serde_json;
use crate::feishu_notify::send_feishu_webhook;
use sysinfo::System;

// 应用状态结构体
#[derive(Clone)]
struct AppState {
    concurrent_requests: Arc<AtomicUsize>,
    video_service: VideoSnapshotService,
}

// 请求体结构体
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

// 并发请求统计结构体
#[derive(Serialize)]
struct ConcurrentStats {
    current_requests: usize,
    message: String,
}

// 系统监控统计结构体
#[derive(Serialize)]
struct SystemStats {
    cpu_usage: f32,
    memory_usage: f64,
    memory_total: u64,
    memory_used: u64,
    current_requests: usize,
    uptime: u64,
}

// 并发请求统计中间件
async fn track_concurrent_requests(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // 请求开始时增加计数
    state.concurrent_requests.fetch_add(1, Ordering::SeqCst);
    let current_count = state.concurrent_requests.load(Ordering::SeqCst);
    
    tracing::info!("Request started. Current concurrent requests: {}", current_count);
    
    // 处理请求
    let response = next.run(request).await;
    
    // 请求结束时减少计数
    state.concurrent_requests.fetch_sub(1, Ordering::SeqCst);
    let final_count = state.concurrent_requests.load(Ordering::SeqCst);
    
    tracing::info!("Request finished. Current concurrent requests: {}", final_count);
    
    response
}

// 获取当前并发请求数量的API接口
async fn get_concurrent_requests(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let current_requests = state.concurrent_requests.load(Ordering::SeqCst);
    
    let stats = ConcurrentStats {
        current_requests,
        message: format!("当前正在处理 {} 个并发请求", current_requests),
    };
    
    (StatusCode::OK, Json(stats))
}

// 获取系统监控统计的API接口
async fn get_system_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // 获取CPU使用率 (所有核心的平均值)
    let cpu_usage = if !sys.cpus().is_empty() {
        sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / sys.cpus().len() as f32
    } else {
        0.0
    };
    
    // 获取内存信息
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let memory_usage = if memory_total > 0 {
        (memory_used as f64 / memory_total as f64) * 100.0
    } else {
        0.0
    };
    
    // 获取当前并发请求数
    let current_requests = state.concurrent_requests.load(Ordering::SeqCst);
    
    let stats = SystemStats {
        cpu_usage,
        memory_usage,
        memory_total,
        memory_used,
        current_requests,
        uptime: 0, // 暂时设为0，如果需要可以用其他方式获取
    };
    
    (StatusCode::OK, Json(stats))
}

// 返回图片二进制流
async fn take_snapshot(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SnapshotRequest>
) -> Response {
    tracing::info!("Received snapshot request for URL: {}", payload.url);
    
    // 默认在视频开始处截图，如果指定了时间戳则使用指定时间
    let timestamp = payload.timestamp.unwrap_or(0.0);
    tracing::info!("Taking snapshot at timestamp: {} seconds", timestamp);
    
    match state.video_service.capture_frame(&payload.url, timestamp).await {
        Ok(image_data) => {
            tracing::info!("Successfully captured frame, size: {} bytes", image_data.len());
            // 飞书通知：截图成功
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            if !webhook_url.is_empty() {
                let msg = format!(
                    "【截图成功】\nURL: {}\n时间戳: {} 秒\n图片大小: {} 字节\n当前并发请求: {}",
                    payload.url, timestamp, image_data.len(),
                    state.concurrent_requests.load(Ordering::SeqCst)
                );
                let webhook_url_clone = webhook_url.clone();
                let msg_clone = msg.clone();
                tokio::spawn(async move {
                    let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
                });
            }
            (
                [("Content-Type", "image/png")],
                image_data
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to capture frame: {}", e);
            // 飞书通知：截图失败
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            if !webhook_url.is_empty() {
                let msg = format!(
                    "【截图失败】\nURL: {}\n时间戳: {} 秒\n错误: {}\n当前并发请求: {}",
                    payload.url, timestamp, e,
                    state.concurrent_requests.load(Ordering::SeqCst)
                );
                let webhook_url_clone = webhook_url.clone();
                let msg_clone = msg.clone();
                tokio::spawn(async move {
                    let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
                });
            }
            // 返回错误图片或错误信息
            let error_img = create_error_image(&format!("Error: {}", e));
            (
                [("Content-Type", "image/png")],
                error_img
            ).into_response()
        }
    }
}

// 视频流截取接口
async fn clip_video(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ClipRequest>
) -> impl IntoResponse {
    let start = payload.start.unwrap_or(0.0);
    let duration = payload.duration;
    let return_url = payload.return_url.unwrap_or(true); // 默认true
    
    match state.video_service.clip_video(&payload.url, start, duration).await {
        Ok(filename) => {
            let video_path = format!("clips/{}", filename);
            // 飞书通知：截视频成功
            let webhook_url = std::env::var("FEISHU_WEBHOOK_URL").unwrap_or_default();
            if !webhook_url.is_empty() {
                let msg = format!(
                    "【视频截取成功】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n文件: {}\n当前并发请求: {}",
                    payload.url, start, duration, filename,
                    state.concurrent_requests.load(Ordering::SeqCst)
                );
                let webhook_url_clone = webhook_url.clone();
                let msg_clone = msg.clone();
                tokio::spawn(async move {
                    let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
                });
            }
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
            if !webhook_url.is_empty() {
                let msg = format!(
                    "【视频截取失败】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n错误: {}\n当前并发请求: {}",
                    payload.url, start, duration, e,
                    state.concurrent_requests.load(Ordering::SeqCst)
                );
                let webhook_url_clone = webhook_url.clone();
                let msg_clone = msg.clone();
                tokio::spawn(async move {
                    let _ = send_feishu_webhook(&webhook_url_clone, &msg_clone).await;
                });
            }
            let err = serde_json::json!({"error": format!("视频截取失败: {}", e)});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
        }
    }
}

fn create_error_image(_error_msg: &str) -> Vec<u8> {
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

    // 初始化应用状态
    let app_state = Arc::new(AppState {
        concurrent_requests: Arc::new(AtomicUsize::new(0)),
        video_service: VideoSnapshotService::new(),
    });

    // 配置 CORS
    let cors = CorsLayer::permissive();

    // 构建路由
    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello from API!" }))
        .route("/api/snapshot", post(take_snapshot))
        .route("/api/clip", post(clip_video))
        .route("/api/concurrent", get(get_concurrent_requests)) // 新增：并发请求统计接口
        .route("/api/system-stats", get(get_system_stats)) // 新增：系统监控统计接口
        .fallback_service(ServeDir::new("frontend/vue-project/dist"))
        .nest_service("/clips", ServeDir::new("clips"))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            track_concurrent_requests,
        )) // 添加并发统计中间件
        .layer(cors)
        .with_state(app_state);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running on http://{}:{}", addr.ip(), addr.port());
    println!("📊 Concurrent requests API: http://{}:{}/api/concurrent", addr.ip(), addr.port());
    tracing::info!("Server started successfully");

    // 运行服务
    axum::serve(listener, app)
        .await
        .unwrap();
}