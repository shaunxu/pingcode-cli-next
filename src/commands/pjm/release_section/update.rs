use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-section update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release section id
    #[arg(value_name = "RELEASE_SECTION_ID")]
    pub release_section_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个发布分组：`PATCH /v1/pjm/projects/{project_id}/release_sections/{release_section_id}`
/// （scope: `pcp:write:pjm:release`）。
///
/// 请求体可选 `name`、`description`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/release_sections/{}",
        args.project_id, args.release_section_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
