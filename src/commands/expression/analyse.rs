use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc expression analyse` 的参数。
#[derive(Debug, Args)]
pub struct AnalyseArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin;
    /// must contain "expressions" (1-16 strings, each 1-8192 chars)
    /// and optional "context"
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 分析一组表达式：`POST /v1/expression/analyse`
/// （scope 依赖上下文主体对应的资源作用域，企业令牌或用户令牌均可）。
///
/// 请求体必填 `expressions`（需要分析的表达式列表，1～16 条，每条 1～8192 个字符），
/// 可选 `context`（分析上下文；`context.product` / `ticket` / `idea` / `project` /
/// `workitem` / `library` / `testcase` / `testrun` / `space` / `page` 传 id 时
/// 会在运行时自动补全为对应资源的全量结构，自定义数据通过 `context.custom` 传入）。
/// 响应为对象，含 `results`（分析结果列表）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postExpressionAnalyse
pub async fn run(ctx: &Ctx, args: &AnalyseArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/expression/analyse", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
