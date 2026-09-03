use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun plan-batch` 的参数。
#[derive(Debug, Args)]
pub struct PlanBatchArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Test plan id or short id
    #[arg(value_name = "TESTPLAN_ID")]
    pub testplan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 在测试计划下批量新增/更新/删除执行用例（inserts/updates/deletes 各最多 50 条；删除执行用例的唯一途径）：`POST /v1/testhub/libraries/{library_id}/testplans/{testplan_id}/testruns/bulk`（scope: `pcp:write:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdTestplansByTestplanIdTestrunsBulk
pub async fn run(ctx: &Ctx, args: &PlanBatchArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/testhub/libraries/{}/testplans/{}/testruns/bulk",
        args.library_id, args.testplan_id
    );
    let response: Value = ctx.client.post(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
