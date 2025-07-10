mod api;
mod core;
mod models;
mod services;
mod utils;

use core::AppBuilder;

/// 视频服务器应用启动类
/// 
/// 类似于 Spring Boot 的 @SpringBootApplication
/// 支持配置：
/// - SERVER_HOST: 监听地址 (默认: 0.0.0.0)  
/// - SERVER_PORT: 监听端口 (默认: 3000)
/// - CLIPS_DIR: 视频片段目录 (默认: clips)
/// - FRONTEND_DIR: 前端静态文件目录 (默认: frontend/vue-project/dist)
/// - RUST_LOG: 日志级别 (默认: info)
/// 
/// # 设计模式说明
/// 
/// 采用了以下设计模式：
/// - **建造者模式** (AppBuilder): 组装应用各组件
/// - **单一职责原则**: 每个模块职责清晰
///   - config.rs: 配置管理
///   - app.rs: 应用运行时
///   - builder.rs: 应用构建器
/// - **依赖注入**: 通过AppState管理应用状态
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用建造者模式启动应用
    // 从环境变量自动加载配置并构建应用
    let app = AppBuilder::quick_build()
        .map_err(|e| format!("应用构建失败: {}", e))?;
    
    // 启动服务器
    app.run().await?;

    Ok(())
}