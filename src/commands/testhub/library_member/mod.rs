//! 测试库成员资源：`pc testhub library-member <operation>`。
//!
//! 对应 `/v1/testhub/libraries/{library_id}/members` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`LibraryMemberCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add;
pub mod get;
pub mod list;
pub mod remove;
pub mod update;

use add::AddArgs;
use get::GetArgs;
use list::ListArgs;
use remove::RemoveArgs;
use update::UpdateArgs;

/// `pc testhub library-member` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum LibraryMemberCommand {
    /// List members of a library (GET /v1/testhub/libraries/{library_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdMembers
    List(ListArgs),
    /// Get a library member by id (GET /v1/testhub/libraries/{library_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdMembersByMemberId
    Get(GetArgs),
    /// Add a member to a library (POST /v1/testhub/libraries/{library_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdMembers
    Add(AddArgs),
    /// Update a library member role (PATCH /v1/testhub/libraries/{library_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryIdMembersByMemberId
    Update(UpdateArgs),
    /// Remove a member from a library (DELETE /v1/testhub/libraries/{library_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubLibrariesByLibraryIdMembersByMemberId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: LibraryMemberCommand) -> anyhow::Result<()> {
    match command {
        LibraryMemberCommand::List(args) => list::run(ctx, &args).await,
        LibraryMemberCommand::Get(args) => get::run(ctx, &args).await,
        LibraryMemberCommand::Add(args) => add::run(ctx, &args).await,
        LibraryMemberCommand::Update(args) => update::run(ctx, &args).await,
        LibraryMemberCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
