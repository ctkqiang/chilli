# 小辣椒 Chilli Portal - 前端门户

基于 Vue 3 的现代化 Web 界面，为系统遥测与网络安全监控平台提供可视化展示。

## 功能特性

- **系统仪表盘** - 实时展示系统状态、运行进程、内存使用率
- **进程管理** - 查看、搜索、终止运行中的进程
- **安全扫描** - 漏洞检测、安全威胁分析
- **Docker 管理** - 容器状态监控和安全检测
- **多语言支持** - 中英文界面切换
- **实时更新** - WebSocket 支持实时数据推送

## 快速开始

```bash
docker run -d \
  --name chilli-portal \
  -p 3000:3000 \
  ctkqiang/chilli-portal:latest
```

访问: http://localhost:3000

## 环境变量

- 无特殊环境变量，自动在 localhost:3000 启动

## 技术栈

- **Vue 3** - 前端框架
- **TypeScript** - 类型系统
- **Vite** - 构建工具
- **Pinia** - 状态管理
- **Vue Router** - 路由
- **vue-i18n** - 国际化
- **Axios** - HTTP 客户端

## 与 Backend 配合使用

需要与后端 API 服务配合使用:

```bash
docker-compose up -d
```

或手动指定 API 地址环境变量进行配置。

## 许可证

MIT License - 详见 LICENSE 文件

## 项目链接

- GitHub: https://github.com/ctkqiang/chilli
- GitCode: https://gitcode.com/ctkqiang_sr/chilli.git
