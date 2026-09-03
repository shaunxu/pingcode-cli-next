use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-user create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 在产品中创建一个外部用户：`POST /v1/ship/products/{product_id}/users`（scope: `pcp:write:ship:product`）。
///
/// 请求体必填 `name`，`email` 与 `mobile` 至少填一个（同时存在以 `mobile` 为准），
/// 可选 `customer_id`（所属客户 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdUsers
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/users",
        product_id = args.product_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
