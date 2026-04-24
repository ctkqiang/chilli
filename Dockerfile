# 小辣椒 Chilli - 系统遥测与网络安全监控平台
# 多阶段构建 Dockerfile

# =============================================================================
# 阶段 1: 构建后端
# =============================================================================
FROM rust:1.88-alpine AS backend-builder

# 安装构建依赖
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml ./
COPY Cargo.lock* ./

# 创建虚拟 main.rs 以缓存依赖
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制后端源码
COPY src ./src

# 创建前端资源目录
RUN mkdir -p /app/portal

# 构建后端
RUN cargo build --release

# =============================================================================
# 阶段 2: 运行环境
# =============================================================================
FROM alpine:3.19

# 安装运行时依赖（CA 证书用于 HTTPS 请求）
RUN apk add --no-cache ca-certificates wget

# 标签信息
LABEL maintainer="ctkqiang"
LABEL description="小辣椒 Chilli - 系统遥测与网络安全监控平台"
LABEL version="0.0.1"

# 创建非 root 用户
RUN adduser -D -s /bin/sh chilli

# 从构建阶段复制二进制文件
COPY --from=backend-builder /app/target/release/chilli /usr/local/bin/chilli

# 创建前端资源目录
RUN mkdir -p /app/portal

# 设置权限
RUN chmod +x /usr/local/bin/chilli

# 创建数据目录
RUN mkdir -p /data /app/portal && chown -R chilli:chilli /data /app/portal

# 切换到非 root 用户
USER chilli

# 数据目录
VOLUME ["/data"]

# 暴露端口（后端 API）
EXPOSE 9333

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:9333/health || exit 1

# 运行应用
ENTRYPOINT ["/usr/local/bin/chilli"]
