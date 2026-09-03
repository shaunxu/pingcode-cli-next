use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-priority list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {}

/// 获取全部工作项优先级列表：`GET /v1/pjm/workitem_priorities`（分页，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 分页获取企业内全部工作项优先级。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPriorities
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/workitem_priorities").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
