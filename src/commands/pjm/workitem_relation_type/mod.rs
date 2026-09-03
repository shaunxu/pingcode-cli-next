//! 工作项关联类型（workitem relation type）资源（只读）：
//! `pc pjm workitem-relation-type <operation>`。
//!
//! 对应 `/v1/pjm/workitem_relation_types` 接口，提供企业内工作项
//! 关联类型（系统预设与自定义）的查询。创建/管理关联实例见
//! `pc pjm workitem-relation`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemRelationTypeCommand`] 枚举加一个变体，并在 [`run`]
//!    的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc pjm workitem-relation-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemRelationTypeCommand {
    /// List work item relation types (GET /v1/pjm/workitem_relation_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemRelationTypes
    List(ListArgs),

    /// Get a relation type by id (GET /v1/pjm/workitem_relation_types/{relation_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemRelationTypesByRelationTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemRelationTypeCommand) -> anyhow::Result<()> {
    match command {
        WorkitemRelationTypeCommand::List(args) => list::run(ctx, &args).await,
        WorkitemRelationTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
