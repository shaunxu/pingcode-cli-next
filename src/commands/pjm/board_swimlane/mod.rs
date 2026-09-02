//! 泳道（board swimlane）资源：`pc pjm board-swimlane <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes`
//! 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`BoardSwimlaneCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm board-swimlane` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum BoardSwimlaneCommand {
    /// List swimlanes of a board (GET /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardIdSwimlanes
    List(ListArgs),

    /// Get a swimlane by id (GET /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes/{swimlane_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardIdSwimlanesBySwimlaneId
    Get(GetArgs),

    /// Create a swimlane (POST /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdBoardsByBoardIdSwimlanes
    Create(CreateArgs),

    /// Partially update a swimlane (PATCH /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes/{swimlane_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdBoardsByBoardIdSwimlanesBySwimlaneId
    Update(UpdateArgs),

    /// Delete a swimlane (DELETE /v1/pjm/projects/{project_id}/boards/{board_id}/swimlanes/{swimlane_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdBoardsByBoardIdSwimlanesBySwimlaneId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: BoardSwimlaneCommand) -> anyhow::Result<()> {
    match command {
        BoardSwimlaneCommand::List(args) => list::run(ctx, &args).await,
        BoardSwimlaneCommand::Get(args) => get::run(ctx, &args).await,
        BoardSwimlaneCommand::Create(args) => create::run(ctx, &args).await,
        BoardSwimlaneCommand::Update(args) => update::run(ctx, &args).await,
        BoardSwimlaneCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
