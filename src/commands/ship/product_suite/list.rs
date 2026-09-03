use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-suite list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
}

/// 分页获取产品中的需求模块列表：`GET /v1/ship/products/{product_id}/suites`（scope: `pcp:read:ship:product`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductIdSuites
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/suites",
        product_id = args.product_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
