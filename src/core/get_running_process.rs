use chrono::{DateTime, Utc};
use kill_tree::blocking::kill_tree;
use listeners::{get_all, Listener};
use std::ffi::OsString;
use sysinfo::System;

use crate::{
    models::{log_level::LogLevel, process_info::ProcessInfo, system_overview::SystemOverview},
    utils::{self, logger},
};

#[allow(dead_code)]
fn os_string_to_string(os: OsString) -> String {
    os.to_string_lossy().into_owned()
}

#[allow(dead_code)]
fn extract_port_from_socket(socket: &str) -> Option<u16> {
    socket.split(':').last()?.parse().ok()
}

/**
 * 采集系统运行状态与进程信息
 *
 * # 功能概述
 * 本函数通过系统API获取当前运行的所有进程信息，包括进程详情、内存使用、启动时间、
 * 监听端口等，并关联网络监听信息，构建完整的系统运行概览。
 *
 * # 数据采集架构
 * ```
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                     系统进程信息采集架构                                 │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │   ┌─────────────────┐     ┌─────────────────┐                          │
 * │   │   sysinfo::System│     │ listeners::get_all│                         │
 * │   │   系统信息API    │     │   网络监听API    │                          │
 * │   └────────┬────────┘     └────────┬────────┘                          │
 * │            │                       │                                   │
 * │            ▼                       ▼                                   │
 * │   ┌─────────────────┐     ┌─────────────────┐                          │
 * │   │ 进程列表         │     │ 监听端口映射     │                          │
 * │   │ (PID, Name,     │     │ (PID → Ports)   │                          │
 * │   │  Memory, etc)   │     │                 │                          │
 * │   └────────┬────────┘     └────────┬────────┘                          │
 * │            │                       │                                   │
 * │            └───────────┬───────────┘                                   │
 * │                        ▼                                               │
 * │            ┌─────────────────────┐                                     │
 * │            │   数据关联与转换     │                                     │
 * │            │  - 匹配PID与端口     │                                     │
 * │            │  - 转换数据类型      │                                     │
 * │            │  - 格式化时间        │                                     │
 * │            └───────────┬─────────┘                                     │
 * │                        ▼                                               │
 * │            ┌─────────────────────┐                                     │
 * │            │   SystemOverview    │                                     │
 * │            │   系统概览对象       │                                     │
 * │            └─────────────────────┘                                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 采集的数据项
 * ## 进程信息 (ProcessInfo)
 * - `pid`: 进程ID
 * - `name`: 进程名称
 * - `cmdline`: 启动命令行参数
 * - `memory_bytes`: 内存使用量（字节）
 * - `start_time`: 启动时间（RFC3339格式）
 * - `uptime_seconds`: 运行时长（秒）
 * - `listening_ports`: 监听的端口号列表
 *
 * ## 系统概览 (SystemOverview)
 * - `processes`: 所有进程信息列表
 * - `total_memory`: 系统总内存
 * - `used_memory`: 已使用内存
 * - `uptime_seconds`: 系统运行时长
 *
 * # 技术实现
 * ## 端口监听关联
 * 使用 `listeners` 库获取系统所有网络监听信息，通过PID将进程与端口关联：
 * ```
 * PID 1234 → [8080, 8443]  // 进程1234监听8080和8443端口
 * PID 5678 → [3306]        // 进程5678监听3306端口
 * ```
 *
 * ## 数据转换
 * - `OsString` → `String`: 命令行参数转换
 * - 时间戳 → RFC3339: 启动时间格式化
 * - Socket地址 → Port: 提取端口号
 *
 * # 返回值
 * 返回 `SystemOverview` 结构体，包含完整的系统和进程信息
 *
 * # 使用示例
 * ```rust
 * let overview = get_running_applicaitons();
 * println!("系统总内存: {} MB", overview.total_memory / 1024 / 1024);
 * println!("运行进程数: {}", overview.processes.len());
 *
 * for proc in &overview.processes {
 *     println!("PID: {}, Name: {}, Ports: {:?}",
 *         proc.pid, proc.name, proc.listening_ports);
 * }
 * ```
 */
#[allow(dead_code)]
pub fn get_running_applicaitons() -> SystemOverview {
    let mut sys = System::new_all();
    sys.refresh_all();

    let listeners: Vec<Listener> = match get_all() {
        Ok(l) => l.into_iter().collect(),
        Err(e) => {
            logger::log(LogLevel::Error, &format!("获取端口监听信息失败: {}", e));
            Vec::new()
        }
    };

    let listener_map: std::collections::HashMap<u32, Vec<u16>> = {
        let mut map = std::collections::HashMap::new();

        for listener in &listeners {
            let process = &listener.process;

            if let Some(port) = extract_port_from_socket(&listener.socket.to_string()) {
                map.entry(process.pid as u32)
                    .or_insert_with(Vec::new)
                    .push(port);
            }
        }
        map
    };

    let processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| {
            let ports = listener_map.get(&pid.as_u32()).cloned().unwrap_or_default();

            let start_time_str = DateTime::from_timestamp(proc.start_time() as i64, 0)
                .unwrap_or_else(|| Utc::now())
                .to_rfc3339();

            ProcessInfo {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cmdline: proc
                    .cmd()
                    .iter()
                    .map(|s| os_string_to_string(s.clone()))
                    .collect(),
                memory_bytes: proc.memory(),
                start_time: start_time_str,
                uptime_seconds: proc.run_time(),
                listening_ports: ports,
            }
        })
        .collect();

    logger::log(
        LogLevel::Debug,
        &format!("采集到 {} 个进程信息", processes.len()),
    );

    SystemOverview {
        processes,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        uptime_seconds: System::uptime(),
    }
}

/**
 * 终止指定PID的进程及其所有子进程
 *
 * # 功能概述
 * 本函数使用 `kill_tree` 库递归终止指定进程ID的进程及其整个进程树。
 * 与普通的 `kill` 命令不同，它会确保所有子进程也被正确终止，避免孤儿进程。
 *
 * # 进程树终止流程
 * ```
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                      进程树终止流程                                      │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │   ┌─────────────────┐                                                   │
 * │   │   输入 PID      │                                                   │
 * │   │   (目标进程)     │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │  kill_tree()    │  ──▶  发送终止信号给目标进程                       │
 * │   │  递归终止        │  ──▶  遍历所有子进程                              │
 * │   │                 │  ──▶  发送终止信号给子进程                         │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │   等待进程退出   │                                                   │
 * │   │   (优雅关闭)     │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │   返回结果      │                                                   │
 * │   │   Ok(()) / Err  │                                                   │
 * │   └─────────────────┘                                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 进程树示例
 * 假设要终止PID为1000的进程，其进程树结构如下：
 * ```
 * PID 1000 (目标进程)
 * ├── PID 1001 (子进程)
 * │   ├── PID 1003 (孙进程)
 * │   └── PID 1004 (孙进程)
 * └── PID 1002 (子进程)
 *     └── PID 1005 (孙进程)
 * ```
 * 调用 `kill_process_by_pid(1000)` 将终止所有6个进程。
 *
 * # 终止信号
 * 默认发送 `SIGTERM` (Unix) 或 `WM_CLOSE` (Windows) 信号，允许进程优雅关闭。
 * 如果进程不响应，可能需要结合超时机制强制终止。
 *
 * # 参数
 * - `pid`: 要终止的进程ID（根进程）
 *
 * # 返回值
 * - `Ok(())`: 进程树终止成功
 * - `Err(anyhow::Error)`: 可能的错误包括：
 *   - 进程不存在或已退出
 * *   - 权限不足（无法终止系统进程或其他用户进程）
 *   - 信号发送失败
 *   - 子进程终止超时
 *
 * # 使用示例
 * ```rust
 * use crate::core::get_running_process::kill_process_by_pid;
 *
 * fn main() -> anyhow::Result<()> {
 *     let target_pid = 12345;
 *
 *     match kill_process_by_pid(target_pid) {
 *         Ok(_) => println!("成功终止进程 {} 及其所有子进程", target_pid),
 *         Err(e) => eprintln!("终止进程失败: {}", e),
 *     }
 *
 *     Ok(())
 * }
 * ```
 *
 * # 与系统命令对比
 * | 方式 | 是否递归 | 是否跨平台 | 优雅关闭 |
 * |------|----------|------------|----------|
 * | `kill PID` | 否 | 是 | 是 |
 * | `kill -9 PID` | 否 | 是 | 否 |
 * | `pkill -P PID` | 是 | 否 | 是 |
 * | `kill_tree` | 是 | 是 | 是 |
 */
pub fn kill_process_by_pid(pid: u32) -> anyhow::Result<()> {
    utils::logger::log(LogLevel::Debug, &format!("正在终止进程树，根 PID: {}", pid));

    kill_tree(pid)?;

    utils::logger::log(LogLevel::Debug, &format!("成功终止进程树，根 PID: {}", pid));
    Ok(())
}
