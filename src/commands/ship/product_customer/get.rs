use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-customer get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Customer id
    #[arg(value_name = "CUSTOMER_ID")]
    pub customer_id: String,
}

/// 获取产品中的一个客户：`GET /v1/ship/products/{product_id}/customers/{customer_id}`（scope: `pcp:read:ship:product`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdCustomersByCustomerId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/customers/{customer_id}",
        product_id = args.product_id,
        customer_id = args.customer_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
