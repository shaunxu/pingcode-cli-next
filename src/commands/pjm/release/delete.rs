use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release id
    #[arg(value_name = "RELEASE_ID")]
    pub release_id: String,
}

/// 删除一个发布：`DELETE /v1/pjm/projects/{project_id}/releases/{release_id}`
/// （scope: `pcp:write:pjm:release`）。
///
/// 返回被删除的发布对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdReleasesByReleaseId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/releases/{}",
        args.project_id, args.release_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
