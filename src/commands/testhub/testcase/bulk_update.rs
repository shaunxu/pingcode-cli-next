use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase bulk-update` 的参数。
#[derive(Debug, Args)]
pub struct BulkUpdateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量部分更新测试用例（一次最多 100 条）：`PATCH /v1/testhub/testcases/bulk`（scope: `pcp:write:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasesBulk
pub async fn run(ctx: &Ctx, args: &BulkUpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx
        .client
        .patch("/v1/testhub/testcases/bulk", &body)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
