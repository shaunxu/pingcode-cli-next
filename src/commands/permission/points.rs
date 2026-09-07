use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc permission points` 的参数。
#[derive(Debug, Args)]
pub struct PointsArgs {}

/// 获取全部权限点定义：`GET /v1/permission/points`
/// （scope: `pcp:read:global:permission`，企业令牌或用户令牌均可）。
///
/// 响应为对象，包含 `global`、`pjm`、`ship`、`testhub`、`wiki` 属性，
/// 对应各模块的权限信息数组。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionPoints
pub async fn run(ctx: &Ctx, _args: &PointsArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/permission/points").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
