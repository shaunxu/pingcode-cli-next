use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property-plan list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by library id (locally-configured plans)
    #[arg(long, value_name = "ID")]
    pub library_id: Option<String>,
}

/// 分页获取用例属性方案列表：`GET /v1/testhub/testcase_property_plans`（scope: `pcp:read:testhub:configuration`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlans
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(library_id) = &args.library_id {
        query.insert("library_id".into(), json!(library_id));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/testcase_property_plans", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
