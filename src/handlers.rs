use axum::{
    response::{IntoResponse, Response},
    extract::{Json, State},
    http::StatusCode,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use sysinfo::System;
use serde_json;

use crate::models::{
    AppState, SnapshotRequest, ClipRequest, 
    ConcurrentStats, SystemStats
};
use crate::utils::{create_error_image, send_feishu_notification};

// 获取当前并发请求数量的API接口
pub async fn get_concurrent_requests(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let current_requests = state.concurrent_requests.load(Ordering::SeqCst);
    
    let stats = ConcurrentStats {
        current_requests,
        message: format!("当前正在处理 {} 个并发请求", current_requests),
    };
    
    (StatusCode::OK, Json(stats))
}

// 获取系统监控统计的API接口
pub async fn get_system_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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
pub async fn take_snapshot(
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
            let msg = format!(
                "【截图成功】\nURL: {}\n时间戳: {} 秒\n图片大小: {} 字节",
                payload.url, timestamp, image_data.len()
            );
            send_feishu_notification(&state, &msg).await;
            
            (
                [("Content-Type", "image/png")],
                image_data
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to capture frame: {}", e);
            
            // 飞书通知：截图失败
            let msg = format!(
                "【截图失败】\nURL: {}\n时间戳: {} 秒\n错误: {}",
                payload.url, timestamp, e
            );
            send_feishu_notification(&state, &msg).await;
            
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
pub async fn clip_video(
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
            let msg = format!(
                "【视频截取成功】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n文件: {}",
                payload.url, start, duration, filename
            );
            send_feishu_notification(&state, &msg).await;
            
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
            let msg = format!(
                "【视频截取失败】\nURL: {}\n起始: {} 秒\n时长: {} 秒\n错误: {}",
                payload.url, start, duration, e
            );
            send_feishu_notification(&state, &msg).await;
            
            let err = serde_json::json!({"error": format!("视频截取失败: {}", e)});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
        }
    }
} 