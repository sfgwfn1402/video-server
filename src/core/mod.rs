//! 应用核心架构模块
//! 
//! 包含应用的核心组件：
//! - 配置管理 (config)
//! - 应用运行时 (app) 
//! - 应用构建器 (builder)
//! 
//! # 设计理念
//! 
//! 采用分层架构和关注点分离：
//! - **配置层**: 管理应用配置和环境变量
//! - **构建层**: 使用建造者模式组装应用
//! - **运行层**: 管理服务器生命周期和路由

pub mod config;
pub mod app; 
pub mod builder;

// 重新导出常用类型，简化导入
pub use config::AppConfig;
pub use app::VideoServerApp;
pub use builder::AppBuilder; 