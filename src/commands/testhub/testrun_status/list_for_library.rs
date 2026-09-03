use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun-status list-for-library` 的参数。
#[derive(Debug, Args)]
pub struct ListForLibraryArgs {
    /// Library id
    #[arg(long, value_name = "ID")]
    pub library_id: String,
}

/// 获取测试库下可用的执行结果状态：`GET /v1/testhub/testrun/statuses`（scope: `pcp:read:testhub:testplan`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunStatusesByLibraryId
pub async fn run(ctx: &Ctx, args: &ListForLibraryArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("library_id".into(), json!(args.library_id));

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/testrun/statuses", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
