use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-tag delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,
}

/// 删除产品中的一个标签：`DELETE /v1/ship/products/{product_id}/tags/{tag_id}`（scope: `pcp:write:ship:product`）。
///
/// 返回被删除的对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdTagsByTagId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/tags/{tag_id}",
        product_id = args.product_id,
        tag_id = args.tag_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
