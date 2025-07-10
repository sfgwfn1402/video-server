mod video_snapshot;
mod feishu_notify;
mod models;
mod handlers;
mod middleware;
mod utils;

use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

use models::AppState;
use video_snapshot::VideoSnapshotService;
use handlers::{take_snapshot, clip_video, get_concurrent_requests, get_system_stats};
use middleware::track_concurrent_requests;

#[tokio::main]
async fn main() {
    // 初始化环境
    dotenv::dotenv().ok();
    std::fs::create_dir_all("clips").ok();
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
        .route("/api/concurrent", get(get_concurrent_requests))
        .route("/api/system-stats", get(get_system_stats))
        .fallback_service(ServeDir::new("frontend/vue-project/dist"))
        .nest_service("/clips", ServeDir::new("clips"))
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            track_concurrent_requests,
        ))
        .layer(cors)
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("🚀 Server running on http://{}:{}", addr.ip(), addr.port());
    println!("📊 Concurrent requests API: http://{}:{}/api/concurrent", addr.ip(), addr.port());
    tracing::info!("Server started successfully");

    axum::serve(listener, app).await.unwrap();
}