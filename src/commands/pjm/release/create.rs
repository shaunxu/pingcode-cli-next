use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个发布：`POST /v1/pjm/projects/{project_id}/releases`
/// （scope: `pcp:write:pjm:release`）。
///
/// 请求体必填 `name`、`start_at`、`end_at`（epoch 毫秒）、`assignee_id`，
/// 可选 `stage_id`、`category_ids`（发布类别 id 数组），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdReleases
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}/releases", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
