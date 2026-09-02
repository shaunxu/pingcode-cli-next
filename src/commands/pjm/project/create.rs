use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个项目：`POST /v1/pjm/projects`（scope: `pcp:write:pjm:project`）。
///
/// 请求体必填 `name`、`type`（scrum/kanban/waterfall/hybrid）、`identifier`，
/// 可选 `scope_type`、`scope_id`、`visibility`、`process_id`、`description`、
/// `members`、`start_at`、`end_at`、`assignee_id`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjects
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/projects", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
