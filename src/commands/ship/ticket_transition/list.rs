use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-transition list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Ticket id
    #[arg(value_name = "TICKET_ID")]
    pub ticket_id: String,
}

/// 获取工单流转记录列表：
/// `GET /v1/ship/tickets/{ticket_id}/transition_histories`
/// （分页，scope: `pcp:read:ship:ticket`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketsByTicketIdTransitionHistories
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/ship/tickets/{}/transition_histories", args.ticket_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
