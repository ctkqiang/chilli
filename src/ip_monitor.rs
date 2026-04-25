use crate::models::log_level::LogLevel;
use crate::utils;
use sea_orm::DatabaseConnection;

pub fn start_ip_monitor(db: DatabaseConnection) {
    tokio::spawn(async move {
        loop {
            match crate::core::ip_audit::get_active_connections() {
                Ok(connections) => {
                    for conn in connections {
                        let log = crate::models::access_log::Model {
                            id: 0,
                            src_ip: conn.src_ip.clone(),
                            dst_port: conn.dst_port as i32,
                            process_name: conn.process_name.clone(),
                            pid: conn.pid as i64,
                            timestamp: conn.timestamp,
                        };

                        if let Err(e) =
                            crate::service::database::insert_access_log(&db, &log).await
                        {
                            utils::logger::log(
                                LogLevel::Warn,
                                &format!("IP审计日志写入失败: {}", e),
                            );
                        }
                    }
                }
                Err(e) => {
                    utils::logger::log(
                        LogLevel::Warn,
                        &format!("获取活动连接失败: {}", e),
                    );
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}
