use serde::Serialize;

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