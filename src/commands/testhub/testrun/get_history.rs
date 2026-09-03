use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun get-history` 的参数。
#[derive(Debug, Args)]
pub struct GetHistoryArgs {
    /// Test run id or short id
    #[arg(value_name = "TESTRUN_ID")]
    pub testrun_id: String,
    /// History (result record) id
    #[arg(value_name = "HISTORY_ID")]
    pub history_id: String,
}

/// 获取一条执行结果记录：`GET /v1/testhub/testruns/{testrun_id}/histories/{history_id}`（scope: `pcp:read:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunIdHistoriesByHistoryId
pub async fn run(ctx: &Ctx, args: &GetHistoryArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/testruns/{}/histories/{}",
        args.testrun_id, args.history_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
