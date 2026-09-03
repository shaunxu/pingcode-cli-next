use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization user bulk-update` 的参数。
#[derive(Debug, Args)]
pub struct BulkUpdateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量更新企业成员属性：`PATCH /v1/directory/users/bulk`
/// （scope: `pcp:write:global:team`）。
///
/// 用于将多个成员的同一属性更新为相同值。请求体必填 `user_ids`
/// （成员 id 数组，不能包含自己和团队拥有者）、`property_name`
/// （目前仅支持 `status`）、`property_value`（`enabled` / `disabled`）。
/// 响应为每个成员的结果数组（`state` / `user_id` / `message`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryUsersBulk
pub async fn run(ctx: &Ctx, args: &BulkUpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.patch("/v1/directory/users/bulk", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
