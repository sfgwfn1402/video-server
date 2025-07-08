#!/bin/bash
set -e

# 1. 构建前端
cd frontend/vue-project
npm install
npm run build

# 2. 构建后端
cd ../../
cargo build --release

# 3. 部署到服务器（请替换 user@server:/opt/video-server/ 为你的服务器信息）
# scp target/release/video-server user@server:/opt/video-server/
# scp -r frontend/vue-project/dist user@server:/opt/video-server/frontend/vue-project/

echo "本地构建完成，已生成可执行文件和前端静态资源。"
echo "请将 target/release/video-server 和 frontend/vue-project/dist 部署到服务器。"
