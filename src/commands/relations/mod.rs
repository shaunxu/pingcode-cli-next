//! 关联（relations）资源：`pc relations <operation>`。
//!
//! 关联是跨模块的通用资源（可把需求与工单等不同模块的实体互相关联），
//! 对应 `/v1/relations` 的 REST 接口，因此与评论、工时一样直接挂在命令顶层。
//!
//! 注意与 `pc pjm workitem-relation` 的区别：后者走
//! `/v1/pjm/workitems/{workitem_id}/relations`，只表达工作项之间的关联且
//! 区分关联类型（`relation_type`）；本资源走 `/v1/relations`，用
//! `principal_type`/`target_type` 表达跨模块实体关联，不区分关联类型。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`RelationsCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc relations` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum RelationsCommand {
    /// List relations of a principal (GET /v1/relations)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getRelationsByPrincipalTypeAndPrincipalIdAndTargetType
    List(ListArgs),

    /// Get a relation by id (GET /v1/relations/{relation_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getRelationsByRelationId
    Get(GetArgs),

    /// Create a relation between two principals (POST /v1/relations)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postRelations
    Create(CreateArgs),

    /// Delete a relation by id (DELETE /v1/relations/{relation_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteRelationsByRelationId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: RelationsCommand) -> anyhow::Result<()> {
    match command {
        RelationsCommand::List(args) => list::run(ctx, &args).await,
        RelationsCommand::Get(args) => get::run(ctx, &args).await,
        RelationsCommand::Create(args) => create::run(ctx, &args).await,
        RelationsCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
