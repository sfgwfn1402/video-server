use axum::{
    middleware::Next,
    response::Response,
    extract::State,
    http::Request,
    body::Body,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::models::AppState;

// 并发请求统计中间件
pub async fn track_concurrent_requests(
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