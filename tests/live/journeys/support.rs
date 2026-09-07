//! 旅程间共享的辅助：测试项目/用户/工作项复用与时间戳。
//!
//! 多个 pjm 旅程共用一个测试项目（进程内只创建一次，减少租户残留）；
//! 无删除接口的共享项目用 `pc-live-shared` 前缀，便于在测试租户人工清理。

use std::sync::{Mutex, OnceLock};

use serde_json::json;

use crate::common::{str_field, values, LiveCtx};

static SHARED_PROJECT: OnceLock<Mutex<Option<String>>> = OnceLock::new();

/// 取共享测试项目 id：优先 `PC_LIVE_PROJECT_ID`，否则进程内只创建一次。
pub fn shared_project_id(ctx: &LiveCtx) -> String {
    if let Some(pid) = &ctx.reuse_project_id {
        return pid.clone();
    }
    let cell = SHARED_PROJECT.get_or_init(|| Mutex::new(None));
    let mut guard = cell.lock().unwrap();
    if let Some(pid) = guard.as_ref() {
        return pid.clone();
    }
    let name = ctx.unique_name("shared");
    let identifier = ctx.unique_identifier("SH");
    // hybrid 项目同时具备看板/迭代/发布能力，供看板、发布、工作项扩展等多个旅程复用。
    let body = json!({
        "name": name,
        "identifier": identifier,
        "type": "hybrid",
        "visibility": "private",
    });
    let project = ctx.run_ok(&["pjm", "project", "create", "--data", &body.to_string()]);
    let pid = str_field(&project, "id")
        .expect("create shared project response has no id")
        .to_string();
    eprintln!("created shared project {name} (id={pid})");
    *guard = Some(pid.clone());
    pid
}

/// 取企业第一个用户 id（用于 assignee/report_by/participant 等字段）。
pub fn first_user_id(ctx: &LiveCtx) -> Option<String> {
    let users = ctx.run_ok(&["organization", "user", "list", "--page-size", "10"]);
    values(&users)
        .first()
        .and_then(|u| str_field(u, "id"))
        .map(str::to_string)
}

/// 取项目的第一个工作项类型 id。
pub fn first_workitem_type_id(ctx: &LiveCtx, project_id: &str) -> String {
    let types = ctx.run_ok(&[
        "pjm",
        "workitem-type",
        "list-for-project",
        "--project-id",
        project_id,
    ]);
    values(&types)
        .first()
        .and_then(|t| str_field(t, "id"))
        .expect("project has no workitem types")
        .to_string()
}

/// 在指定项目创建一个工作项，返回其 id。
pub fn create_workitem(ctx: &LiveCtx, project_id: &str, type_id: &str, title: &str) -> String {
    let body = json!({ "project_id": project_id, "type_id": type_id, "title": title });
    let wi = ctx.run_ok(&["pjm", "workitem", "create", "--data", &body.to_string()]);
    str_field(&wi, "id")
        .expect("create workitem response has no id")
        .to_string()
}

/// 当前时间的 10 位秒级时间戳。
pub fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_secs() as i64
}
