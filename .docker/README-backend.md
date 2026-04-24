# 小辣椒 Chilli - 系统遥测与网络安全监控平台

高性能系统遥测与网络安全监控平台，基于 Rust + Vue.js 构建。

## 核心特性

- **可视化仪表盘** - 基于 Vue 3 的现代化 Web 界面，实时展示系统状态
- **实时进程监控** - 采集系统进程信息、内存使用、端口监听状态
- **安全漏洞扫描** - 自动检测系统进程和服务的安全漏洞
- **Docker 安全管理** - 监控容器运行状态和安全配置
- **GitHub 安全公告同步** - 自动同步安全漏洞数据库
- **系统遥测** - CPU、内存、运行时间等关键指标实时采集
- **多语言支持** - 支持中文和英文界面切换
- **RESTful API** - 基于 Axum 框架的高性能 HTTP API
- **多数据库支持** - SQLite / MySQL / PostgreSQL

## 快速开始

### Backend API (端口 9333)

```bash
docker run -d \
  --name chilli-api \
  -p 9333:9333 \
  -v $(pwd)/data:/data \
  -e DATABASE_URL=mysql://user:password@db:3306/chilli \
  ctkqiang/chilli:latest
```

访问 API: http://localhost:9333/health

### Frontend Portal (端口 3000)

```bash
docker run -d \
  --name chilli-portal \
  -p 3000:3000 \
  ctkqiang/chilli-portal:latest
```

访问界面: http://localhost:3000

### 完整堆栈

```bash
docker-compose up -d
```

## 环境变量 (Backend)

- `PORT` - 服务端口 (默认: 9333)
- `DATABASE_URL` - 数据库连接字符串
- `RUST_LOG` - 日志级别 (info/debug/warn/error)
- `MYSQL_HOST` - MySQL 主机
- `MYSQL_USER` - MySQL 用户
- `MYSQL_PASSWORD` - MySQL 密码
- `MYSQL_DATABASE` - MySQL 数据库

## 技术栈

**后端**: Rust + Axum + SeaORM + Tokio  
**前端**: Vue 3 + TypeScript + Pinia + Vite  
**数据库**: MySQL / PostgreSQL / SQLite

## 许可证

MIT License - 详见 LICENSE 文件

## 项目链接

- GitHub: https://github.com/ctkqiang/chilli
- GitCode: https://gitcode.com/ctkqiang_sr/chilli.git
