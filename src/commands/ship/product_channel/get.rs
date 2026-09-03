use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-channel get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Ticket channel id
    #[arg(value_name = "CHANNEL_ID")]
    pub channel_id: String,
}

/// 获取产品中的一个工单渠道：`GET /v1/ship/products/{product_id}/channels/{channel_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdChannelsByChannelId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/channels/{channel_id}",
        product_id = args.product_id,
        channel_id = args.channel_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
