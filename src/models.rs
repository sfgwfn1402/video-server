use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use crate::video_snapshot::VideoSnapshotService;

// 应用状态结构体
#[derive(Clone)]
pub struct AppState {
    pub concurrent_requests: Arc<AtomicUsize>,
    pub video_service: VideoSnapshotService,
}

// 请求体结构体
#[derive(Deserialize)]
pub struct SnapshotRequest {
    pub url: String,
    pub timestamp: Option<f64>, // 可选的时间戳，单位秒
}

#[derive(Deserialize)]
pub struct ClipRequest {
    pub url: String,
    pub start: Option<f64>,
    pub duration: f64,
    pub return_url: Option<bool>, // 新增
}

#[derive(Serialize)]
pub struct ClipResponse {
    pub video_url: String,
}

// 并发请求统计结构体
#[derive(Serialize)]
pub struct ConcurrentStats {
    pub current_requests: usize,
    pub message: String,
}

// 系统监控统计结构体
#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_usage: f64,
    pub memory_total: u64,
    pub memory_used: u64,
    pub current_requests: usize,
    pub uptime: u64,
} 