//! 工作项交付目标（deliverable）资源：`pc pjm deliverable <operation>`。
//!
//! 对应 `/v1/pjm/deliverables` 及其直接子路径的 REST 接口。
//! 交付目标仅适用于 waterfall / hybrid 项目的工作项。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`DeliverableCommand`] 枚举加一个变体，并在 [`run`] 的 match 中
//!    加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc pjm deliverable` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum DeliverableCommand {
    /// List work item deliverable targets (GET /v1/pjm/deliverables)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmDeliverables
    List(ListArgs),

    /// Get a deliverable target by id (GET /v1/pjm/deliverables/{deliverable_target_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmDeliverablesByDeliverableTargetId
    Get(GetArgs),

    /// Create a deliverable target (POST /v1/pjm/deliverables)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmDeliverables
    Create(CreateArgs),

    /// Partially update a deliverable target (PATCH /v1/pjm/deliverables/{deliverable_target_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmDeliverablesByDeliverableTargetId
    Update(UpdateArgs),

    /// Delete a deliverable target (DELETE /v1/pjm/deliverables/{deliverable_target_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmDeliverablesByDeliverableTargetId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: DeliverableCommand) -> anyhow::Result<()> {
    match command {
        DeliverableCommand::List(args) => list::run(ctx, &args).await,
        DeliverableCommand::Get(args) => get::run(ctx, &args).await,
        DeliverableCommand::Create(args) => create::run(ctx, &args).await,
        DeliverableCommand::Update(args) => update::run(ctx, &args).await,
        DeliverableCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
