use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea search` 的参数。
#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search criteria as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 搜索需求列表：`POST /v1/ship/ideas/search`（scope: `pcp:read:ship:idea`）。
///
/// 用于按类 MongoDB 结构化条件搜索需求；复杂组合、日期或自定义属性过滤
/// 请用本命令而非 `list`。
///
/// 请求体必填 `mode`（固定 `"query"`）与 `payload`（搜索参数对象），
/// 过滤条件写在 `payload.filter` 中，具体字段见官方文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeasSearch
pub async fn run(ctx: &Ctx, args: &SearchArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/ship/ideas/search", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
