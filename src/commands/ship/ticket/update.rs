use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Ticket id
    #[arg(value_name = "TICKET_ID")]
    pub ticket_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工单：`PATCH /v1/ship/tickets/{ticket_id}`（scope: `pcp:write:ship:ticket`）。
///
/// 请求体可选 `title`、`description`、`type_id`、`state_id`、`assignee_id`、
/// `submitter_id`（企业鉴权时有效）、`solution_id`、`priority_id`、`customer_id`、
/// `properties`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipTicketsByTicketId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/ship/tickets/{}", args.ticket_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
