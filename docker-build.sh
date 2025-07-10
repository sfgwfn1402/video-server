#!/bin/bash

# =============================================================================
# 🚀 Video Server Docker 构建脚本
# =============================================================================

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查Docker是否运行
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker未运行，请先启动Docker"
        exit 1
    fi
}

# 清理Docker空间
cleanup_docker() {
    print_info "清理Docker空间..."
    
    # 删除悬空镜像
    docker image prune -f
    
    # 删除未使用的容器
    docker container prune -f
    
    # 删除未使用的网络
    docker network prune -f
    
    # 删除未使用的卷
    docker volume prune -f
    
    # 删除构建缓存
    docker builder prune -f
    
    print_success "Docker空间清理完成"
}

# 显示Docker空间使用情况
show_docker_usage() {
    print_info "Docker空间使用情况："
    docker system df
}

# 构建镜像
build_image() {
    local dockerfile=${1:-"Dockerfile.optimized"}
    local tag=${2:-"video-server:latest"}
    
    print_info "开始构建镜像: $tag"
    print_info "使用Dockerfile: $dockerfile"
    
    # 检查Dockerfile是否存在
    if [ ! -f "$dockerfile" ]; then
        print_error "Dockerfile不存在: $dockerfile"
        exit 1
    fi
    
    # 构建镜像，使用BuildKit和缓存
    DOCKER_BUILDKIT=1 docker build \
        --file "$dockerfile" \
        --tag "$tag" \
        --build-arg BUILDKIT_INLINE_CACHE=1 \
        --progress=plain \
        .
    
    if [ $? -eq 0 ]; then
        print_success "镜像构建成功: $tag"
    else
        print_error "镜像构建失败"
        exit 1
    fi
}

# 运行容器
run_container() {
    local tag=${1:-"video-server:latest"}
    local port=${2:-"3000"}
    
    print_info "启动容器..."
    
    # 停止并删除已存在的容器
    docker stop video-server 2>/dev/null || true
    docker rm video-server 2>/dev/null || true
    
    # 运行新容器
    docker run -d \
        --name video-server \
        --restart unless-stopped \
        -p "$port:3000" \
        -v "$(pwd)/clips:/app/clips" \
        -e SERVER_PORT=3000 \
        -e CLIPS_DIR=clips \
        "$tag"
    
    if [ $? -eq 0 ]; then
        print_success "容器启动成功，端口: $port"
        print_info "容器状态:"
        docker ps | grep video-server
        print_info "容器日志:"
        sleep 2
        docker logs video-server
    else
        print_error "容器启动失败"
        exit 1
    fi
}

# 显示帮助信息
show_help() {
    echo "🚀 Video Server Docker 构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  build          构建镜像（默认使用Dockerfile.optimized）"
    echo "  run            运行容器"
    echo "  clean          清理Docker空间"
    echo "  usage          显示Docker空间使用情况"
    echo "  full           完整流程：清理+构建+运行"
    echo "  help           显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 build                    # 构建镜像"
    echo "  $0 run                      # 运行容器"
    echo "  $0 full                     # 完整流程"
    echo "  $0 clean                    # 清理空间"
}

# 主函数
main() {
    case "${1:-help}" in
        "build")
            check_docker
            show_docker_usage
            build_image
            ;;
        "run")
            check_docker
            run_container
            ;;
        "clean")
            check_docker
            cleanup_docker
            show_docker_usage
            ;;
        "usage")
            check_docker
            show_docker_usage
            ;;
        "full")
            check_docker
            print_info "开始完整构建流程..."
            cleanup_docker
            show_docker_usage
            build_image
            run_container
            print_success "完整流程执行完成！"
            ;;
        "help"|*)
            show_help
            ;;
    esac
}

# 执行主函数
main "$@" 