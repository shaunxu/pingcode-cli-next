use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-customer update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Customer id
    #[arg(value_name = "CUSTOMER_ID")]
    pub customer_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个客户：`PATCH /v1/ship/products/{product_id}/customers/{customer_id}`（scope: `pcp:write:ship:product`）。
///
/// 请求体可选 `name`、`assignee_id`（负责人 id）、`scale`（规模）、`description`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipProductsByProductIdCustomersByCustomerId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/customers/{customer_id}",
        product_id = args.product_id,
        customer_id = args.customer_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
