//! 工作项关联（workitem relation）资源：`pc pjm workitem-relation <operation>`。
//!
//! 对应 `/v1/pjm/workitems/{workitem_id}/relations` 及其直接子路径的
//! REST 接口。关联类型字典见 `pc pjm workitem-relation-type`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemRelationCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;

/// `pc pjm workitem-relation` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemRelationCommand {
    /// List relations of a work item (GET /v1/pjm/workitems/{workitem_id}/relations)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdRelations
    List(ListArgs),

    /// Get a relation by id (GET /v1/pjm/workitems/{workitem_id}/relations/{relation_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdRelationsByRelationId
    Get(GetArgs),

    /// Relate a work item to another (POST /v1/pjm/workitems/{workitem_id}/relations)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemsByWorkitemIdRelations
    Create(CreateArgs),

    /// Remove a relation (DELETE /v1/pjm/workitems/{workitem_id}/relations/{relation_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemIdRelationsByRelationId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemRelationCommand) -> anyhow::Result<()> {
    match command {
        WorkitemRelationCommand::List(args) => list::run(ctx, &args).await,
        WorkitemRelationCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemRelationCommand::Create(args) => create::run(ctx, &args).await,
        WorkitemRelationCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
