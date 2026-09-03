use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-suite list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Parent suite id; 'root' lists top-level suites; omit to list all suites
    #[arg(long, value_name = "PARENT")]
    pub parent_id: Option<String>,
}

/// 获取测试库下的用例模块列表：`GET /v1/testhub/libraries/{library_id}/suites`（scope: `pcp:read:testhub:library`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdSuites
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(parent_id) = &args.parent_id {
        query.insert("parent_id".into(), json!(parent_id));
    }

    let path = format!("/v1/testhub/libraries/{}/suites", args.library_id);
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
