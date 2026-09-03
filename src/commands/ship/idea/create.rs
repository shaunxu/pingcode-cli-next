use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个需求：`POST /v1/ship/ideas`（scope: `pcp:write:ship:idea`）。
///
/// 请求体必填 `product_id`、`title`（不超过 255 字符），可选 `assignee_id`、
/// `description`、`suite_id`（产品模块 id）、`priority_id`、`properties`
/// （属性键值对，需包含在当前产品的需求属性视图中），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeas
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/ship/ideas", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
