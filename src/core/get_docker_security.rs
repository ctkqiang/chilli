use crate::models::log_level::LogLevel;
use crate::utils;
use serde::{Deserialize, Serialize};
use std::process::Command;

/**
 * Docker 容器安全扫描模块
 *
 * # 功能概述
 * 扫描 Docker 容器的安全配置，检测潜在的安全风险和漏洞。
 *
 * # 检测项目
 * - 特权模式容器
 * - 敏感目录挂载
 * - 容器镜像漏洞
 * - 不安全的网络配置
 * - 资源限制缺失
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSecurityIssue {
    pub container_id: String,
    pub container_name: String,
    pub issue_type: IssueType,
    pub severity: Severity,
    pub description: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    PrivilegedMode,
    SensitiveMount,
    NoResourceLimits,
    RootUser,
    InsecureCapability,
    ExposedDockerSocket,
    HostNetwork,
    HostPid,
    WritableRootfs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::High => write!(f, "high"),
            Severity::Medium => write!(f, "medium"),
            Severity::Low => write!(f, "low"),
        }
    }
}

/**
 * 扫描所有 Docker 容器的安全问题
 */
pub async fn scan_docker_containers() -> Vec<ContainerSecurityIssue> {
    let mut issues = Vec::new();

    // 检查 Docker 是否可用
    if !is_docker_available() {
        utils::logger::log(LogLevel::Warn, "Docker 不可用，跳过容器安全扫描");
        return issues;
    }

    // 获取所有运行中的容器
    let containers = match get_running_containers() {
        Ok(c) => c,
        Err(e) => {
            utils::logger::log(LogLevel::Error, &format!("获取容器列表失败: {}", e));
            return issues;
        }
    };

    for container in containers {
        // 检查特权模式
        if let Some(issue) = check_privileged_mode(&container) {
            issues.push(issue);
        }

        // 检查敏感目录挂载
        if let Some(issue) = check_sensitive_mounts(&container) {
            issues.push(issue);
        }

        // 检查 root 用户运行
        if let Some(issue) = check_root_user(&container) {
            issues.push(issue);
        }

        // 检查资源限制
        if let Some(issue) = check_resource_limits(&container) {
            issues.push(issue);
        }

        // 检查网络模式
        if let Some(issue) = check_network_mode(&container) {
            issues.push(issue);
        }

        // 检查 PID 命名空间
        if let Some(issue) = check_pid_mode(&container) {
            issues.push(issue);
        }

        // 检查可写 rootfs
        if let Some(issue) = check_writable_rootfs(&container) {
            issues.push(issue);
        }
    }

    utils::logger::log(
        LogLevel::Info,
        &format!("Docker 安全扫描完成，发现 {} 个问题", issues.len()),
    );

    issues
}

/**
 * 检查 Docker 是否可用
 */
fn is_docker_available() -> bool {
    Command::new("docker")
        .args(["version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Debug, Clone)]
struct ContainerInfo {
    id: String,
    name: String,
    image: String,
}

/**
 * 获取运行中的容器列表
 */
fn get_running_containers() -> Result<Vec<ContainerInfo>, Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.ID}}|{{.Names}}|{{.Image}}"])
        .output()?;

    if !output.status.success() {
        return Err("获取容器列表失败".into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let containers: Vec<ContainerInfo> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                Some(ContainerInfo {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(containers)
}

/**
 * 获取容器详细信息（JSON 格式）
 */
fn get_container_inspect(
    container_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .args(["inspect", container_id])
        .output()?;

    if !output.status.success() {
        return Err(format!("获取容器 {} 详情失败", container_id).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let inspect: Vec<serde_json::Value> = serde_json::from_str(&stdout)?;

    inspect
        .into_iter()
        .next()
        .ok_or_else(|| "容器详情为空".into())
}

/**
 * 检查容器是否以特权模式运行
 */
fn check_privileged_mode(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let privileged = inspect
        .get("HostConfig")?
        .get("Privileged")?
        .as_bool()
        .unwrap_or(false);

    if privileged {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::PrivilegedMode,
            severity: Severity::Critical,
            description: format!(
                "容器 '{}' 以特权模式运行，拥有主机的完全访问权限",
                container.name
            ),
            remediation: "移除 --privileged 标志，使用细粒度的 capabilities 替代".to_string(),
        })
    } else {
        None
    }
}

/**
 * 检查容器是否挂载了敏感目录
 */
fn check_sensitive_mounts(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let mounts = inspect.get("HostConfig")?.get("Binds")?.as_array()?;

    let sensitive_paths = [
        "/var/run/docker.sock",
        "/",
        "/etc",
        "/root",
        "/var",
        "/proc",
        "/sys",
    ];

    for mount in mounts {
        let mount_str = mount.as_str().unwrap_or("");
        for sensitive in &sensitive_paths {
            if mount_str.contains(sensitive) {
                return Some(ContainerSecurityIssue {
                    container_id: container.id.clone(),
                    container_name: container.name.clone(),
                    issue_type: IssueType::SensitiveMount,
                    severity: Severity::High,
                    description: format!("容器 '{}' 挂载了敏感目录: {}", container.name, mount_str),
                    remediation: format!("移除对 {} 的挂载，使用最小权限原则", sensitive),
                });
            }
        }
    }

    // 检查是否挂载了 Docker socket
    let mount_source = inspect.get("Mounts")?.as_array()?;

    for mount in mount_source {
        let source = mount.get("Source")?.as_str()?;
        if source.contains("docker.sock") {
            return Some(ContainerSecurityIssue {
                container_id: container.id.clone(),
                container_name: container.name.clone(),
                issue_type: IssueType::ExposedDockerSocket,
                severity: Severity::Critical,
                description: format!(
                    "容器 '{}' 挂载了 Docker socket，可能导致容器逃逸",
                    container.name
                ),
                remediation: "移除 docker.sock 挂载，使用 Docker API 代理或限制权限".to_string(),
            });
        }
    }

    None
}

/**
 * 检查容器是否以 root 用户运行
 */
fn check_root_user(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let user = inspect.get("Config")?.get("User")?.as_str().unwrap_or("");

    // 如果 User 为空或为 "root"，则认为是以 root 运行
    if user.is_empty() || user == "root" {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::RootUser,
            severity: Severity::Medium,
            description: format!(
                "容器 '{}' 以 root 用户运行，存在权限提升风险",
                container.name
            ),
            remediation: "在 Dockerfile 中使用 USER 指令指定非 root 用户".to_string(),
        })
    } else {
        None
    }
}

/**
 * 检查容器是否设置了资源限制
 */
fn check_resource_limits(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let host_config = inspect.get("HostConfig")?;

    let memory_limit = host_config.get("Memory")?.as_i64().unwrap_or(0);

    let cpu_quota = host_config.get("CpuQuota")?.as_i64().unwrap_or(0);

    // 如果没有设置内存限制或 CPU 限制
    if memory_limit == 0 && cpu_quota == 0 {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::NoResourceLimits,
            severity: Severity::Low,
            description: format!(
                "容器 '{}' 没有设置资源限制（内存/CPU），可能导致资源耗尽攻击",
                container.name
            ),
            remediation: "使用 --memory 和 --cpus 参数设置资源限制".to_string(),
        })
    } else {
        None
    }
}

/**
 * 检查容器是否使用 host 网络模式
 */
fn check_network_mode(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let network_mode = inspect
        .get("HostConfig")?
        .get("NetworkMode")?
        .as_str()
        .unwrap_or("");

    if network_mode == "host" {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::HostNetwork,
            severity: Severity::High,
            description: format!(
                "容器 '{}' 使用 host 网络模式，共享主机网络命名空间",
                container.name
            ),
            remediation: "使用 bridge 网络模式或自定义网络，避免使用 --network host".to_string(),
        })
    } else {
        None
    }
}

/**
 * 检查容器是否使用 host PID 命名空间
 */
fn check_pid_mode(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let pid_mode = inspect
        .get("HostConfig")?
        .get("PidMode")?
        .as_str()
        .unwrap_or("");

    if pid_mode == "host" {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::HostPid,
            severity: Severity::High,
            description: format!(
                "容器 '{}' 使用 host PID 命名空间，可以查看和操纵主机进程",
                container.name
            ),
            remediation: "移除 --pid host 参数，使用默认的 PID 命名空间隔离".to_string(),
        })
    } else {
        None
    }
}

/**
 * 检查容器是否使用可写的 rootfs
 */
fn check_writable_rootfs(container: &ContainerInfo) -> Option<ContainerSecurityIssue> {
    let inspect = get_container_inspect(&container.id).ok()?;

    let readonly_rootfs = inspect
        .get("HostConfig")?
        .get("ReadonlyRootfs")?
        .as_bool()
        .unwrap_or(false);

    if !readonly_rootfs {
        Some(ContainerSecurityIssue {
            container_id: container.id.clone(),
            container_name: container.name.clone(),
            issue_type: IssueType::WritableRootfs,
            severity: Severity::Low,
            description: format!("容器 '{}' 的文件系统可写，可能被篡改", container.name),
            remediation: "使用 --read-only 参数使 rootfs 只读，配合 tmpfs 挂载可写目录".to_string(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Critical.to_string(), "critical");
        assert_eq!(Severity::High.to_string(), "high");
        assert_eq!(Severity::Medium.to_string(), "medium");
        assert_eq!(Severity::Low.to_string(), "low");
    }
}
