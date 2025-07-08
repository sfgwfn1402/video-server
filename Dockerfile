# ---------- 第一阶段：构建前端 ----------
FROM node:20-alpine AS frontend-build

WORKDIR /app/frontend

COPY frontend/vue-project/package*.json ./vue-project/
RUN cd vue-project && npm config set registry https://registry.npmmirror.com/ && npm install

COPY frontend/vue-project ./vue-project
RUN cd vue-project && npm run build

# ---------- 第二阶段：构建后端 ----------
FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /app

COPY . .

ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR="/usr/local/musl"

RUN cargo build --release

# ---------- 第三阶段：精简运行环境 ----------
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ffmpeg ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/video-server .
COPY --from=builder /app/frontend/vue-project/dist ./frontend/vue-project/dist
EXPOSE 3000
CMD ["./video-server"]