
# 拉取 amd64 架构的 node:20-alpine
docker pull --platform linux/amd64 node:20-alpine

docker pull --platform linux/amd64 clux/muslrust:stable

# 拉取 amd64 架构的 rust:1.77-slim
docker pull --platform linux/amd64 rust:1.77-slim

# 拉取 amd64 架构的 debian:bullseye-slim
docker pull --platform linux/amd64 debian:bullseye-slim

docker pull --platform linux/amd64 messense/rust-musl-cross:x86_64-musl

# 查看详情
docker inspect --format '{{.Architecture}}' node:20-alpine


docker save -o video-server.tar video-server:latest

# 在 Mac 上重新构建镜像，指定架构为 AMD64
docker build --platform linux/amd64 -t video-server:latest . --progress=plain 


docker load -i video-server.tar


docker run -d -p 3000:3000 -v /DATA03/video:/DATA03/video -v /DATA03/clips:/app/clips --name video-server video-server:latest