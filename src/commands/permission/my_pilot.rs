use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// Pilot 的类型（查询参数 `type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PilotType {
    /// Ship product
    Product,
    /// Pjm project
    Project,
    /// Testhub library
    Library,
    /// Wiki space
    Space,
}

impl PilotType {
    pub fn as_str(self) -> &'static str {
        match self {
            PilotType::Product => "product",
            PilotType::Project => "project",
            PilotType::Library => "library",
            PilotType::Space => "space",
        }
    }
}

/// `pc permission my-pilot` 的参数。
#[derive(Debug, Args)]
pub struct MyPilotArgs {
    /// Pilot type
    #[arg(long, value_enum, value_name = "TYPE")]
    pub r#type: PilotType,

    /// Pilot id
    #[arg(long, value_name = "ID")]
    pub id: String,
}

/// 获取我的 Pilot 权限：`GET /v1/permission/my/pilot?type=<type>&id=<id>`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 响应为对象数组，元素包含 `key`（权限点）与 `has_permission`（是否具备）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyPilot
pub async fn run(ctx: &Ctx, args: &MyPilotArgs) -> anyhow::Result<()> {
    let query = json!({
        "type": args.r#type.as_str(),
        "id": args.id,
    });

    let response: Value = ctx
        .client
        .get_with_query("/v1/permission/my/pilot", &query)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
