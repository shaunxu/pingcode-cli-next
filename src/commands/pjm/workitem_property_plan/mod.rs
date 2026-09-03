//! 工作项属性方案（workitem property plan）资源：
//! `pc pjm workitem-property-plan <operation>`。
//!
//! 对应「工作项配置」中的属性方案 `/v1/pjm/workitem_property_plans` 及其
//! 直接子路径（方案内工作项属性成员）的 REST 接口，scope 为
//! `pcp:(read|write):pjm:configuration`。
//!
//! 方案本身只支持查询（list/get）；方案内的工作项属性成员支持添加、查询与
//! 移除（不支持更新）。工作项属性字典本身的创建/修改见
//! `pc pjm workitem-property`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemPropertyPlanCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add_property;
pub mod get;
pub mod get_property;
pub mod list;
pub mod list_properties;
pub mod remove_property;

use add_property::AddPropertyArgs;
use get::GetArgs;
use get_property::GetPropertyArgs;
use list::ListArgs;
use list_properties::ListPropertiesArgs;
use remove_property::RemovePropertyArgs;

/// `pc pjm workitem-property-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemPropertyPlanCommand {
    /// List work item property plans (GET /v1/pjm/workitem_property_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertyPlans
    List(ListArgs),

    /// Get a work item property plan by id (GET /v1/pjm/workitem_property_plans/{property_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertyPlansByPropertyPlanId
    Get(GetArgs),

    /// Add a work item property to a property plan (POST /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemPropertyPlansByPropertyPlanIdWorkitemProperties
    AddProperty(AddPropertyArgs),

    /// List work item properties in a property plan (GET /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertyPlansByPropertyPlanIdWorkitemProperties
    ListProperties(ListPropertiesArgs),

    /// Get a work item property in a property plan (GET /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertyPlansByPropertyPlanIdWorkitemPropertiesByPropertyId
    GetProperty(GetPropertyArgs),

    /// Remove a work item property from a property plan (DELETE /v1/pjm/workitem_property_plans/{property_plan_id}/workitem_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemPropertyPlansByPropertyPlanIdWorkitemPropertiesByPropertyId
    RemoveProperty(RemovePropertyArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemPropertyPlanCommand) -> anyhow::Result<()> {
    match command {
        WorkitemPropertyPlanCommand::List(args) => list::run(ctx, &args).await,
        WorkitemPropertyPlanCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemPropertyPlanCommand::AddProperty(args) => add_property::run(ctx, &args).await,
        WorkitemPropertyPlanCommand::ListProperties(args) => list_properties::run(ctx, &args).await,
        WorkitemPropertyPlanCommand::GetProperty(args) => get_property::run(ctx, &args).await,
        WorkitemPropertyPlanCommand::RemoveProperty(args) => remove_property::run(ctx, &args).await,
    }
}
