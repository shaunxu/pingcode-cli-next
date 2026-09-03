use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun histories` 的参数。
#[derive(Debug, Args)]
pub struct HistoriesArgs {
    /// Test run id or short id
    #[arg(value_name = "TESTRUN_ID")]
    pub testrun_id: String,
}

/// 获取执行用例的结果记录列表：`GET /v1/testhub/testruns/{testrun_id}/histories`（scope: `pcp:read:testhub:testplan`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunIdHistories
pub async fn run(ctx: &Ctx, args: &HistoriesArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testruns/{}/histories", args.testrun_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
