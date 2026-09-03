use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-plan get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Requirement plan id
    #[arg(value_name = "PLAN_ID")]
    pub plan_id: String,
}

/// 获取产品中的一个需求排期：`GET /v1/ship/products/{product_id}/plans/{plan_id}`（scope: `pcp:read:ship:product`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdPlansByPlanId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/plans/{plan_id}",
        product_id = args.product_id,
        plan_id = args.plan_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
