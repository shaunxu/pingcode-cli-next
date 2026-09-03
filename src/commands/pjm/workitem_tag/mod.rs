//! 工作项标签（workitem tag）资源：`pc pjm workitem-tag <operation>`。
//!
//! 对应标签字典的 `/v1/pjm/workitem_tags` 及项目维度
//! `/v1/pjm/workitem/tags` 接口。给某个工作项加/取/删标签见
//! `pc pjm workitem add-tag|get-tag|remove-tag`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemTagCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
//!    加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod list_for_project;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_project::ListForProjectArgs;
use update::UpdateArgs;

/// `pc pjm workitem-tag` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemTagCommand {
    /// Create a work item tag (POST /v1/pjm/workitem_tags)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemTags
    Create(CreateArgs),

    /// List all work item tags (GET /v1/pjm/workitem_tags)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTags
    List(ListArgs),

    /// List work item tags in a project (GET /v1/pjm/workitem/tags?project_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTagsByProjectId
    ListForProject(ListForProjectArgs),

    /// Get a work item tag by id (GET /v1/pjm/workitem_tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTagsByTagId
    Get(GetArgs),

    /// Partially update a work item tag (PATCH /v1/pjm/workitem_tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemTagsByTagId
    Update(UpdateArgs),

    /// Delete a work item tag (DELETE /v1/pjm/workitem_tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTagsByTagId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemTagCommand) -> anyhow::Result<()> {
    match command {
        WorkitemTagCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemTagCommand::List(args) => list::run(ctx, &args).await,
        WorkitemTagCommand::ListForProject(args) => list_for_project::run(ctx, &args).await,
        WorkitemTagCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemTagCommand::Update(args) => update::run(ctx, &args).await,
        WorkitemTagCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
