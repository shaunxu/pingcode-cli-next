use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-priority get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item priority id
    #[arg(value_name = "PRIORITY_ID")]
    pub priority_id: String,
}

/// 获取一个工作项优先级：`GET /v1/pjm/workitem_priorities/{priority_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPrioritiesByPriorityId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_priorities/{}", args.priority_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
