use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-stage get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Release stage id
    #[arg(value_name = "RELEASE_STAGE_ID")]
    pub release_stage_id: String,
}

/// 获取一个发布阶段：`GET /v1/pjm/release_stages/{release_stage_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmReleaseStagesByReleaseStageId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/release_stages/{}", args.release_stage_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
