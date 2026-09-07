use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc expression evaluate` 的参数。
#[derive(Debug, Args)]
pub struct EvaluateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin;
    /// must contain "expressions" (1-16 strings, each 1-8192 chars)
    /// and optional "context"
    #[arg(long, value_name = "JSON")]
    pub data: String,

    /// Include metrics (the actual cost of expression evaluation) in the response
    #[arg(long)]
    pub expand: bool,
}

/// 计算一组表达式：`POST /v1/expression/evaluate`
/// （scope 依赖上下文主体对应的资源作用域，企业令牌或用户令牌均可）。
///
/// 请求体必填 `expressions`（需要计算的表达式列表，1～16 条，每条 1～8192 个字符），
/// 可选 `context`（计算上下文；`context.product` / `ticket` / `idea` / `project` /
/// `workitem` / `library` / `testcase` / `testrun` / `space` / `page` 传 id 时
/// 会在运行时自动补全为对应资源的全量结构，自定义数据通过 `context.custom` 传入）。
/// 查询参数 `expand=true` 时响应额外包含 `metrics`（计算的实际消耗）。
/// 响应为对象，含 `results`（计算结果列表）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postExpressionEvaluate
pub async fn run(ctx: &Ctx, args: &EvaluateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = if args.expand {
        ctx.client
            .post_with_query("/v1/expression/evaluate", &json!({"expand": true}), &body)
            .await?
    } else {
        ctx.client.post("/v1/expression/evaluate", &body).await?
    };

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
