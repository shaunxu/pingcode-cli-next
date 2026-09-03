use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-state list-for-library` 的参数。
#[derive(Debug, Args)]
pub struct ListForLibraryArgs {
    /// Library id
    #[arg(long, value_name = "ID")]
    pub library_id: String,
}

/// 获取测试库下可用的用例状态：`GET /v1/testhub/testcase/states`（scope: `pcp:read:testhub:testcase`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseStatesByLibraryId
pub async fn run(ctx: &Ctx, args: &ListForLibraryArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("library_id".into(), json!(args.library_id));

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/testcase/states", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
