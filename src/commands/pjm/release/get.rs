use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release id
    #[arg(value_name = "RELEASE_ID")]
    pub release_id: String,
}

/// 获取一个发布：`GET /v1/pjm/projects/{project_id}/releases/{release_id}`
/// （scope: `pcp:read:pjm:release`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleasesByReleaseId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/projects/{}/releases/{}",
        args.project_id, args.release_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
