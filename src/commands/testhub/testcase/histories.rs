use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase histories` 的参数。
#[derive(Debug, Args)]
pub struct HistoriesArgs {
    /// Test case id or short id
    #[arg(value_name = "TESTCASE_ID")]
    pub testcase_id: String,
}

/// 获取测试用例的执行历史（每条测试执行的最近结果）：`GET /v1/testhub/testcases/{testcase_id}/histories`（scope: `pcp:read:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasesByTestcaseIdHistories
pub async fn run(ctx: &Ctx, args: &HistoriesArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testcases/{}/histories", args.testcase_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
