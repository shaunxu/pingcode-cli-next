use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Test case id or short id
    #[arg(value_name = "TESTCASE_ID")]
    pub testcase_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新测试用例（steps 为整列表替换）：`PATCH /v1/testhub/testcases/{testcase_id}`（scope: `pcp:write:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasesByTestcaseId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/testhub/testcases/{}", args.testcase_id);
    let response: Value = ctx.client.patch(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
