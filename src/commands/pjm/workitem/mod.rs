//! 工作项（workitem）资源：`pc pjm workitem <operation>`。
//!
//! 对应 `/v1/pjm/workitems` 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `create.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add_tag;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod get;
pub mod get_tag;
pub mod list;
pub mod remove_tag;
pub mod search;
pub mod update;

use add_tag::AddTagArgs;
use batch_update::BatchUpdateArgs;
use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use get_tag::GetTagArgs;
use list::ListArgs;
use remove_tag::RemoveTagArgs;
use search::SearchArgs;
use update::UpdateArgs;

/// `pc pjm workitem` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemCommand {
    /// List work items (GET /v1/pjm/workitems)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitems
    // 参数结构体较大（众多过滤项），装箱以避免枚举变体间体积差异过大。
    List(Box<ListArgs>),

    /// Get a work item by id or short id (GET /v1/pjm/workitems/{workitem_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemId
    Get(GetArgs),

    /// Create a work item (POST /v1/pjm/workitems)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitems
    Create(CreateArgs),

    /// Partially update a work item (PATCH /v1/pjm/workitems/{workitem_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemsByWorkitemId
    Update(UpdateArgs),

    /// Delete a work item (DELETE /v1/pjm/workitems/{workitem_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemId
    Delete(DeleteArgs),

    /// Search work items with structured filters (POST /v1/pjm/workitems/search)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsSearch
    Search(SearchArgs),

    /// Batch-update the same property on multiple work items (PATCH /v1/pjm/workitems)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitems
    BatchUpdate(BatchUpdateArgs),

    /// Add a tag to a work item (POST /v1/pjm/workitems/{workitem_id}/tags)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsByWorkitemIdTags
    AddTag(AddTagArgs),

    /// Get a tag on a work item (GET /v1/pjm/workitems/{workitem_id}/tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTagsByTagId
    GetTag(GetTagArgs),

    /// Remove a tag from a work item (DELETE /v1/pjm/workitems/{workitem_id}/tags/{tag_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemIdTagsByTagId
    RemoveTag(RemoveTagArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemCommand) -> anyhow::Result<()> {
    match command {
        WorkitemCommand::List(args) => list::run(ctx, &args).await,
        WorkitemCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemCommand::Update(args) => update::run(ctx, &args).await,
        WorkitemCommand::Delete(args) => delete::run(ctx, &args).await,
        WorkitemCommand::Search(args) => search::run(ctx, &args).await,
        WorkitemCommand::BatchUpdate(args) => batch_update::run(ctx, &args).await,
        WorkitemCommand::AddTag(args) => add_tag::run(ctx, &args).await,
        WorkitemCommand::GetTag(args) => get_tag::run(ctx, &args).await,
        WorkitemCommand::RemoveTag(args) => remove_tag::run(ctx, &args).await,
    }
}
