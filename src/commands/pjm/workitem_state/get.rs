use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item state id
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,
}

/// 获取一个工作项状态：`GET /v1/pjm/workitem_states/{state_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatesByStateId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_states/{}", args.state_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
