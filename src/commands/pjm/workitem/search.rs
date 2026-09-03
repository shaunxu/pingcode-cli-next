use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem search` 的参数。
#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search criteria as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 搜索工作项列表：`POST /v1/pjm/workitems/search`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 用于按类 MongoDB 结构化条件搜索工作项；复杂组合、日期或自定义属性
/// 过滤请用本命令而非 `list`。默认不含已删除/已归档项。
///
/// 请求体必填 `mode`（固定 `"query"`）与 `payload`（搜索参数对象），
/// 过滤条件写在 `payload.filter` 中，具体字段见官方文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsSearch
pub async fn run(ctx: &Ctx, args: &SearchArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/workitems/search", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
