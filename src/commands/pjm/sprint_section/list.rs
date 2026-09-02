use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint-section list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,
}

/// 获取迭代分组列表：`GET /v1/pjm/projects/{project_id}/sprint_sections`
/// （分页，scope: `pcp:read:pjm:sprint`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintSections
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/projects/{}/sprint_sections", args.project_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
