use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization role get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Role id
    #[arg(value_name = "ROLE_ID")]
    pub role_id: String,
}

/// 获取一个角色：`GET /v1/directory/roles/{role_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 按 id 获取角色全量信息，含是否为系统内置（`is_system`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryRolesByRoleId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/roles/{}", args.role_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
