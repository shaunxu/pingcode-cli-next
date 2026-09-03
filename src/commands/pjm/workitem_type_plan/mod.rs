//! 工作项类型方案（workitem type plan）资源：
//! `pc pjm workitem-type-plan <operation>`。
//!
//! 对应「工作项配置」中的类型方案 `/v1/pjm/workitem_type_plans` 及其
//! 直接子路径（方案内工作项类型成员）的 REST 接口，scope 为
//! `pcp:(read|write):pjm:configuration`。
//!
//! 方案本身只支持查询（list/get）；方案内的工作项类型成员支持增删改查。
//! 工作项类型字典本身的创建/修改/删除见 `pc pjm workitem-type`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemTypePlanCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add_type;
pub mod get;
pub mod get_type;
pub mod list;
pub mod list_types;
pub mod remove_type;
pub mod update_type;

use add_type::AddTypeArgs;
use get::GetArgs;
use get_type::GetTypeArgs;
use list::ListArgs;
use list_types::ListTypesArgs;
use remove_type::RemoveTypeArgs;
use update_type::UpdateTypeArgs;

/// `pc pjm workitem-type-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemTypePlanCommand {
    /// List work item type plans (GET /v1/pjm/workitem_type_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlans
    List(ListArgs),

    /// Get a work item type plan by id (GET /v1/pjm/workitem_type_plans/{workitem_type_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlansByWorkitemTypePlanId
    Get(GetArgs),

    /// Add a work item type to a type plan (POST /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypes
    AddType(AddTypeArgs),

    /// List work item types in a type plan (GET /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypes
    ListTypes(ListTypesArgs),

    /// Get a work item type in a type plan (GET /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
    GetType(GetTypeArgs),

    /// Partially update a work item type in a type plan (PATCH /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
    UpdateType(UpdateTypeArgs),

    /// Remove a work item type from a type plan (DELETE /v1/pjm/workitem_type_plans/{workitem_type_plan_id}/workitem_types/{workitem_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTypePlansByWorkitemTypePlanIdWorkitemTypesByWorkitemTypeId
    RemoveType(RemoveTypeArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemTypePlanCommand) -> anyhow::Result<()> {
    match command {
        WorkitemTypePlanCommand::List(args) => list::run(ctx, &args).await,
        WorkitemTypePlanCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemTypePlanCommand::AddType(args) => add_type::run(ctx, &args).await,
        WorkitemTypePlanCommand::ListTypes(args) => list_types::run(ctx, &args).await,
        WorkitemTypePlanCommand::GetType(args) => get_type::run(ctx, &args).await,
        WorkitemTypePlanCommand::UpdateType(args) => update_type::run(ctx, &args).await,
        WorkitemTypePlanCommand::RemoveType(args) => remove_type::run(ctx, &args).await,
    }
}
