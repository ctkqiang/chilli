use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::path::Path;

const TARGET_PORTS: [u16; 3] = [3306, 5432, 6379];

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub src_ip: String,
    pub dst_port: u16,
    pub pid: u32,
    pub process_name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

struct RawTcpEntry {
    src_ip: String,
    dst_port: u16,
    inode: u64,
}

fn parse_hex_ip(hex: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let ip_int = u32::from_str_radix(hex, 16)
        .map_err(|e| format!("无效的IP十六进制格式 '{}': {}", hex, e))?;

    let ip = format!(
        "{}.{}.{}.{}",
        ip_int & 0xFF,
        (ip_int >> 8) & 0xFF,
        (ip_int >> 16) & 0xFF,
        (ip_int >> 24) & 0xFF,
    );

    Ok(ip)
}

fn parse_hex_port(hex: &str) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
    u16::from_str_radix(hex, 16)
        .map_err(|e| format!("无效的端口十六进制格式 '{}': {}", hex, e).into())
}

fn parse_proc_net_tcp(path: &str) -> Result<Vec<RawTcpEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let file = fs::File::open(path)
        .map_err(|e| format!("无法打开 {}: {}", path, e))?;

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.map_err(|e| format!("读取行失败: {}", e))?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 10 {
            continue;
        }

        let local_parts: Vec<&str> = parts[1].split(':').collect();
        let remote_parts: Vec<&str> = parts[2].split(':').collect();

        if local_parts.len() != 2 || remote_parts.len() != 2 {
            continue;
        }

        let src_ip = parse_hex_ip(local_parts[0])?;
        let _dst_ip = parse_hex_ip(remote_parts[0])?;
        let dst_port = parse_hex_port(remote_parts[1])?;

        if !TARGET_PORTS.contains(&dst_port) {
            continue;
        }

        let inode = parts[9]
            .parse::<u64>()
            .map_err(|e| format!("无效的inode值: {}", e))?;

        entries.push(RawTcpEntry {
            src_ip,
            dst_port,
            inode,
        });
    }

    Ok(entries)
}

fn build_inode_pid_map() -> Result<HashMap<u64, u32>, Box<dyn std::error::Error + Send + Sync>> {
    let mut inode_pid_map = HashMap::new();
    let proc_dir = Path::new("/proc");

    let entries = fs::read_dir(proc_dir)
        .map_err(|e| format!("无法读取 /proc 目录: {}", e))?;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let dir_name = entry.file_name();
        let dir_name_str = dir_name.to_string_lossy();

        let pid: u32 = match dir_name_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let fd_dir = entry.path().join("fd");
        let fd_entries = match fs::read_dir(&fd_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for fd_entry in fd_entries {
            let fd_entry = match fd_entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let link_target = match fs::read_link(fd_entry.path()) {
                Ok(t) => t,
                Err(_) => continue,
            };

            let link_str = link_target.to_string_lossy();

            if link_str.starts_with("socket:[") && link_str.ends_with(']') {
                let inode_str = &link_str[8..link_str.len() - 1];
                if let Ok(inode) = inode_str.parse::<u64>() {
                    inode_pid_map.insert(inode, pid);
                }
            }
        }
    }

    Ok(inode_pid_map)
}

fn get_process_name(pid: u32) -> String {
    let comm_path = format!("/proc/{}/comm", pid);
    match fs::read_to_string(&comm_path) {
        Ok(name) => name.trim().to_string(),
        Err(_) => "unknown".to_string(),
    }
}

fn get_connections_from_proc() -> Result<Vec<ConnectionInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let tcp4_entries = parse_proc_net_tcp("/proc/net/tcp")
        .unwrap_or_else(|_| Vec::new());

    let tcp6_entries = parse_proc_net_tcp("/proc/net/tcp6")
        .unwrap_or_else(|_| Vec::new());

    let inode_pid_map = build_inode_pid_map()?;
    let timestamp = Utc::now();

    let mut results = Vec::new();

    for entry in tcp4_entries.iter().chain(tcp6_entries.iter()) {
        let pid = match inode_pid_map.get(&entry.inode) {
            Some(&p) => p,
            None => continue,
        };

        let process_name = get_process_name(pid);

        results.push(ConnectionInfo {
            src_ip: entry.src_ip.clone(),
            dst_port: entry.dst_port,
            pid,
            process_name,
            timestamp,
        });
    }

    Ok(results)
}

fn get_connections_from_listeners() -> Result<Vec<ConnectionInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let listeners = listeners::get_all()
        .map_err(|e| format!("获取监听器列表失败: {}", e))?;

    let timestamp = Utc::now();
    let mut results = Vec::new();

    for listener in listeners {
        if listener.protocol != listeners::Protocol::TCP {
            continue;
        }

        let port = listener.socket.port();
        if !TARGET_PORTS.contains(&port) {
            continue;
        }

        let src_ip = socket_addr_to_ip_string(&listener.socket);

        results.push(ConnectionInfo {
            src_ip,
            dst_port: port,
            pid: listener.process.pid,
            process_name: listener.process.name.clone(),
            timestamp,
        });
    }

    Ok(results)
}

fn socket_addr_to_ip_string(addr: &SocketAddr) -> String {
    addr.ip().to_string()
}

pub fn get_active_connections() -> Result<Vec<ConnectionInfo>, Box<dyn std::error::Error + Send + Sync>> {
    if Path::new("/proc/net/tcp").exists() {
        get_connections_from_proc()
    } else {
        get_connections_from_listeners()
    }
}
