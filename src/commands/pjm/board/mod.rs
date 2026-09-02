//! 看板（board）资源：`pc pjm board <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/boards` 及其直接子路径的
//! REST 接口。看板栏（entries）与泳道（swimlanes）分别在
//! [`super::board_entry`] 与 [`super::board_swimlane`] 模块。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`BoardCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
//!    加一行分发。

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

/// `pc pjm board` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum BoardCommand {
    /// List boards of a project (GET /v1/pjm/projects/{project_id}/boards)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoards
    List(ListArgs),

    /// Get a board by id (GET /v1/pjm/projects/{project_id}/boards/{board_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardId
    Get(GetArgs),

    /// Create a board (POST /v1/pjm/projects/{project_id}/boards)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdBoards
    Create(CreateArgs),

    /// Partially update a board (PATCH /v1/pjm/projects/{project_id}/boards/{board_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdBoardsByBoardId
    Update(UpdateArgs),

    /// Delete a board (DELETE /v1/pjm/projects/{project_id}/boards/{board_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdBoardsByBoardId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: BoardCommand) -> anyhow::Result<()> {
    match command {
        BoardCommand::List(args) => list::run(ctx, &args).await,
        BoardCommand::Get(args) => get::run(ctx, &args).await,
        BoardCommand::Create(args) => create::run(ctx, &args).await,
        BoardCommand::Update(args) => update::run(ctx, &args).await,
        BoardCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
