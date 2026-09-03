use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun replace` 的参数。
#[derive(Debug, Args)]
pub struct ReplaceArgs {
    /// Test run id or short id
    #[arg(value_name = "TESTRUN_ID")]
    pub testrun_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 全量替换执行结果与步骤（status_id 与 steps 必填）：`PUT /v1/testhub/testruns/{testrun_id}`（scope: `pcp:write:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/putTesthubTestrunsByTestrunId
pub async fn run(ctx: &Ctx, args: &ReplaceArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/testhub/testruns/{}", args.testrun_id);
    let response: Value = ctx.client.put(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
