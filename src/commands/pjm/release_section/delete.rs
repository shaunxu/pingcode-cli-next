use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-section delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release section id
    #[arg(value_name = "RELEASE_SECTION_ID")]
    pub release_section_id: String,
}

/// 删除一个发布分组：`DELETE /v1/pjm/projects/{project_id}/release_sections/{release_section_id}`
/// （scope: `pcp:write:pjm:release`）。
///
/// 返回被删除的发布分组对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleaseSectionsByReleaseSectionId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/release_sections/{}",
        args.project_id, args.release_section_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
