# 🏗️ Video Server 架构快速指南

## 📁 项目结构总览

```
video-server/
├── 🚀 src/main.rs                    # 应用启动入口 (23行)
├── ⚙️ src/config.rs                  # 配置管理模块 (Spring Boot风格)
├── 📡 src/api/                       # API层 - HTTP请求处理
│   ├── mod.rs                        # API模块导出
│   ├── handlers.rs                   # 业务处理函数
│   └── middleware.rs                 # 中间件 (并发监控)
├── 📊 src/models/                    # 数据模型层
│   ├── mod.rs                        # 模型导出
│   ├── app_state.rs                  # 应用状态
│   ├── requests.rs                   # 请求数据结构
│   └── responses.rs                  # 响应数据结构
├── 🎥 src/services/                  # 业务服务层
│   ├── mod.rs                        # 服务导出
│   ├── video/                        # 视频处理服务
│   │   ├── mod.rs                    # 视频模块导出
│   │   ├── snapshot.rs               # 视频截图服务 (核心)
│   │   └── stream_handler.rs         # 流处理器 (多协议支持)
│   └── notification/                 # 通知服务
│       ├── mod.rs                    # 通知模块导出
│       └── feishu.rs                 # 飞书通知集成
├── 🔧 src/utils/                     # 工具函数层
│   ├── mod.rs                        # 工具导出
│   ├── image.rs                      # 图像处理工具
│   └── system.rs                     # 系统信息工具
├── 🌐 frontend/vue-project/          # Vue.js前端
└── 📋 config.env.example             # 配置示例
```

## 🔄 数据流向图

```
HTTP请求 → API层 → 服务层 → 工具层
    ↓        ↓       ↓       ↓
中间件监控  处理器   FFmpeg  系统调用
    ↓        ↓       ↓       ↓
并发统计   业务逻辑  视频处理  文件操作
    ↓        ↓       ↓       ↓
状态更新   响应构建  结果输出  资源管理
```

## 🎯 核心组件速览

### 🚀 启动流程 (main.rs)
```rust
VideoServerApp::create()                 // 1. 创建应用构建器
    .with_config(AppConfig::from_env())   // 2. 加载环境配置
    .init_environment()                   // 3. 初始化环境
    .build()                             // 4. 构建应用
    .run()                               // 5. 启动服务器
```

### ⚙️ 配置系统 (config.rs)
- **AppConfig**: 应用配置 (端口、目录等)
- **AppBuilder**: 构建器模式 (像Spring Boot)
- **VideoServerApp**: 主应用类
- **环境变量支持**: SERVER_PORT, CLIPS_DIR等

### 📡 API层架构
```rust
/api/hello          → 健康检查
/api/snapshot       → 视频截图 (POST)
/api/clip          → 视频剪辑 (POST)
/api/concurrent    → 并发统计 (GET)
/api/system-stats  → 系统监控 (GET)
/clips/*           → 静态文件服务
```

### 🎥 核心业务服务

#### VideoSnapshotService (视频截图)
```rust
// 支持多种协议
RTSP: rtsp://camera/stream
RTMP: rtmp://live/stream  
HLS:  http://server/playlist.m3u8
HTTP: http://server/video.mp4

// 智能参数适配
realmonitor摄像头 → 简化参数
普通流媒体     → 完整参数
```

#### 音频转码处理
```rust
pcm_alaw → aac (自动检测转换)
设置比特率: 128kbps
容器兼容: MP4格式
```

## 🛠️ 关键技术栈

| 层级 | 技术 | 用途 |
|------|------|------|
| **Web框架** | Axum | HTTP服务器 |
| **异步运行时** | Tokio | 异步处理 |
| **视频处理** | FFmpeg | 视频截图/剪辑 |
| **并发控制** | AtomicUsize | 请求计数 |
| **配置管理** | dotenv | 环境变量 |
| **日志系统** | tracing | 结构化日志 |
| **前端** | Vue.js | 用户界面 |

## ⚡ 快速定位代码

### 想要修改API接口？
👉 `src/api/handlers.rs` - 所有API处理函数

### 想要添加新的视频格式支持？ 
👉 `src/services/video/snapshot.rs` - 修改FFmpeg参数

### 想要改变服务器配置？
👉 `src/config.rs` - 配置管理
👉 `config.env.example` - 环境变量

### 想要添加新的通知方式？
👉 `src/services/notification/` - 新建通知服务

### 想要修改前端界面？
👉 `frontend/vue-project/src/` - Vue组件

## 🔍 代码阅读顺序建议

### 第一遍：理解架构 (10分钟)
1. `src/main.rs` - 看应用如何启动
2. `src/config.rs` - 理解配置系统  
3. `src/models/app_state.rs` - 了解应用状态

### 第二遍：核心业务 (20分钟)
1. `src/api/handlers.rs` - API接口逻辑
2. `src/services/video/snapshot.rs` - 视频处理核心
3. `src/api/middleware.rs` - 并发控制

### 第三遍：完整功能 (30分钟)
1. `src/services/video/stream_handler.rs` - 多协议支持
2. `src/utils/` - 工具函数
3. `src/services/notification/` - 通知服务

## 🧪 快速测试指南

```bash
# 1. 启动服务器
cargo run

# 2. 测试健康检查
curl http://localhost:3000/api/hello

# 3. 测试系统状态
curl http://localhost:3000/api/system-stats

# 4. 测试视频截图 (需要真实视频流)
curl -X POST http://localhost:3000/api/snapshot \
  -H "Content-Type: application/json" \
  -d '{"url":"rtsp://your-camera/stream","timestamp":10.0}'
```

## 🎨 设计模式应用

- **Builder模式**: AppBuilder构建应用
- **Service模式**: 业务逻辑分层
- **Middleware模式**: 请求拦截处理
- **Factory模式**: StreamHandler创建
- **Strategy模式**: 不同协议处理策略

## 📈 性能优化点

- **异步处理**: Tokio + Axum异步框架
- **并发控制**: AtomicUsize原子计数
- **资源管理**: 自动清理临时文件
- **错误处理**: Result类型安全处理
- **内存安全**: Rust所有权系统

## 🔧 扩展建议

1. **添加数据库**: 视频记录持久化
2. **缓存机制**: Redis缓存热点数据
3. **负载均衡**: 多实例部署
4. **监控告警**: Prometheus + Grafana
5. **API文档**: OpenAPI/Swagger集成 