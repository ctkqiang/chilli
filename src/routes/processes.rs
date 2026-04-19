use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::core::get_running_process::get_running_applicaitons;
use crate::models::system_overview::SystemOverview;

/**
 * 获取当前运行中的进程树
 *
 * 本路由用于获取当前系统中所有运行中的进程树。
 * 它从系统调用中获取进程信息，并将其组织成一个 JSON 格式的响应。
 */
pub async fn runnning_processes() -> Json<SystemOverview> {
    let overview = get_running_applicaitons();
    Json(overview)
}

/**
 * HTTP API端点：终止指定PID的进程树
 *
 * # 功能概述
 * 本端点接收HTTP POST请求，根据URL路径参数中的PID终止对应的进程及其所有子进程。
 * 这是一个危险操作，会强制结束目标进程，请谨慎使用。
 *
 * # API端点信息
 * - **HTTP方法**: POST
 * - **路径**: `/api/kill/:pid`
 * - **Content-Type**: application/json
 *
 * # 请求参数
 * | 参数 | 类型 | 位置 | 必填 | 说明 |
 * |------|------|------|------|------|
 * | pid | u32 | Path | 是 | 要终止的进程ID |
 *
 * # 响应格式
 *
 * ## 成功响应 (200 OK)
 * ```json
 * {
 *   "status": "success",
 *   "pid": 12345
 * }
 * ```
 *
 * ## 错误响应 (500 Internal Server Error)
 * ```json
 * {
 *   "status": "error",
 *   "message": "Permission denied (os error 1)"
 * }
 * ```
 *
 * # 处理流程
 * ```
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                     进程终止API处理流程                                  │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │   HTTP POST /api/kill/12345                                             │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │ Axum Router     │  解析路径参数 pid=12345                           │
 * │   │ 路由匹配         │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │ kill_process()  │  调用处理函数                                      │
 * │   │ 处理函数         │                                                   │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐                                                   │
 * │   │ kill_tree(12345)│  递归终止进程树                                    │
 * │   │ 核心终止逻辑     │  - 发送SIGTERM给PID 12345                         │
 * │   │                 │  - 遍历并终止所有子进程                             │
 * │   └────────┬────────┘                                                   │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐     ┌─────────────────┐                          │
 * │   │   终止成功       │────▶│ 返回 200 OK     │                          │
 * │   │   Ok(())        │     │ + success JSON  │                          │
 * │   └─────────────────┘     └─────────────────┘                          │
 * │            │                                                            │
 * │            ▼                                                            │
 * │   ┌─────────────────┐     ┌─────────────────┐                          │
 * │   │   终止失败       │────▶│ 返回 500 Error  │                          │
 * │   │   Err(e)        │     │ + error JSON    │                          │
 * │   └─────────────────┘     └─────────────────┘                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 * ```
 *
 * # 常见错误及处理建议
 *
 * | 错误信息 | 原因 | 解决方案 |
 * |----------|------|----------|
 * | Permission denied | 权限不足 | 使用sudo运行或检查进程所有者 |
 * | No such process | 进程不存在 | 检查PID是否正确 |
 * | Operation not permitted | 尝试终止系统进程 | 避免终止关键系统进程 |
 *
 * # 使用示例
 *
 * ## cURL请求
 * ```bash
 * # 终止PID为12345的进程
 * curl -X POST http://localhost:9333/api/kill/12345
 *
 * # 预期响应
 * {"status":"success","pid":12345}
 * ```
 *
 * ## JavaScript/Fetch
 * ```javascript
 * async function killProcess(pid) {
 *   const response = await fetch(`http://localhost:9333/api/kill/${pid}`, {
 *     method: 'POST'
 *   });
 *   const result = await response.json();
 *
 *   if (result.status === 'success') {
 *     console.log(`进程 ${result.pid} 已终止`);
 *   } else {
 *     console.error('终止失败:', result.message);
 *   }
 * }
 *
 * killProcess(12345);
 * ```
 */
pub async fn kill_process(Path(pid): Path<u32>) -> impl IntoResponse {
    match crate::core::get_running_process::kill_process_by_pid(pid) {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "status": "success", "pid": pid })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
        ),
    }
}
