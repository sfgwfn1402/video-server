use serde::Deserialize;

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