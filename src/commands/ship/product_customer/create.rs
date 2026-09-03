use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-customer create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 在产品中创建一个客户：`POST /v1/ship/products/{product_id}/customers`（scope: `pcp:write:ship:product`）。
///
/// 请求体必填 `name`，可选 `assignee_id`（负责人 id）、`scale`（规模）、`description`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdCustomers
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/customers",
        product_id = args.product_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
