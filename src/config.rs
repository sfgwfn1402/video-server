use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use std::env;

use crate::models::AppState;
use crate::services::video::VideoSnapshotService;
use crate::api::{take_snapshot, clip_video, get_concurrent_requests, get_system_stats, track_concurrent_requests};

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: [u8; 4],
    pub port: u16,
    pub clips_dir: String,
    pub frontend_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: [0, 0, 0, 0],
            port: 3000,
            clips_dir: "clips".to_string(),
            frontend_dir: "frontend/vue-project/dist".to_string(),
        }
    }
}

impl AppConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // 从环境变量读取配置
        if let Ok(port) = env::var("SERVER_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.port = port_num;
            }
        }
        
        if let Ok(clips_dir) = env::var("CLIPS_DIR") {
            config.clips_dir = clips_dir;
        }
        
        if let Ok(frontend_dir) = env::var("FRONTEND_DIR") {
            config.frontend_dir = frontend_dir;
        }
        
        if let Ok(host) = env::var("SERVER_HOST") {
            if let Ok(addr) = host.parse::<std::net::Ipv4Addr>() {
                config.host = addr.octets();
            }
        }
        
        config
    }

    /// 设置监听地址
    pub fn with_host(mut self, host: [u8; 4]) -> Self {
        self.host = host;
        self
    }

    /// 设置端口
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// 设置视频片段存储目录
    pub fn with_clips_dir(mut self, dir: &str) -> Self {
        self.clips_dir = dir.to_string();
        self
    }

    /// 设置前端静态文件目录
    pub fn with_frontend_dir(mut self, dir: &str) -> Self {
        self.frontend_dir = dir.to_string();
        self
    }

    /// 获取监听地址
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}

/// 应用构建器
pub struct AppBuilder {
    config: AppConfig,
}

impl AppBuilder {
    /// 创建新的应用构建器
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    /// 设置配置
    pub fn with_config(mut self, config: AppConfig) -> Self {
        self.config = config;
        self
    }

    /// 初始化环境
    pub fn init_environment(self) -> Self {
        // 加载环境变量
        dotenv::dotenv().ok();
        
        // 创建必要目录
        std::fs::create_dir_all(&self.config.clips_dir).ok();
        
        // 初始化日志
        tracing_subscriber::fmt::init();
        
        self
    }

    /// 构建应用状态
    fn build_app_state(&self) -> Arc<AppState> {
        Arc::new(AppState {
            concurrent_requests: Arc::new(AtomicUsize::new(0)),
            video_service: VideoSnapshotService::new(),
        })
    }

    /// 构建路由
    fn build_router(&self, app_state: Arc<AppState>) -> Router {
        Router::new()
            .route("/api/hello", get(|| async { "Hello from API!" }))
            .route("/api/snapshot", post(take_snapshot))
            .route("/api/clip", post(clip_video))
            .route("/api/concurrent", get(get_concurrent_requests))
            .route("/api/system-stats", get(get_system_stats))
            .fallback_service(ServeDir::new(&self.config.frontend_dir))
            .nest_service(&format!("/{}", &self.config.clips_dir), ServeDir::new(&self.config.clips_dir))
            .layer(axum_middleware::from_fn_with_state(
                app_state.clone(),
                track_concurrent_requests,
            ))
            .layer(CorsLayer::permissive())
            .with_state(app_state)
    }

    /// 构建完整的应用
    pub fn build(self) -> VideoServerApp {
        let app_state = self.build_app_state();
        let router = self.build_router(app_state);
        
        VideoServerApp {
            config: self.config,
            router,
        }
    }
}

/// 视频服务器应用
pub struct VideoServerApp {
    config: AppConfig,
    router: Router,
}

impl VideoServerApp {
    /// 创建新的应用实例
    pub fn create() -> AppBuilder {
        AppBuilder::new()
    }

    /// 启动服务器
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.config.socket_addr();
        
        // 启动信息
        self.print_startup_info(&addr);
        
        // 创建监听器
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::info!("Server started successfully");
        
        // 启动服务
        axum::serve(listener, self.router).await?;
        
        Ok(())
    }

    /// 打印启动信息
    fn print_startup_info(&self, addr: &SocketAddr) {
        println!();
        println!("🚀 Video Server Starting...");
        println!("📡 Server running on http://{}:{}", addr.ip(), addr.port());
        println!("📊 Monitoring API: http://{}:{}/api/system-stats", addr.ip(), addr.port());
        println!("🎥 Clips directory: {}", self.config.clips_dir);
        println!("🌐 Frontend directory: {}", self.config.frontend_dir);
        println!("✨ Ready to process video streams!");
        println!("📋 Configuration:");
        println!("   - Host: {:?}", self.config.host);
        println!("   - Port: {}", self.config.port);
        println!("   - Use env vars: SERVER_HOST, SERVER_PORT, CLIPS_DIR, FRONTEND_DIR");
        println!();
    }
} 