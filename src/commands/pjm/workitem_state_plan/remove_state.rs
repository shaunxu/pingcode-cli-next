use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state-plan remove-state` 的参数。
#[derive(Debug, Args)]
pub struct RemoveStateArgs {
    /// Work item state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,

    /// Work item state id
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,
}

/// 在工作项状态方案中移除一个状态：
/// `DELETE /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states/{state_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemStatePlansByStatePlanIdWorkitemStatesByStateId
pub async fn run(ctx: &Ctx, args: &RemoveStateArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_state_plans/{}/workitem_states/{}",
        args.state_plan_id, args.state_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
