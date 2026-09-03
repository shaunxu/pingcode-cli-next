use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Test run id or short id
    #[arg(value_name = "TESTRUN_ID")]
    pub testrun_id: String,
    /// Include deleted test runs
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个执行用例（id 或 short_id）：`GET /v1/testhub/testruns/{testrun_id}`（scope: `pcp:read:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!("/v1/testhub/testruns/{}", args.testrun_id);
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
