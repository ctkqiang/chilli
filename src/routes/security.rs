use crate::core::get_docker_security::scan_docker_containers;
use crate::core::get_running_process::get_running_applicaitons;
use crate::core::get_security::check_vulnerability_for_app;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::json;
use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
struct DetectedService {
    port: u16,
    service_type: String,
    version: Option<String>,
    vulnerabilities: Vec<String>,
}

/// 扫描漏洞的并发限制
const MAX_CONCURRENT_SCANS: usize = 5;

#[allow(unused)]
pub async fn scan_vulnerabilities(
    Extension(_db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let system_overview = get_running_applicaitons();
    let mut all_issues = Vec::new();
    let mut detected_services = Vec::new();
    let mut scanned_ports = std::collections::HashSet::new();

    // 获取本机 IP
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    // 创建并发限制器
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_SCANS));

    // 收集所有需要扫描的端口和对应的进程名
    let mut scan_targets: Vec<(u16, String)> = Vec::new();

    for process in &system_overview.processes {
        for port in &process.listening_ports {
            if scanned_ports.contains(port) {
                continue;
            }
            scanned_ports.insert(*port);
            scan_targets.push((*port, process.name.clone()));
        }
    }

    // 首先扫描所有检测到的进程端口
    let mut scan_tasks = Vec::new();

    for (port, app_name) in scan_targets {
        let local_ip = local_ip.clone();
        let sem = semaphore.clone();

        let task = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            // 对特定端口进行主动探测
            let service = probe_service(&local_ip, port).await;

            // 检查特定漏洞
            let specific_vuln = check_specific_vulnerability(&service).await;

            // 常规漏洞扫描
            let ecosystem = detect_ecosystem(&port);
            let vuln_result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                check_vulnerability_for_app(&app_name, "unknown", &ecosystem)
            ).await.ok().and_then(|r| r.ok());

            (service, specific_vuln, vuln_result)
        });

        scan_tasks.push(task);
    }

    // 等待所有扫描任务完成
    for task in scan_tasks {
        if let Ok((service, specific_vuln, vuln_issues)) = task.await {
            detected_services.push(service.clone());

            if let Some(vuln) = specific_vuln {
                all_issues.push(json!({
                    "service": service.service_type,
                    "port": service.port,
                    "vulnerability": vuln,
                    "severity": "high"
                }));
            }

            if let Some(issues) = vuln_issues {
                all_issues.extend(issues.iter().map(|i| {
                    json!({
                        "service": &service.service_type,
                        "port": service.port,
                        "severity": &i.severity,
                        "summary": &i.summary,
                        "ghsa_id": &i.ghsa_id,
                        "cve_id": &i.cve_id
                    })
                }));
            }
        }
    }

    // 主动扫描常见漏洞端口（包括 Docker 容器可能映射的端口）
    let common_vuln_ports: Vec<u16> = vec![
        9000, 9001, 8812, 9009, 8822, 8123, 8080, 3000, 5000, 8000,
    ];

    let mut port_tasks = Vec::new();

    for port in common_vuln_ports {
        if scanned_ports.contains(&port) {
            continue;
        }

        let local_ip = local_ip.clone();
        let sem = semaphore.clone();

        let task = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let service = probe_service(&local_ip, port).await;

            if service.service_type != "unknown" {
                let vuln = check_specific_vulnerability(&service).await;
                Some((service, vuln))
            } else {
                None
            }
        });

        port_tasks.push(task);
    }

    for task in port_tasks {
        if let Ok(Some((service, vuln))) = task.await {
            detected_services.push(service.clone());

            if let Some(v) = vuln {
                all_issues.push(json!({
                    "service": service.service_type,
                    "port": service.port,
                    "vulnerability": v,
                    "severity": "high"
                }));
            }
        }
    }

    let response = json!({
        "total_processes": system_overview.processes.len(),
        "vulnerabilities_found": all_issues.len(),
        "detected_services": detected_services,
        "issues": all_issues
    });

    (StatusCode::OK, Json(response)).into_response()
}

async fn probe_service(ip: &str, port: u16) -> DetectedService {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    // QuestDB 特定端口列表
    let questdb_ports = [8812u16, 9001, 9003, 9002, 9009, 8822, 9000];

    // 如果是 QuestDB 端口，先尝试 QuestDB 特定探测
    if questdb_ports.contains(&port) {
        let url = format!(
            "http://{}:{}/exec?query=select%20*%20from%20tables();",
            ip, port
        );

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let text = response.text().await.unwrap_or_default();
                    // 检查是否是 QuestDB 响应
                    if text.contains("dataset")
                        || text.contains("columns")
                        || text.contains("timestamp")
                        || text.contains("query")
                        || text.contains("count")
                    {
                        return DetectedService {
                            port,
                            service_type: "questdb".to_string(),
                            version: None,
                            vulnerabilities: vec!["CNVD-2026-84827".to_string()],
                        };
                    }
                }
            }
            Err(_) => {}
        }

        // 即使 HTTP 探测失败，如果是 QuestDB 端口，也标记为可能存在漏洞
        return DetectedService {
            port,
            service_type: "questdb".to_string(),
            version: None,
            vulnerabilities: vec!["CNVD-2026-84827 (待确认)".to_string()],
        };
    }

    // 尝试健康检查端点
    let health_url = format!("http://{}:{}/health", ip, port);
    match client.get(&health_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                return DetectedService {
                    port,
                    service_type: "generic_http".to_string(),
                    version: None,
                    vulnerabilities: vec![],
                };
            }
        }
        Err(_) => {}
    }

    // 默认返回基于端口的猜测
    DetectedService {
        port,
        service_type: detect_ecosystem(&port),
        version: None,
        vulnerabilities: vec![],
    }
}

async fn check_specific_vulnerability(service: &DetectedService) -> Option<String> {
    if !service.vulnerabilities.is_empty() {
        return Some(service.vulnerabilities[0].clone());
    }

    // QuestDB CNVD-2026-84827 检测
    if service.service_type == "questdb" {
        return Some("CNVD-2026-84827 - QuestDB 远程代码执行漏洞".to_string());
    }
    None
}

fn get_local_ip() -> Option<String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let addr = socket.local_addr().ok()?;
    Some(addr.ip().to_string())
}

fn detect_ecosystem(port: &u16) -> String {
    match port {
        // Infrastructure
        21 => "ftp".to_string(),
        22 => "ssh".to_string(),
        23 => "telnet".to_string(),
        25 => "smtp".to_string(),
        53 => "dns".to_string(),
        80 | 443 => "http/https".to_string(),

        // Node.js / npm
        3000 | 5173 | 4200 | 3001 | 3005 | 4000 | 1234 | 5001 => "npm/node/vite".to_string(),

        // Python / pip
        8000 | 8501 | 8001 | 8008 | 8888 | 5005 => "pip/python/jupyter".to_string(),

        // ClickHouse
        8123 | 9004 | 9440 => "clickhouse".to_string(),

        // QuestDB
        8812 | 9001 | 9003 | 9002 | 9009 | 8822 => "questdb".to_string(),

        // MySQL
        3306 | 33060 | 33061 | 4567 | 4444 => "mysql/galera".to_string(),

        // PostgreSQL
        5432 | 5433 | 5434 | 6432 | 9999 => "postgresql/pgbouncer".to_string(),

        // Redis
        6379 | 16379 | 26379 | 6380 | 6381 | 16380 => "redis/sentinel".to_string(),

        // MongoDB
        27017 | 27018 | 27019 | 28017 => "mongodb".to_string(),

        // Redis Cluster
        7000 | 7001 | 7002 | 7003 | 7004 | 7005 | 7006 => "redis-cluster".to_string(),

        // Kafka / Zookeeper
        9092 | 9093 | 9094 | 2181 | 2182 | 2183 | 9091 => "kafka/zookeeper".to_string(),

        // Java / Maven
        8443 | 8010 | 8005 => "maven/java/fpm".to_string(),

        // Docker / Swarm / etcd
        2375 | 2376 | 2377 | 7946 | 4789 | 2379 | 2380 => "docker/swarm/etcd".to_string(),

        // Prometheus / Grafana
        9090 | 9100 | 9115 | 9104 => "prometheus/grafana".to_string(),

        // Portainer / MinIO
        9443 | 9000 => "portainer/minio".to_string(),

        // Webmin / cPanel
        10000 | 20000 => "webmin/usermin/cpanel".to_string(),

        // MQTT
        1883 | 8883 | 1884 | 8884 | 18083 => "mqtt/mosquitto/emqx".to_string(),

        // NATS
        4222 | 6222 | 8222 | 7422 | 4223 => "nats/leaf".to_string(),

        // RabbitMQ
        5672 | 15672 | 25672 | 4369 => "rabbitmq".to_string(),

        // MSSQL
        1433 | 1434 => "mssql".to_string(),

        // Oracle
        1521 | 1821 | 2483 | 2484 => "oracle".to_string(),

        // Elasticsearch
        9200 | 9300 | 9600 | 9700 => "elasticsearch/logstash/opensearch".to_string(),

        // Kibana
        5601 => "kibana".to_string(),

        // SNMP
        161 | 162 => "snmp".to_string(),

        // LDAP
        389 | 636 => "ldap".to_string(),

        // rsync
        873 => "rsync".to_string(),

        // NFS
        2049 => "nfs".to_string(),

        // VNC
        5900 | 5901 | 5902 => "vnc".to_string(),

        // RDP
        3389 => "rdp".to_string(),

        _ => "unknown".to_string(),
    }
}

/**
 * Docker 容器安全扫描端点
 */
pub async fn scan_docker_security() -> impl IntoResponse {
    let issues = scan_docker_containers().await;

    let response = json!({
        "total_containers_scanned": issues.len(),
        "critical": issues.iter().filter(|i| i.severity.to_string() == "critical").count(),
        "high": issues.iter().filter(|i| i.severity.to_string() == "high").count(),
        "medium": issues.iter().filter(|i| i.severity.to_string() == "medium").count(),
        "low": issues.iter().filter(|i| i.severity.to_string() == "low").count(),
        "issues": issues
    });

    (StatusCode::OK, Json(response)).into_response()
}
