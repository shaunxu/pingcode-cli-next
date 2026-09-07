use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc permission my-global` 的参数。
#[derive(Debug, Args)]
pub struct MyGlobalArgs {}

/// 获取我的全局权限：`GET /v1/permission/my/global`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 响应为对象数组，元素包含 `key`（权限点）与 `has_permission`（是否具备）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyGlobal
pub async fn run(ctx: &Ctx, _args: &MyGlobalArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/permission/my/global").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
