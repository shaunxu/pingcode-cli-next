use anyhow::{bail, Result};
use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by space id
    #[arg(long, value_name = "ID")]
    pub space_id: Option<String>,

    /// Filter by parent page id (direct children only); mutually exclusive with --ancestor-id
    #[arg(long, value_name = "ID")]
    pub parent_id: Option<String>,

    /// Filter by ancestor page id (all descendants); mutually exclusive with --parent-id
    #[arg(long, value_name = "ID")]
    pub ancestor_id: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 分页获取页面列表：`GET /v1/wiki/pages`（scope: `pcp:read:wiki:page`）。
///
/// 可按空间、父页面（直接子页面）或祖先页面（全部子孙）过滤；
/// `--parent-id` 与 `--ancestor-id` 互斥，只能传入其中一个。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPages
pub async fn run(ctx: &Ctx, args: &ListArgs) -> Result<()> {
    if args.parent_id.is_some() && args.ancestor_id.is_some() {
        bail!("--parent-id and --ancestor-id are mutually exclusive");
    }

    let mut query = serde_json::Map::new();
    if let Some(space_id) = &args.space_id {
        query.insert("space_id".into(), json!(space_id));
    }
    if let Some(parent_id) = &args.parent_id {
        query.insert("parent_id".into(), json!(parent_id));
    }
    if let Some(ancestor_id) = &args.ancestor_id {
        query.insert("ancestor_id".into(), json!(ancestor_id));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/wiki/pages", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
