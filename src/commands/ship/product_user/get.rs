use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-user get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// External user id
    #[arg(value_name = "USER_ID")]
    pub user_id: String,
}

/// 获取产品中的一个外部用户：`GET /v1/ship/products/{product_id}/users/{user_id}`（scope: `pcp:read:ship:product`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdUsersByUserId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/users/{user_id}",
        product_id = args.product_id,
        user_id = args.user_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
