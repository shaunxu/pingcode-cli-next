use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个项目：`PATCH /v1/pjm/projects/{project_id}`（scope: `pcp:write:pjm:project`）。
///
/// 请求体可选 `name`、`identifier`、`description`、`start_at`、`end_at`、
/// `assignee_id`、`state_id`、`properties`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}", args.project_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
