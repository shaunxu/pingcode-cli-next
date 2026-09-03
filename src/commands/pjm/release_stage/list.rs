use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-stage list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取发布阶段列表：`GET /v1/pjm/release_stages`（分页，企业级配置，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmReleaseStages
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/release_stages").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
