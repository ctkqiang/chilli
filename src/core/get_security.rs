use crate::core::get_github_advisories::fetch_advisories;
use crate::models::github_advisories::ScanRequest;
use crate::models::log_level::LogLevel;
use crate::models::security::Model as SecurityIssue;
use crate::utils::logger;
use anyhow::Result;

#[allow(unused)]
pub async fn check_vulnerability_for_app(
    app_name: &str,
    app_version: &str,
    ecosystem: &str,
) -> Result<Vec<SecurityIssue>> {
    logger::log(
        LogLevel::Info,
        &format!(
            "扫描应用: {} {} (生态: {})",
            app_name, app_version, ecosystem
        ),
    );

    let req = ScanRequest {
        package: app_name.to_string(),
        version: app_version.to_string(),
        ecosystem: ecosystem.to_string(),
    };

    let scan_result = fetch_advisories(req).await?;

    let issues: Vec<SecurityIssue> = scan_result
        .vulnerabilities
        .into_iter()
        .map(|v| SecurityIssue {
            id: 0, // 数据库自增ID
            severity: v.severity,
            summary: v.summary,
            description: None,
            ghsa_id: v.ghsa_id,
            cve_id: v.cve_id,
            package: app_name.to_string(),
            current_version: app_version.to_string(),
            vulnerable_range: v.vulnerable_range,
            fixed_version: v.patched_version,
            published_at: "".to_string(),
        })
        .collect();

    logger::log(LogLevel::Info, &format!("发现 {} 个漏洞", issues.len()));
    Ok(issues)
}
