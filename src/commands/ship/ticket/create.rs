use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个工单：`POST /v1/ship/tickets`（scope: `pcp:write:ship:ticket`）。
///
/// 请求体必填 `product_id`、`title`（不超过 255 字符）、`type_id`，可选
/// `description`、`submitter_id`（企业鉴权时有效）、`customer_id`、`channel_id`、
/// `assignee_id`、`priority_id`、`properties`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipTickets
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/ship/tickets", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
