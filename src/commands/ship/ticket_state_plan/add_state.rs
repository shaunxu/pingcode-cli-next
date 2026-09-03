use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state-plan add-state` 的参数。
#[derive(Debug, Args)]
pub struct AddStateArgs {
    /// Ticket state plan id
    #[arg(value_name = "STATE_PLAN_ID")]
    pub state_plan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向工单状态方案中添加一个状态：`POST /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states`
/// （scope: `pcp:write:ship:configuration`）。
///
/// 请求体必填 `state_id`（工单状态 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketStatePlansByStatePlanIdTicketStates
pub async fn run(ctx: &Ctx, args: &AddStateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;
    let path = format!(
        "/v1/ship/ticket_state_plans/{state_plan_id}/ticket_states",
        state_plan_id = args.state_plan_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
