use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem batch-update` 的参数。
#[derive(Debug, Args)]
pub struct BatchUpdateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量部分更新工作项属性：`PATCH /v1/pjm/workitems`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 用于将多个工作项的同一属性更新为相同值，单次最多 100 条。
///
/// 请求体必填 `ids`（工作项 id 列表，最多 100 个）、`property_name`
/// （属性名，与「部分更新一个工作项」字段名一致；自定义属性用
/// `properties.<key>`）、`property_value`（属性值，类型随属性变化）。
/// 暂不支持更新 id/url/identifier/type/parent_id/created_at 等只读字段。
/// 响应为 `{ inserts, updates, deletes }` 计数对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitems
pub async fn run(ctx: &Ctx, args: &BatchUpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.patch("/v1/pjm/workitems", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
