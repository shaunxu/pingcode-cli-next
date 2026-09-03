//! 部门（department）资源：`pc organization department <operation>`。
//!
//! 对应 `/v1/directory/departments` 及其子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`DepartmentCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc organization department` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum DepartmentCommand {
    /// List departments (GET /v1/directory/departments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryDepartments
    List(Box<ListArgs>),

    /// Get a department by id (GET /v1/directory/departments/{department_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryDepartmentsByDepartmentId
    Get(GetArgs),

    /// Create a department (POST /v1/directory/departments)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryDepartments
    Create(CreateArgs),

    /// Partially update a department (PATCH /v1/directory/departments/{department_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryDepartmentsByDepartmentId
    Update(UpdateArgs),

    /// Delete a department (DELETE /v1/directory/departments/{department_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteDirectoryDepartmentsByDepartmentId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: DepartmentCommand) -> anyhow::Result<()> {
    match command {
        DepartmentCommand::List(args) => list::run(ctx, &args).await,
        DepartmentCommand::Get(args) => get::run(ctx, &args).await,
        DepartmentCommand::Create(args) => create::run(ctx, &args).await,
        DepartmentCommand::Update(args) => update::run(ctx, &args).await,
        DepartmentCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
