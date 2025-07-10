use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

use crate::core::config::AppConfig;
use crate::models::AppState;
use crate::api::{take_snapshot, clip_video, get_concurrent_requests, get_system_stats, track_concurrent_requests};

/// 视频服务器应用
/// 
/// 负责应用的运行时管理，包括：
/// - 服务器启动和生命周期管理
/// - 路由注册和中间件配置
/// - 静态文件服务
/// - CORS和并发请求跟踪
pub struct VideoServerApp {
    config: AppConfig,
    router: Router,
}

impl VideoServerApp {
    /// 创建新的应用实例
    /// 
    /// # Arguments
    /// * `config` - 应用配置
    /// * `app_state` - 应用状态
    /// 
    /// # Returns
    /// 配置好的应用实例
    pub fn new(config: AppConfig, app_state: Arc<AppState>) -> Self {
        let router = Self::build_router(&config, app_state);
        
        Self {
            config,
            router,
        }
    }

    /// 启动服务器
    /// 
    /// # Returns
    /// 启动结果，成功返回Ok(())，失败返回错误信息
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.config.socket_addr();
        
        // 验证配置
        self.config.validate()
            .map_err(|e| format!("配置验证失败: {}", e))?;
        
        // 确保目录存在
        self.config.ensure_directories()
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        // 打印启动信息
        self.print_startup_info(&addr);
        
        // 创建监听器
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::info!("Server started successfully at {}", addr);
        
        // 启动服务
        axum::serve(listener, self.router).await?;
        
        Ok(())
    }

    /// 构建路由
    /// 
    /// 注册所有的API端点和中间件
    fn build_router(config: &AppConfig, app_state: Arc<AppState>) -> Router {
        Router::new()
            // API路由
            .route("/api/hello", get(|| async { "Hello from Video Server API!" }))
            .route("/api/snapshot", post(take_snapshot))
            .route("/api/clip", post(clip_video))
            .route("/api/concurrent", get(get_concurrent_requests))
            .route("/api/system-stats", get(get_system_stats))
            
            // 静态文件服务 - clips目录
            .nest_service(
                &format!("/{}", &config.clips_dir), 
                ServeDir::new(&config.clips_dir)
            )
            
            // 前端静态文件服务（fallback）
            .fallback_service(ServeDir::new(&config.frontend_dir))
            
            // 中间件
            .layer(axum_middleware::from_fn_with_state(
                app_state.clone(),
                track_concurrent_requests,
            ))
            .layer(CorsLayer::permissive())
            
            // 应用状态
            .with_state(app_state)
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
        
        // 打印配置信息
        self.config.print_info();
        
        println!();
        
        // 打印API端点
        self.print_api_endpoints(addr);
    }

    /// 打印API端点信息
    fn print_api_endpoints(&self, addr: &SocketAddr) {
        let base_url = format!("http://{}:{}", addr.ip(), addr.port());
        
        println!("🔗 Available API Endpoints:");
        println!("   GET  {}/api/hello         - 健康检查", base_url);
        println!("   POST {}/api/snapshot      - 视频截图", base_url);
        println!("   POST {}/api/clip          - 视频剪辑", base_url);
        println!("   GET  {}/api/concurrent    - 并发请求统计", base_url);
        println!("   GET  {}/api/system-stats  - 系统状态监控", base_url);
        println!("   GET  {}/{}/*              - 视频片段文件", base_url, self.config.clips_dir);
        println!("   GET  {}/*                 - 前端静态文件", base_url);
        println!();
    }
} 