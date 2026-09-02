use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint-section delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Sprint section id
    #[arg(value_name = "SECTION_ID")]
    pub section_id: String,
}

/// 删除一个迭代分组：`DELETE /v1/pjm/projects/{project_id}/sprint_sections/{section_id}`
/// （scope: `pcp:write:pjm:sprint`）。
///
/// 返回被删除的迭代分组对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdSprintSectionsBySectionId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/sprint_sections/{}",
        args.project_id, args.section_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
