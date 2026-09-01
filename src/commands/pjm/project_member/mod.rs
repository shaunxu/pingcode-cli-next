//! 项目成员（project member）资源：`pc pjm project-member <operation>`。
//!
//! 对应 `/v1/pjm/projects/{project_id}/members` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ProjectMemberCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc pjm project-member` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum ProjectMemberCommand {
    /// List members of a project (GET /v1/pjm/projects/{project_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembers
    List(ListArgs),

    /// Get a project member by id (GET /v1/pjm/projects/{project_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdMembersByMemberId
    Get(GetArgs),

    /// Add a member to a project (POST /v1/pjm/projects/{project_id}/members)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmProjectsByProjectIdMembers
    Add(AddArgs),

    /// Partially update a project member (PATCH /v1/pjm/projects/{project_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdMembersByMemberId
    Update(UpdateArgs),

    /// Remove a member from a project (DELETE /v1/pjm/projects/{project_id}/members/{member_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmProjectsByProjectIdMembersByMemberId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: ProjectMemberCommand) -> anyhow::Result<()> {
    match command {
        ProjectMemberCommand::List(args) => list::run(ctx, &args).await,
        ProjectMemberCommand::Get(args) => get::run(ctx, &args).await,
        ProjectMemberCommand::Add(args) => add::run(ctx, &args).await,
        ProjectMemberCommand::Update(args) => update::run(ctx, &args).await,
        ProjectMemberCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
