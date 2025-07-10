# 🎥 Video Server

一个基于 Rust 和 Vue.js 的现代化视频流处理服务器，支持多种视频协议的截图和剪辑功能。

## ✨ 功能特性

### 🔧 核心功能
- **多协议支持**: RTSP、RTMP、HLS、HTTP 视频流
- **视频截图**: 支持指定时间戳的高质量截图
- **视频剪辑**: 支持指定时间段的视频片段提取
- **实时监控**: 系统资源和并发请求监控
- **通知推送**: 飞书 Webhook 通知集成

### 🎨 用户界面
- **现代化设计**: Apple 风格的玻璃态效果
- **实时监控面板**: CPU、内存使用率可视化
- **响应式布局**: 适配各种屏幕尺寸
- **直观操作**: 简单易用的视频处理界面

### 🚀 性能特性
- **高并发**: 异步处理，支持多请求并发
- **智能编码**: 根据协议自动选择最优编码策略
- **资源监控**: 实时系统资源使用统计
- **错误处理**: 完善的错误处理和恢复机制

## 🏗️ 项目架构

### 后端架构 (Rust)

```
src/
├── main.rs                      # 应用入口
├── api/                         # API 层
│   ├── handlers.rs              # API 处理函数
│   ├── middleware.rs            # 中间件
│   └── mod.rs                   # 模块声明
├── models/                      # 数据模型层
│   ├── app_state.rs             # 应用状态
│   ├── requests.rs              # 请求结构体
│   ├── responses.rs             # 响应结构体
│   └── mod.rs                   # 模块声明
├── services/                    # 业务服务层
│   ├── video/                   # 视频处理服务
│   │   ├── snapshot.rs          # 截图服务
│   │   ├── stream_handler.rs    # 流处理器
│   │   └── mod.rs               # 模块声明
│   ├── notification/            # 通知服务
│   │   ├── feishu.rs            # 飞书通知
│   │   └── mod.rs               # 模块声明
│   └── mod.rs                   # 服务模块声明
└── utils/                       # 工具函数层
    ├── image.rs                 # 图片处理工具
    ├── system.rs                # 系统工具
    └── mod.rs                   # 工具模块声明
```

### 前端架构 (Vue.js)

```
frontend/vue-project/
├── src/
│   ├── App.vue                  # 主应用组件
│   ├── components/
│   │   └── VideoClipTool.vue    # 视频剪辑工具组件
│   ├── assets/                  # 静态资源
│   └── main.js                  # 应用入口
├── public/                      # 公共文件
└── dist/                        # 构建输出
```

## 🚀 快速开始

### 环境要求

- **Rust**: >= 1.75.0
- **Node.js**: >= 18.0.0
- **FFmpeg**: >= 4.0.0

### 安装依赖

#### 安装 FFmpeg

**macOS:**
```bash
brew install ffmpeg
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install ffmpeg
```

**Windows:**
```bash
# 使用 Chocolatey
choco install ffmpeg
```

#### 克隆项目
```bash
git clone <repository-url>
cd video-server
```

#### 安装 Rust 依赖
```bash
cargo build
```

#### 安装前端依赖
```bash
cd frontend/vue-project
npm install
npm run build
cd ../..
```

### 配置环境变量

创建 `.env` 文件（可选）：

```bash
# 飞书 Webhook URL（可选）
FEISHU_WEBHOOK_URL=https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-url
```

### 启动服务

```bash
cargo run
```

服务将在 `http://localhost:3000` 启动。

## 📖 API 文档

### 基础信息

- **基础URL**: `http://localhost:3000`
- **数据格式**: JSON
- **请求方式**: POST/GET

### API 端点

#### 1. 视频截图

**端点**: `POST /api/snapshot`

**请求体**:
```json
{
  "url": "rtsp://admin:password@192.168.1.100/cam/realmonitor?channel=1&subtype=0",
  "timestamp": 5.0
}
```

**参数说明**:
- `url`: 视频流地址（支持 RTSP、RTMP、HLS、HTTP）
- `timestamp`: 截图时间戳（秒），可选，默认为 0

**响应**: 返回 PNG 图片二进制数据

#### 2. 视频剪辑

**端点**: `POST /api/clip`

**请求体**:
```json
{
  "url": "rtsp://admin:password@192.168.1.100/cam/realmonitor?channel=1&subtype=0",
  "start": 10.0,
  "duration": 30.0,
  "return_url": true
}
```

**参数说明**:
- `url`: 视频流地址
- `start`: 开始时间（秒），可选，默认为 0
- `duration`: 持续时长（秒）
- `return_url`: 是否返回文件URL，可选，默认为 true

**响应**:
```json
{
  "video_url": "/clips/filename.mp4"
}
```

#### 3. 并发请求统计

**端点**: `GET /api/concurrent`

**响应**:
```json
{
  "current_requests": 3,
  "message": "当前正在处理 3 个并发请求"
}
```

#### 4. 系统监控统计

**端点**: `GET /api/system-stats`

**响应**:
```json
{
  "cpu_usage": 45.2,
  "memory_usage": 68.5,
  "memory_total": 16777216000,
  "memory_used": 11489740800,
  "current_requests": 2,
  "uptime": 0
}
```

## 🎯 支持的视频格式

### 输入格式支持

| 协议/格式 | 描述 | 示例URL |
|-----------|------|---------|
| **RTSP** | 实时流协议 | `rtsp://admin:pass@192.168.1.100/stream` |
| **RTMP** | 实时消息协议 | `rtmp://live.example.com/live/stream` |
| **HLS** | HTTP 直播流 | `https://example.com/live/stream.m3u8` |
| **HTTP** | HTTP 视频文件 | `https://example.com/video.mp4` |
| **本地文件** | 本地视频文件 | `/path/to/video.mp4` |

### 输出格式

- **截图**: PNG 格式，高质量输出
- **视频**: MP4 格式，H.264 编码，AAC 音频

## 🔧 部署指南

### Docker 部署

#### 构建镜像
```bash
docker build -t video-server .
```

#### 运行容器
```bash
docker run -d \
  --name video-server \
  -p 3000:3000 \
  -v $(pwd)/clips:/app/clips \
  -e FEISHU_WEBHOOK_URL=your-webhook-url \
  video-server
```

### 生产环境部署

#### 1. 编译优化版本
```bash
cargo build --release
```

#### 2. 构建前端
```bash
cd frontend/vue-project
npm run build
cd ../..
```

#### 3. 启动服务
```bash
./target/release/video-server
```

#### 4. 使用进程管理器（推荐）
```bash
# 使用 systemd
sudo systemctl enable video-server
sudo systemctl start video-server

# 或使用 PM2
pm2 start ./target/release/video-server --name video-server
```

## ⚙️ 配置说明

### FFmpeg 参数优化

不同协议使用不同的 FFmpeg 参数：

#### RTSP 流
```bash
-rtsp_transport tcp -analyzeduration 5000000 -probesize 5000000
```

#### RTMP 流
```bash
-timeout 10000000 -analyzeduration 2000000 -probesize 2000000
```

#### HLS 流
```bash
-timeout 10000000 -user_agent "Mozilla/5.0 (compatible; VideoServer/1.0)"
```

### 音频处理

- 自动将 `pcm_alaw` 等格式转换为 `AAC`
- 音频比特率设置为 128kbps
- 支持无音频的视频流

## 🧪 开发指南

### 本地开发

#### 启动开发服务器
```bash
# 后端
cargo run

# 前端（新终端）
cd frontend/vue-project
npm run dev
```

#### 代码格式化
```bash
cargo fmt
```

#### 运行测试
```bash
cargo test
```

### 项目结构说明

- **分层架构**: API → Services → Utils
- **模块化设计**: 功能按模块组织
- **依赖注入**: 通过 AppState 管理状态
- **错误处理**: 完善的错误类型和处理

### 添加新功能

1. **API 层**: 在 `src/api/handlers.rs` 添加新的处理函数
2. **服务层**: 在 `src/services/` 添加业务逻辑
3. **模型层**: 在 `src/models/` 定义数据结构
4. **前端**: 在 `frontend/vue-project/src/` 添加组件

## 🐛 故障排除

### 常见问题

#### 1. FFmpeg 相关错误

**问题**: `Option reconnect not found`
**解决**: 更新 FFmpeg 到最新版本，或使用项目中的兼容参数

**问题**: `Unrecognized option 'stimeout'`
**解决**: 项目已自动处理，使用标准的 timeout 参数

#### 2. RTSP 连接问题

**问题**: 连接超时
**解决**: 
- 检查网络连接
- 确认摄像头地址和认证信息
- 使用 TCP 传输协议

#### 3. 音频编码问题

**问题**: `Could not find tag for codec pcm_alaw`
**解决**: 项目自动将音频重新编码为 AAC 格式

### 调试技巧

#### 启用详细日志
```bash
RUST_LOG=debug cargo run
```

#### 检查 FFmpeg 命令
查看日志中的 FFmpeg 命令行参数，手动执行测试

## 📊 性能优化

### 建议配置

- **并发限制**: 根据服务器性能调整
- **内存管理**: 定期清理临时文件
- **网络优化**: 使用 CDN 分发静态文件

### 监控指标

- CPU 使用率
- 内存使用率
- 并发请求数
- 错误率统计

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📞 联系方式

- 项目链接: [https://github.com/your-username/video-server](https://github.com/your-username/video-server)
- 问题反馈: [Issues](https://github.com/your-username/video-server/issues)

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 现代化的 Rust Web 框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [FFmpeg](https://ffmpeg.org/) - 多媒体处理工具
- [Tokio](https://tokio.rs/) - 异步运行时

---

⭐ 如果这个项目对你有帮助，请给个 Star！ 