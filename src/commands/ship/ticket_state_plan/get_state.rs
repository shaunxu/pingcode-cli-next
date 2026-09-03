use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan get-state` 的参数。
#[derive(Debug, Args)]
pub struct GetStateArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
    /// Ticket state id (the id within the state plan)
    #[arg(value_name = "STATE_ID")]
    pub state_id: String,
}

/// 获取工单状态方案中的一个状态：`GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id}`
/// （scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStatesByStateId
pub async fn run(ctx: &Ctx, args: &GetStateArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id}",
        state_plan_id = args.state_plan_id,
        state_id = args.state_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
