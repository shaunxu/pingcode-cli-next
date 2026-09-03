use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase search` 的参数。
#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 按结构化过滤条件搜索测试用例：`POST /v1/testhub/testcases/search`（scope: `pcp:write:testhub:testcase`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasesSearch
pub async fn run(ctx: &Ctx, args: &SearchArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx
        .client
        .post("/v1/testhub/testcases/search", &body)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
