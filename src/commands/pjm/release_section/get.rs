use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-section get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release section id
    #[arg(value_name = "RELEASE_SECTION_ID")]
    pub release_section_id: String,
}

/// 获取一个发布分组：`GET /v1/pjm/projects/{project_id}/release_sections/{release_section_id}`
/// （scope: `pcp:read:pjm:release`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/release_sections/{}",
        args.project_id, args.release_section_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
