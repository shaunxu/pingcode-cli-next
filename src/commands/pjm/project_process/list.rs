use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-process list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 获取全部项目流程：`GET /v1/pjm/processes`（分页，企业级配置，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 分页获取企业内全部项目流程。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProcesses
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/processes").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
