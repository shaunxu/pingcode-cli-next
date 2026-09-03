use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by test plan id
    #[arg(long, value_name = "ID")]
    pub testplan_id: Option<String>,
    /// Filter by test case id
    #[arg(long, value_name = "ID")]
    pub testcase_id: Option<String>,
    /// Filter by suite id
    #[arg(long, value_name = "ID")]
    pub suite_id: Option<String>,
    /// Filter by execution result status id
    #[arg(long, value_name = "ID")]
    pub status_id: Option<String>,
    /// Filter by keyword
    #[arg(long, value_name = "KEYWORDS")]
    pub keywords: Option<String>,
}

/// 分页获取执行用例列表：`GET /v1/testhub/testruns`（scope: `pcp:read:testhub:testplan`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestruns
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(testplan_id) = &args.testplan_id {
        query.insert("testplan_id".into(), json!(testplan_id));
    }
    if let Some(testcase_id) = &args.testcase_id {
        query.insert("testcase_id".into(), json!(testcase_id));
    }
    if let Some(suite_id) = &args.suite_id {
        query.insert("suite_id".into(), json!(suite_id));
    }
    if let Some(status_id) = &args.status_id {
        query.insert("status_id".into(), json!(status_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/testhub/testruns", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
