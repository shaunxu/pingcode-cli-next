use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Test plan id or short id
    #[arg(value_name = "TESTPLAN_ID")]
    pub testplan_id: String,
    /// Include deleted test plans
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个测试计划（id 或 short_id）：`GET /v1/testhub/libraries/{library_id}/testplans/{testplan_id}`（scope: `pcp:read:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplansByTestplanId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!(
        "/v1/testhub/libraries/{}/testplans/{}",
        args.library_id, args.testplan_id
    );
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
