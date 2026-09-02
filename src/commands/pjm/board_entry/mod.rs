//! 看板栏（board entry）资源：`pc pjm board-entry <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/boards/{board_id}/entries`
//! 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`BoardEntryCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
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

/// `pc pjm board-entry` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum BoardEntryCommand {
    /// List entries of a board (GET /v1/pjm/projects/{project_id}/boards/{board_id}/entries)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardIdEntries
    List(ListArgs),

    /// Get a board entry by id (GET /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
    Get(GetArgs),

    /// Create a board entry (POST /v1/pjm/projects/{project_id}/boards/{board_id}/entries)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdBoardsByBoardIdEntries
    Create(CreateArgs),

    /// Partially update a board entry (PATCH /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
    Update(UpdateArgs),

    /// Delete a board entry (DELETE /v1/pjm/projects/{project_id}/boards/{board_id}/entries/{entry_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdBoardsByBoardIdEntriesByEntryId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: BoardEntryCommand) -> anyhow::Result<()> {
    match command {
        BoardEntryCommand::List(args) => list::run(ctx, &args).await,
        BoardEntryCommand::Get(args) => get::run(ctx, &args).await,
        BoardEntryCommand::Create(args) => create::run(ctx, &args).await,
        BoardEntryCommand::Update(args) => update::run(ctx, &args).await,
        BoardEntryCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
