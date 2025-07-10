# ⚡ 5分钟快速入门指南

## 🎯 核心理念

这是一个 **Spring Boot 风格的 Rust 视频服务器**，支持多协议视频流处理。

## 📁 只需记住这些关键文件

```
src/
├── main.rs              # 🚀 启动入口 - 像Spring Boot一样简洁
├── config.rs            # ⚙️ 配置管理 - 环境变量驱动  
├── api/handlers.rs      # 📡 API接口 - 所有HTTP端点
└── services/video/snapshot.rs  # 🎥 核心业务 - FFmpeg视频处理
```

## 🔥 启动就这么简单

```bash
# 1. 编译运行
cargo run

# 2. 测试接口
curl http://localhost:3000/api/hello

# 3. 查看监控
curl http://localhost:3000/api/system-stats
```

## 🛠️ 核心功能一览

| API端点 | 功能 | 示例 |
|---------|------|------|
| `POST /api/snapshot` | 视频截图 | 支持 RTSP/RTMP/HLS/HTTP |
| `POST /api/clip` | 视频剪辑 | 指定时间段剪切 |
| `GET /api/system-stats` | 系统监控 | CPU/内存/并发数 |

## ⚙️ 配置超简单

创建 `.env` 文件：
```bash
SERVER_PORT=3000
CLIPS_DIR=clips
FRONTEND_DIR=frontend/vue-project/dist
```

## 🎥 视频处理核心

**智能协议识别**：
- `rtsp://` → RTSP协议处理
- `rtmp://` → RTMP协议处理  
- `http://*.m3u8` → HLS协议处理
- `http://*.mp4` → HTTP协议处理

**特殊优化**：
- `realmonitor` 摄像头 → 简化参数
- 自动音频转码：`pcm_alaw` → `aac`

## 🔍 代码结构速记

```
📡 API层      → 接收HTTP请求
📊 Models层   → 数据结构定义  
🎥 Service层  → 业务逻辑处理
🔧 Utils层    → 工具函数
⚙️ Config层   → 配置管理
```

## 🚀 扩展开发

**添加新API**：
1. 在 `src/api/handlers.rs` 添加处理函数
2. 在 `src/config.rs` 注册路由

**修改视频处理**：
1. 编辑 `src/services/video/snapshot.rs`
2. 调整 FFmpeg 参数

**更改配置**：
1. 修改 `src/config.rs` 中的 `AppConfig`
2. 添加对应的环境变量支持

## 🎮 交互式导览

```bash
# 运行代码导览工具
python3 code_tour.py
```

- **0** - 查看项目结构
- **1** - 应用启动流程  
- **2** - 配置系统
- **3** - API处理逻辑
- **a** - 完整架构文档

## 💡 设计亮点

- **🌟 Spring Boot风格启动**：一行代码启动应用
- **🔧 Builder模式配置**：链式调用，优雅配置
- **📦 模块化架构**：清晰分层，职责分离
- **⚡ 异步高性能**：Tokio + Axum 异步框架
- **🛡️ 类型安全**：Rust 编译时错误检查
- **🎯 智能处理**：自动协议识别和参数优化

---

**🎉 恭喜！你现在已经掌握了项目的核心架构！**

需要深入了解某个模块？查看 `ARCHITECTURE.md` 详细文档。 