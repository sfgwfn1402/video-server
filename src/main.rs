mod api;
mod config;
mod models;
mod services;
mod utils;

use config::{VideoServerApp, AppConfig};

/// 视频服务器应用启动类
/// 
/// 类似于 Spring Boot 的 @SpringBootApplication
/// 支持配置：
/// - SERVER_HOST: 监听地址 (默认: 0.0.0.0)  
/// - SERVER_PORT: 监听端口 (默认: 3000)
/// - CLIPS_DIR: 视频片段目录 (默认: clips)
/// - FRONTEND_DIR: 前端静态文件目录 (默认: frontend/vue-project/dist)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spring Boot 风格的应用启动
    VideoServerApp::create()
        .with_config(AppConfig::from_env())  // 从环境变量加载配置
        .init_environment()                   // 初始化环境
        .build()                             // 构建应用
        .run()                               // 启动运行
        .await?;

    Ok(())
}