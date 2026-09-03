use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-category list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,
}

/// 获取发布类别列表：`GET /v1/pjm/projects/{project_id}/release_categories`
/// （分页，scope: `pcp:read:pjm:release`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseCategories
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/projects/{}/release_categories", args.project_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
