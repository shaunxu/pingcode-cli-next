use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship product create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个产品：`POST /v1/ship/products`（scope: `pcp:write:ship:product`）。
///
/// 请求体必填 `name`（不超过 32 字符）、`identifier`（大写字母/数字/下划线/
/// 连接线，不超过 15 字符，企业内唯一），可选 `description`、`members`、
/// `scope_type`（organization/user_group，默认 organization）、`scope_id`
/// （scope_type 为 user_group 时必填）、`visibility`（public/private，默认 private），
/// 完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipProducts
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/ship/products", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
