use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-category get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release category id
    #[arg(value_name = "RELEASE_CATEGORY_ID")]
    pub release_category_id: String,
}

/// 获取一个发布类别：`GET /v1/pjm/projects/{project_id}/release_categories/{release_category_id}`
/// （scope: `pcp:read:pjm:release`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/release_categories/{}",
        args.project_id, args.release_category_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
