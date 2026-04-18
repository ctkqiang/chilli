use chrono::{DateTime, Utc};
use listeners::{Listener, get_all};
use std::ffi::OsString;
use sysinfo::System;

use crate::{
    models::{log_level::LogLevel, process_info::ProcessInfo, system_overview::SystemOverview},
    utils::logger,
};

#[allow(dead_code)]
fn os_string_to_string(os: OsString) -> String {
    os.to_string_lossy().into_owned()
}

#[allow(dead_code)]
fn extract_port_from_socket(socket: &str) -> Option<u16> {
    socket.split(':').last()?.parse().ok()
}

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
