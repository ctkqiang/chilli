use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub memory_bytes: u64,
    pub start_time: String,
    pub uptime_seconds: u64,
    pub listening_ports: Vec<u16>,
}
