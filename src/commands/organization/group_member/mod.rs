//! 团队成员（group member）资源：`pc organization group-member <operation>`。
//!
//! 对应 `/v1/directory/groups/{group_id}/members` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`GroupMemberCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc organization group-member` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum GroupMemberCommand {
    /// List members of a team/group (GET /v1/directory/groups/{group_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroupsByGroupIdMembers
    List(ListArgs),

    /// Get a member of a team/group by id (GET /v1/directory/groups/{group_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroupsByGroupIdMembersByMemberId
    Get(GetArgs),

    /// Add a member to a team/group (POST /v1/directory/groups/{group_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryGroupsByGroupIdMembers
    Add(AddArgs),

    /// Remove a member from a team/group (DELETE /v1/directory/groups/{group_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteDirectoryGroupsByGroupIdMembersByMemberId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: GroupMemberCommand) -> anyhow::Result<()> {
    match command {
        GroupMemberCommand::List(args) => list::run(ctx, &args).await,
        GroupMemberCommand::Get(args) => get::run(ctx, &args).await,
        GroupMemberCommand::Add(args) => add::run(ctx, &args).await,
        GroupMemberCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
