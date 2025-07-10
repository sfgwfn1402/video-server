use std::net::SocketAddr;
use std::env;

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

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("端口不能为0".to_string());
        }
        
        if self.clips_dir.is_empty() {
            return Err("clips目录不能为空".to_string());
        }
        
        if self.frontend_dir.is_empty() {
            return Err("frontend目录不能为空".to_string());
        }
        
        Ok(())
    }

    /// 创建必要的目录结构
    pub fn ensure_directories(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.clips_dir)?;
        Ok(())
    }

    /// 打印配置信息
    pub fn print_info(&self) {
        println!("📋 Configuration:");
        println!("   - Host: {:?}", self.host);
        println!("   - Port: {}", self.port);
        println!("   - Clips directory: {}", self.clips_dir);
        println!("   - Frontend directory: {}", self.frontend_dir);
        println!("   - Socket address: {}", self.socket_addr());
        println!("   - Use env vars: SERVER_HOST, SERVER_PORT, CLIPS_DIR, FRONTEND_DIR");
    }
} 