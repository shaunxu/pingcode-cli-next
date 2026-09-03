use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase bulk-create` 的参数。
#[derive(Debug, Args)]
pub struct BulkCreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量创建测试用例（一次最多 100 条）：`POST /v1/testhub/testcases/bulk`（scope: `pcp:write:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasesBulk
pub async fn run(ctx: &Ctx, args: &BulkCreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/testhub/testcases/bulk", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
