use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-member remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Member id (enterprise user id or team id)
    #[arg(value_name = "MEMBER_ID")]
    pub member_id: String,
}

/// 从产品中移除一个成员：`DELETE /v1/ship/products/{product_id}/members/{member_id}`（scope: `pcp:write:ship:product`）。
///
/// 返回被移除的成员对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipProductsByProductIdMembersByMemberId
pub async fn run(ctx: &Ctx, args: &RemoveArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/products/{product_id}/members/{member_id}",
        product_id = args.product_id,
        member_id = args.member_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
