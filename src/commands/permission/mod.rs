//! 权限（permission）资源：`pc permission <operation>`。
//!
//! 对应 `/v1/permission/...` 的 REST 接口（开放平台文档中的「全局 › 权限」分组）：
//! 全部权限点定义、当前用户的全局 / Pilot / Principal 权限查询与批量检查。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `points.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`PermissionCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod check_global;
pub mod check_pilot;
pub mod check_principal;
pub mod my_global;
pub mod my_pilot;
pub mod my_principal;
pub mod points;

use check_global::CheckGlobalArgs;
use check_pilot::CheckPilotArgs;
use check_principal::CheckPrincipalArgs;
use my_global::MyGlobalArgs;
use my_pilot::MyPilotArgs;
use my_principal::MyPrincipalArgs;
use points::PointsArgs;

/// `pc permission` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum PermissionCommand {
    /// List all permission point definitions (GET /v1/permission/points)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionPoints
    Points(PointsArgs),

    /// Get my global permissions (GET /v1/permission/my/global)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyGlobal
    MyGlobal(MyGlobalArgs),

    /// Get my permissions on a pilot (GET /v1/permission/my/pilot)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyPilot
    MyPilot(MyPilotArgs),

    /// Get my permissions on a principal (GET /v1/permission/my/principal)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPermissionMyPrincipal
    MyPrincipal(MyPrincipalArgs),

    /// Check my global permissions for the given permission points (POST /v1/permission/check/global)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckGlobal
    CheckGlobal(CheckGlobalArgs),

    /// Check my permissions on multiple pilots (POST /v1/permission/check/pilot)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckPilot
    CheckPilot(CheckPilotArgs),

    /// Check my permissions on multiple principals (POST /v1/permission/check/principal)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPermissionCheckPrincipal
    CheckPrincipal(CheckPrincipalArgs),
}

pub async fn run(ctx: &Ctx, command: PermissionCommand) -> anyhow::Result<()> {
    match command {
        PermissionCommand::Points(args) => points::run(ctx, &args).await,
        PermissionCommand::MyGlobal(args) => my_global::run(ctx, &args).await,
        PermissionCommand::MyPilot(args) => my_pilot::run(ctx, &args).await,
        PermissionCommand::MyPrincipal(args) => my_principal::run(ctx, &args).await,
        PermissionCommand::CheckGlobal(args) => check_global::run(ctx, &args).await,
        PermissionCommand::CheckPilot(args) => check_pilot::run(ctx, &args).await,
        PermissionCommand::CheckPrincipal(args) => check_principal::run(ctx, &args).await,
    }
}
