use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-category create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个发布类别：`POST /v1/pjm/projects/{project_id}/release_categories`
/// （scope: `pcp:write:pjm:release`）。
///
/// 请求体必填 `name`，可选 `section_id`（所属发布分组 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdReleaseCategories
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/projects/{}/release_categories", args.project_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
