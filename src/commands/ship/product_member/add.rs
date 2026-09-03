use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-member add` 的参数。
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向产品中添加一个成员：`POST /v1/ship/products/{product_id}/members`（scope: `pcp:write:ship:product`）。
///
/// 请求体必填 `member`（成员引用，如 `{"type":"user","id":"<user_id>"}` 或团队引用），
/// 可选 `role_id`（产品角色 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdMembers
pub async fn run(ctx: &Ctx, args: &AddArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/members",
        product_id = args.product_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
