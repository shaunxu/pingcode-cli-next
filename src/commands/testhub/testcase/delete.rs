use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Test case id or short id
    #[arg(value_name = "TESTCASE_ID")]
    pub testcase_id: String,
}

/// 删除测试用例：`DELETE /v1/testhub/testcases/{testcase_id}`（scope: `pcp:write:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubTestcasesByTestcaseId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testcases/{}", args.testcase_id);
    let response: Value = ctx.client.delete(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
