use serde::Serialize;

use crate::models::process_info::ProcessInfo;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SystemOverview {
    pub processes: Vec<ProcessInfo>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uptime_seconds: u64,
}
