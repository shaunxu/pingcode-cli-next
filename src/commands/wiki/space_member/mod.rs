//! 空间成员（space member）资源：`pc wiki space-member <operation>`。
//!
//! 对应 `/v1/wiki/spaces/{space_id}/members` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`SpaceMemberCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add;
pub mod get;
pub mod list;
pub mod remove;

use add::AddArgs;
use get::GetArgs;
use list::ListArgs;
use remove::RemoveArgs;

/// `pc wiki space-member` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum SpaceMemberCommand {
    /// List members of a wiki space (GET /v1/wiki/spaces/{space_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpacesBySpaceIdMembers
    List(ListArgs),

    /// Get a wiki space member by id (GET /v1/wiki/spaces/{space_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiSpacesBySpaceIdMembersByMemberId
    Get(GetArgs),

    /// Add a member to a wiki space (POST /v1/wiki/spaces/{space_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postWikiSpacesBySpaceIdMembers
    Add(AddArgs),

    /// Remove a member from a wiki space (DELETE /v1/wiki/spaces/{space_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiSpacesBySpaceIdMembersByMemberId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: SpaceMemberCommand) -> anyhow::Result<()> {
    match command {
        SpaceMemberCommand::List(args) => list::run(ctx, &args).await,
        SpaceMemberCommand::Get(args) => get::run(ctx, &args).await,
        SpaceMemberCommand::Add(args) => add::run(ctx, &args).await,
        SpaceMemberCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
