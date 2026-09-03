//! 空间（space）资源：`pc wiki space <operation>`。
//!
//! 对应 `/v1/wiki/spaces` 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`SpaceCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc wiki space` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum SpaceCommand {
    /// List wiki spaces (GET /v1/wiki/spaces)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpaces
    List(Box<ListArgs>),

    /// Get a wiki space by id (GET /v1/wiki/spaces/{space_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpacesBySpaceId
    Get(GetArgs),

    /// Create a wiki space (POST /v1/wiki/spaces)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postWikiSpaces
    Create(CreateArgs),

    /// Partially update a wiki space (PATCH /v1/wiki/spaces/{space_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchWikiSpacesBySpaceId
    Update(UpdateArgs),

    /// Delete a wiki space (DELETE /v1/wiki/spaces/{space_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiSpacesBySpaceId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: SpaceCommand) -> anyhow::Result<()> {
    match command {
        SpaceCommand::List(args) => list::run(ctx, &args).await,
        SpaceCommand::Get(args) => get::run(ctx, &args).await,
        SpaceCommand::Create(args) => create::run(ctx, &args).await,
        SpaceCommand::Update(args) => update::run(ctx, &args).await,
        SpaceCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
