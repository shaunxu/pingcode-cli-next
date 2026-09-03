use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Filter by plan name
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
    /// Filter by creation time, epoch seconds
    #[arg(long, value_name = "RANGE")]
    pub created_between: Option<String>,
    /// Filter by update time, epoch seconds
    #[arg(long, value_name = "RANGE")]
    pub updated_between: Option<String>,
}

/// 分页获取测试库下的测试计划列表：`GET /v1/testhub/libraries/{library_id}/testplans`（scope: `pcp:read:testhub:testplan`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplans
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(name) = &args.name {
        query.insert("name".into(), json!(name));
    }
    if let Some(created_between) = &args.created_between {
        query.insert("created_between".into(), json!(created_between));
    }
    if let Some(updated_between) = &args.updated_between {
        query.insert("updated_between".into(), json!(updated_between));
    }

    let path = format!("/v1/testhub/libraries/{}/testplans", args.library_id);
    let response: Value = ctx
        .client
        .get_with_query(path.as_str(), &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
