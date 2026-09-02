use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm sprint-section get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Sprint section id
    #[arg(value_name = "SECTION_ID")]
    pub section_id: String,
}

/// 获取一个迭代分组：`GET /v1/pjm/projects/{project_id}/sprint_sections/{section_id}`
/// （scope: `pcp:read:pjm:sprint`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdSprintSectionsBySectionId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/sprint_sections/{}",
        args.project_id, args.section_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
