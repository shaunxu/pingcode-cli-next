use anyhow::{bail, Result};
use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc security audit-log list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by operation time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE", required = true)]
    pub operated_between: String,

    /// Filter by operator (user) ids, comma-separated (up to 20 ids)
    #[arg(long, value_name = "IDS")]
    pub operated_bys: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取审计日志列表：`GET /v1/security/audit_logs`
/// （scope: `pcp:read:global:security`，企业令牌或用户令牌均可）。
///
/// 操作时间范围必填；可选按操作人 id 过滤（最多 20 个）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getSecurityAuditLogs
pub async fn run(ctx: &Ctx, args: &ListArgs) -> Result<()> {
    if let Some(operated_bys) = &args.operated_bys {
        let count = operated_bys
            .split(',')
            .filter(|id| !id.trim().is_empty())
            .count();
        if count > 20 {
            bail!("--operated-bys accepts at most 20 ids, got {count}");
        }
    }

    let mut query = serde_json::Map::new();
    query.insert("operated_between".into(), json!(args.operated_between));
    if let Some(operated_bys) = &args.operated_bys {
        query.insert("operated_bys".into(), json!(operated_bys));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/security/audit_logs", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
