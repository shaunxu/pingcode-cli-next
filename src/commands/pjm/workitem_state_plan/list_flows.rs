use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-state-plan list-flows` 的参数。
#[derive(Debug, Args)]
pub struct ListFlowsArgs {
    /// Work item state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
}

/// 获取工作项状态方案中的状态流转列表：
/// `GET /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows`
/// （分页，scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanIdWorkitemStateFlows
pub async fn run(ctx: &Ctx, args: &ListFlowsArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitem_state_plans/{}/workitem_state_flows",
        args.state_plan_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
