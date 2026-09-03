use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc ship ticket-channel list-for-product` 的参数。
#[derive(Debug, Args)]
pub struct ListForProductArgs {
    /// Product id
    #[arg(value_name = "ID")]
    pub product_id: String,
}

/// 分页获取产品中的工单渠道：`GET /v1/ship/ticket/channels`
/// （查询参数 `product_id`，scope: `pcp:read:ship:ticket`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketChannelsByProductId
pub async fn run(ctx: &Ctx, args: &ListForProductArgs) -> anyhow::Result<()> {
    let query = serde_json::Map::from_iter([("product_id".into(), json!(args.product_id))]);

    let response: Value = ctx
        .client
        .get_with_query("/v1/ship/ticket/channels", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
