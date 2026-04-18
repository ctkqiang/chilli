# 小辣椒 Chilli

<div align="center">

**系统遥测与网络安全监控平台**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/Axum-0.7-blue.svg)](https://github.com/tokio-rs/axum)
[![SeaORM](https://img.shields.io/badge/SeaORM-0.12-green.svg)](https://www.sea-ql.org/SeaORM)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

---

## 项目简介

**小辣椒 (Chilli)** 是一个基于 Rust 构建的高性能系统遥测与网络安全监控平台。它集成了实时进程监控、GitHub 安全公告同步、系统状态采集等功能，为开发者和运维人员提供全面的系统可视化和安全态势感知能力。

### 核心特性

- **实时进程监控** - 采集系统进程信息、内存使用、端口监听状态
- **安全公告同步** - 自动同步 GitHub Security Advisory 数据库
- **系统遥测** - CPU、内存、运行时间等关键指标实时采集
- **RESTful API** - 基于 Axum 框架的高性能 HTTP API
- **多数据库支持** - SQLite / MySQL / PostgreSQL / QuestDB
- **异步高性能** - 基于 Tokio 异步运行时，资源占用低

---

## 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                        HTTP API Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  /health    │  │ /api/running│  │   /api/kill/:pid    │  │
│  │  健康检查   │  │  进程列表   │  │    终止进程         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Business Logic Layer                    │
│  ┌──────────────────┐  ┌────────────────────────────────┐  │
│  │  Process Monitor │  │  GitHub Advisory Sync          │  │
│  │  进程监控核心    │  │  安全公告同步引擎              │  │
│  └──────────────────┘  └────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Data Access Layer                       │
│         SeaORM (SQLite / MySQL / PostgreSQL)                │
└─────────────────────────────────────────────────────────────┘
```

---

## 快速开始

### 环境要求

- Rust 1.75+
- SQLite (默认) 或 MySQL/PostgreSQL

### 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/chilli.git
cd chilli

# 编译发布版本
cargo build --release

# 运行
cargo run
```

### 配置

创建 `.env` 文件：

```env
# 数据库配置 (SQLite - 默认)
DATABASE_URL=sqlite://./data/chilli.db

# 或 MySQL
# MYSQL_HOST=localhost
# MYSQL_USER=root
# MYSQL_PASSWORD=password
# MYSQL_DATABASE=chilli

# 或 PostgreSQL
# POSTGRES_HOST=localhost
# POSTGRES_USER=postgres
# POSTGRES_PASSWORD=password
# POSTGRES_DATABASE=chilli

```

---

## API 接口

### 系统状态

| 方法 | 路径      | 描述     |
| ---- | --------- | -------- |
| GET  | `/`       | 服务信息 |
| GET  | `/health` | 健康检查 |

### 进程管理

| 方法 | 路径             | 描述               |
| ---- | ---------------- | ------------------ |
| GET  | `/api/running`   | 获取运行中进程列表 |
| POST | `/api/kill/:pid` | 终止指定进程       |

### 安全公告

| 方法 | 路径                   | 描述               |
| ---- | ---------------------- | ------------------ |
| GET  | `/api/advisories`      | 获取同步的安全公告 |
| POST | `/api/advisories/sync` | 手动触发同步       |

---

## 测试

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration_tests

# 生成测试覆盖率报告
cargo tarpaulin --out Html
```

---

## 技术栈

| 组件                                     | 用途            |
| ---------------------------------------- | --------------- |
| [Axum](https://github.com/tokio-rs/axum) | Web 框架        |
| [Tokio](https://tokio.rs)                | 异步运行时      |
| [SeaORM](https://www.sea-ql.org/SeaORM)  | ORM 框架        |
| [Serde](https://serde.rs)                | 序列化/反序列化 |
| [sysinfo](https://docs.rs/sysinfo)       | 系统信息采集    |
| [listeners](https://docs.rs/listeners)   | 端口监听检测    |
| [reqwest](https://docs.rs/reqwest)       | HTTP 客户端     |

---
