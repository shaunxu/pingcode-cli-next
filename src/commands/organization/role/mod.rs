//! 角色（role）资源：`pc organization role <operation>`。
//!
//! 对应 `/v1/directory/roles` 及其子路径的 REST 接口。只读。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`RoleCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc organization role` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum RoleCommand {
    /// List enterprise roles, including built-in and custom (GET /v1/directory/roles)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryRoles
    List(Box<ListArgs>),

    /// Get a role by id (GET /v1/directory/roles/{role_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryRolesByRoleId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: RoleCommand) -> anyhow::Result<()> {
    match command {
        RoleCommand::List(args) => list::run(ctx, &args).await,
        RoleCommand::Get(args) => get::run(ctx, &args).await,
    }
}
