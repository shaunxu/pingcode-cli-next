use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc ship product get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product id
    #[arg(value_name = "PRODUCT_ID")]
    pub product_id: String,

    /// Include deleted products
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived products
    #[arg(long)]
    pub include_archived: bool,
}

/// 获取一个产品：`GET /v1/ship/products/{product_id}`（scope: `pcp:read:ship:product`）。
///
/// 默认不含已删除/已归档产品。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipProductsByProductId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let path = format!("/v1/ship/products/{}", args.product_id);
    let response: Value = ctx
        .client
        .get_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
