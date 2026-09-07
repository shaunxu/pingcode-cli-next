use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc permission check-principal` 的参数。
#[derive(Debug, Args)]
pub struct CheckPrincipalArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin;
    /// must contain "permissions" (permission point keys), "type"
    /// (idea|ticket|workitem|testcase|page) and "ids" (principal id list)
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量检查我的 Principal 权限：`POST /v1/permission/check/principal`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 请求体必填 `permissions`（权限点列表）、`type`（Principal 类型：
/// `idea` / `ticket` / `workitem` / `testcase` / `page`）、`ids`（Principal id 列表）；
/// 响应为对象，属性名为请求中的权限点，每个属性值含 `principal_ids`
/// （具备该权限的 Principal id 列表，无权限时为空数组）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckPrincipal
pub async fn run(ctx: &Ctx, args: &CheckPrincipalArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx
        .client
        .post("/v1/permission/check/principal", &body)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
