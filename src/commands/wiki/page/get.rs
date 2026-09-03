use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Include deleted pages
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个页面：`GET /v1/wiki/pages/{page_id}`
/// （scope: `pcp:read:wiki:page`）。
///
/// 按 id 获取页面全量信息，默认不含已删除页面。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!("/v1/wiki/pages/{}", args.page_id);
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
