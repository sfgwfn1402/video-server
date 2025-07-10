use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use crate::services::video::VideoSnapshotService;

// 应用状态结构体
#[derive(Clone)]
pub struct AppState {
    pub concurrent_requests: Arc<AtomicUsize>,
    pub video_service: VideoSnapshotService,
} 