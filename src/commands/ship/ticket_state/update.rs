use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-state update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Ticket state id
    #[arg(value_name = "TICKET_STATE_ID")]
    pub ticket_state_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工单状态：`PATCH /v1/ship/ticket_states/{ticket_state_id}`（scope: `pcp:write:ship:configuration`）。
///
/// 请求体可选 `name`、`type`（pending/in_progress/completed/closed），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipTicketStatesByTicketStateId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/ticket_states/{ticket_state_id}",
        ticket_state_id = args.ticket_state_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
