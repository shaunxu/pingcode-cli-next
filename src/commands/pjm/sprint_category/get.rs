use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint-category get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Sprint category id
    #[arg(value_name = "SPRINT_CATEGORY_ID")]
    pub sprint_category_id: String,
}

/// 获取一个迭代类别：`GET /v1/pjm/projects/{project_id}/sprint_categories/{sprint_category_id}`
/// （scope: `pcp:read:pjm:sprint`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintCategoriesBySprintCategoryId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/sprint_categories/{}",
        args.project_id, args.sprint_category_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
