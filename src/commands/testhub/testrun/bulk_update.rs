use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testrun bulk-update` 的参数。
#[derive(Debug, Args)]
pub struct BulkUpdateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量部分更新执行用例（一次最多 100 条）：`PATCH /v1/testhub/testruns/bulk`（scope: `pcp:write:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestrunsBulk
pub async fn run(ctx: &Ctx, args: &BulkUpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.patch("/v1/testhub/testruns/bulk", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
