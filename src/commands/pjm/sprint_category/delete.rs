use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint-category delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Sprint category id
    #[arg(value_name = "SPRINT_CATEGORY_ID")]
    pub sprint_category_id: String,
}

/// 删除一个迭代类别：`DELETE /v1/pjm/projects/{project_id}/sprint_categories/{sprint_category_id}`
/// （scope: `pcp:write:pjm:sprint`）。
///
/// 返回被删除的迭代类别对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdSprintCategoriesBySprintCategoryId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/sprint_categories/{}",
        args.project_id, args.sprint_category_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
