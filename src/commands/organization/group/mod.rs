//! 团队（group，用户组）资源：`pc organization group <operation>`。
//!
//! 对应 `/v1/directory/groups` 及其直接子路径的 REST 接口。
//! 团队成员管理见 `group_member`（`pc organization group-member`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`GroupCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc organization group` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum GroupCommand {
    /// List teams/groups (GET /v1/directory/groups)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroups
    List(Box<ListArgs>),

    /// Get a team/group by id (GET /v1/directory/groups/{group_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroupsByGroupId
    Get(GetArgs),

    /// Create a team/group (POST /v1/directory/groups)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryGroups
    Create(CreateArgs),

    /// Partially update a team/group (PATCH /v1/directory/groups/{group_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryGroupsByGroupId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: GroupCommand) -> anyhow::Result<()> {
    match command {
        GroupCommand::List(args) => list::run(ctx, &args).await,
        GroupCommand::Get(args) => get::run(ctx, &args).await,
        GroupCommand::Create(args) => create::run(ctx, &args).await,
        GroupCommand::Update(args) => update::run(ctx, &args).await,
    }
}
