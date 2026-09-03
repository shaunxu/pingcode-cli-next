use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product-suite create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 在产品中创建一个需求模块：`POST /v1/ship/products/{product_id}/suites`（scope: `pcp:write:ship:product`）。
///
/// 请求体必填 `name`（同一层次下名称不能重复）、`type`（product 子产品 / module 模块），
/// 可选 `parent_id`（父模块 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipProductsByProductIdSuites
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/ship/products/{product_id}/suites",
        product_id = args.product_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
