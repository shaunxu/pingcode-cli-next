use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个迭代：`POST /v1/pjm/projects/{project_id}/sprints`
/// （scope: `pcp:write:pjm:sprint`）。
///
/// 请求体必填 `name`、`start_at`、`end_at`（epoch 毫秒）、`assignee_id`，
/// 可选 `description`、`status`（pending/in_progress/completed）、`category_ids`，
/// 完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdSprints
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}/sprints", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
