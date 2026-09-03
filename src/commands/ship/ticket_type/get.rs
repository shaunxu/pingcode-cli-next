use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Ticket type id
    #[arg(value_name = "TICKET_TYPE_ID")]
    pub ticket_type_id: String,
}

/// 获取一个工单类型：`GET /v1/ship/ticket_types/{ticket_type_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketTypesByTicketTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/ticket_types/{ticket_type_id}",
        ticket_type_id = args.ticket_type_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
