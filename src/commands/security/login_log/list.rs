use anyhow::{bail, Result};
use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc security login-log list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by login time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE", required = true)]
    pub logged_between: String,

    /// Filter by user ids, comma-separated (up to 20 ids)
    #[arg(long, value_name = "IDS")]
    pub user_ids: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取登录日志列表：`GET /v1/security/login_logs`
/// （scope: `pcp:read:global:security`，企业令牌或用户令牌均可）。
///
/// 登录时间范围必填；可选按成员 id 过滤（最多 20 个）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getSecurityLoginLogs
pub async fn run(ctx: &Ctx, args: &ListArgs) -> Result<()> {
    if let Some(user_ids) = &args.user_ids {
        let count = user_ids
            .split(',')
            .filter(|id| !id.trim().is_empty())
            .count();
        if count > 20 {
            bail!("--user-ids accepts at most 20 ids, got {count}");
        }
    }

    let mut query = serde_json::Map::new();
    query.insert("logged_between".into(), json!(args.logged_between));
    if let Some(user_ids) = &args.user_ids {
        query.insert("user_ids".into(), json!(user_ids));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/security/login_logs", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
