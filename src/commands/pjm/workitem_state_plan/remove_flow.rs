use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state-plan remove-flow` 的参数。
#[derive(Debug, Args)]
pub struct RemoveFlowArgs {
    /// Work item state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,

    /// Work item state flow (transition) id
    #[arg(value_name = "FLOW_ID")]
    pub flow_id: String,
}

/// 在工作项状态方案中移除一条状态流转：
/// `DELETE /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows/{flow_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemStatePlansByStatePlanIdWorkitemStateFlowsByFlowId
pub async fn run(ctx: &Ctx, args: &RemoveFlowArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_state_plans/{}/workitem_state_flows/{}",
        args.state_plan_id, args.flow_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
