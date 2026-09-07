use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc permission check-pilot` 的参数。
#[derive(Debug, Args)]
pub struct CheckPilotArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin;
    /// must contain "permissions" (permission point keys), "type"
    /// (product|project|library|space) and "ids" (pilot id list)
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 批量检查我的 Pilot 权限：`POST /v1/permission/check/pilot`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 请求体必填 `permissions`（权限点列表）、`type`（Pilot 类型：
/// `product` / `project` / `library` / `space`）、`ids`（Pilot id 列表）；
/// 响应为对象，属性名为请求中的权限点，每个属性值含 `pilot_ids`
/// （具备该权限的 Pilot id 列表，无权限时为空数组）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckPilot
pub async fn run(ctx: &Ctx, args: &CheckPilotArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/permission/check/pilot", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
