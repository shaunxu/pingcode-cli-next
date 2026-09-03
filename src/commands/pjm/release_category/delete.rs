use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-category delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release category id
    #[arg(value_name = "RELEASE_CATEGORY_ID")]
    pub release_category_id: String,
}

/// 删除一个发布类别：`DELETE /v1/pjm/projects/{project_id}/release_categories/{release_category_id}`
/// （scope: `pcp:write:pjm:release`）。
///
/// 返回被删除的发布类别对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/release_categories/{}",
        args.project_id, args.release_category_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
