use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page list-versions` 的参数。
#[derive(Debug, Args)]
pub struct ListVersionsArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取页面版本列表：`GET /v1/wiki/pages/{page_id}/versions`
/// （scope: `pcp:read:wiki:page`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdVersions
pub async fn run(ctx: &Ctx, args: &ListVersionsArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let path = format!("/v1/wiki/pages/{}/versions", args.page_id);
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
