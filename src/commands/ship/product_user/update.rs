use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-user update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// External user id
    #[arg(value_name = "USER_ID")]
    pub user_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个外部用户：`PATCH /v1/ship/products/{product_id}/users/{user_id}`（scope: `pcp:write:ship:product`）。
///
/// 请求体可选 `customer_id`（外部用户所属客户 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipProductsByProductIdUsersByUserId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/users/{user_id}",
        product_id = args.product_id,
        user_id = args.user_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
