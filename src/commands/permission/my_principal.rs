use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// Principal 的类型（查询参数 `type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Ship idea
    Idea,
    /// Ship ticket
    Ticket,
    /// Pjm work item
    Workitem,
    /// Testhub test case
    Testcase,
    /// Wiki page
    Page,
}

impl PrincipalType {
    pub fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Idea => "idea",
            PrincipalType::Ticket => "ticket",
            PrincipalType::Workitem => "workitem",
            PrincipalType::Testcase => "testcase",
            PrincipalType::Page => "page",
        }
    }
}

/// `pc permission my-principal` 的参数。
#[derive(Debug, Args)]
pub struct MyPrincipalArgs {
    /// Principal type
    #[arg(long, value_enum, value_name = "TYPE")]
    pub r#type: PrincipalType,

    /// Principal id
    #[arg(long, value_name = "ID")]
    pub id: String,
}

/// 获取我的 Principal 权限：`GET /v1/permission/my/principal?type=<type>&id=<id>`
/// （scope: `pcp:read:global:permission`，仅用户令牌可用）。
///
/// 响应为对象数组，元素包含 `key`（权限点）与 `has_permission`（是否具备）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyPrincipal
pub async fn run(ctx: &Ctx, args: &MyPrincipalArgs) -> anyhow::Result<()> {
    let query = json!({
        "type": args.r#type.as_str(),
        "id": args.id,
    });

    let response: Value = ctx
        .client
        .get_with_query("/v1/permission/my/principal", &query)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
