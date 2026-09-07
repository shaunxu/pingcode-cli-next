use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc permission check-global` 的参数。
#[derive(Debug, Args)]
pub struct CheckGlobalArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin;
    /// must contain "permissions" (array of permission point keys to check)
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 检查我的全局权限：`POST /v1/permission/check/global`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 请求体必填 `permissions`（需要检查的权限点列表）；
/// 响应为对象数组，元素包含请求中每个权限点的 `key` 与 `has_permission`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckGlobal
pub async fn run(ctx: &Ctx, args: &CheckGlobalArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx
        .client
        .post("/v1/permission/check/global", &body)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
