use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-suite get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Requirement module id
    #[arg(value_name = "SUITE_ID")]
    pub suite_id: String,
}

/// 获取产品中的一个需求模块：`GET /v1/ship/products/{product_id}/suites/{suite_id}`（scope: `pcp:read:ship:product`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdSuitesBySuiteId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/suites/{suite_id}",
        product_id = args.product_id,
        suite_id = args.suite_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
