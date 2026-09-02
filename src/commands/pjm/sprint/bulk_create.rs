use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint bulk-create` 的参数。
#[derive(Debug, Args)]
pub struct BulkCreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量创建迭代：`POST /v1/pjm/sprints/bulk`（仅企业令牌可用；
/// 单次数组不超过 100 条）。
///
/// 请求体必填 `sprints`（对象数组），数组中每个对象必填 `project_id`、
/// `name`、`start_at`、`end_at`（epoch 毫秒）、`assignee_id`，可选
/// `description`、`status`（pending/in_progress/completed）、`category_ids`，
/// 完整字段见文档。响应为结果对象数组（含 `state` 与 `sprint`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmSprintsBulk
pub async fn run(ctx: &Ctx, args: &BulkCreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/sprints/bulk", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
