use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use crate::core::config::AppConfig;
use crate::core::app::VideoServerApp;
use crate::models::AppState;
use crate::services::video::VideoSnapshotService;

/// 应用构建器
/// 
/// 使用建造者模式来组装应用的各个组件：
/// - 配置管理
/// - 环境初始化
/// - 应用状态构建
/// - 依赖注入
/// 
/// # Example
/// ```rust
/// let app = AppBuilder::new()
///     .with_config(config)
///     .init_environment()
///     .build();
/// 
/// app.run().await?;
/// ```
pub struct AppBuilder {
    config: AppConfig,
}

impl AppBuilder {
    /// 创建新的应用构建器
    /// 
    /// 使用默认配置初始化构建器
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    /// 从环境变量创建构建器
    /// 
    /// 自动从环境变量加载配置
    pub fn from_env() -> Self {
        Self {
            config: AppConfig::from_env(),
        }
    }

    /// 设置配置
    /// 
    /// # Arguments
    /// * `config` - 应用配置实例
    pub fn with_config(mut self, config: AppConfig) -> Self {
        self.config = config;
        self
    }

    /// 初始化环境
    /// 
    /// 执行以下初始化操作：
    /// - 加载.env文件
    /// - 创建必要目录
    /// - 初始化日志系统
    /// - 验证配置
    pub fn init_environment(self) -> Result<Self, String> {
        // 加载环境变量文件
        dotenv::dotenv().ok();
        
        // 验证配置
        self.config.validate()
            .map_err(|e| format!("配置验证失败: {}", e))?;
        
        // 创建必要目录
        self.config.ensure_directories()
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        // 初始化日志系统
        self.init_logging();
        
        tracing::info!("环境初始化完成");
        tracing::debug!("配置信息: {:?}", self.config);
        
        Ok(self)
    }

    /// 构建应用状态
    /// 
    /// 创建并配置应用运行时需要的状态对象
    fn build_app_state(&self) -> Arc<AppState> {
        tracing::info!("构建应用状态...");
        
        Arc::new(AppState {
            concurrent_requests: Arc::new(AtomicUsize::new(0)),
            video_service: VideoSnapshotService::new(),
        })
    }

    /// 构建完整的应用
    /// 
    /// 组装所有组件并返回可运行的应用实例
    /// 
    /// # Returns
    /// 配置完成的VideoServerApp实例
    pub fn build(self) -> VideoServerApp {
        tracing::info!("构建应用实例...");
        
        // 构建应用状态
        let app_state = self.build_app_state();
        
        // 创建应用实例
        let app = VideoServerApp::new(self.config, app_state);
        
        tracing::info!("应用构建完成");
        
        app
    }

    /// 初始化日志系统
    /// 
    /// 配置tracing日志系统，支持不同级别的日志输出
    fn init_logging(&self) {
        // 简化的日志初始化，使用默认配置
        tracing_subscriber::fmt()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .init();
        
        // 从环境变量获取日志级别，默认为info
        let log_level = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());
            
        tracing::info!("日志系统已初始化，级别: {}", log_level);
    }

    /// 快速构建 - 便捷方法
    /// 
    /// 结合环境初始化和构建的快捷方式
    /// 
    /// # Returns
    /// Result包装的VideoServerApp实例
    pub fn quick_build() -> Result<VideoServerApp, String> {
        Self::from_env()
            .init_environment()?
            .build_result()
    }

    /// 构建并返回Result
    /// 
    /// 提供Result包装的构建结果，便于错误处理
    fn build_result(self) -> Result<VideoServerApp, String> {
        Ok(self.build())
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
} 