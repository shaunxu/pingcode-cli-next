use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc relations create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: principal_type, principal_id, target_type, target_id.
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个关联：`POST /v1/relations`（scope 同时依赖关联主体和关联目标的
/// 写作用域，如关联需求与工单需要 `pcp:write:ship:idea` +
/// `pcp:write:ship:ticket`）。
///
/// 请求体必填 `principal_type`/`principal_id`（关联主体）、
/// `target_type`/`target_id`（关联目标）四个字段，主体类型与目标类型需搭配
/// 使用（如 `idea` ↔ `ticket`）；通用关联不区分关联类型（无
/// `relation_type` 字段）。Wiki 页面暂不支持作为关联主体创建。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postRelations
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/relations", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
