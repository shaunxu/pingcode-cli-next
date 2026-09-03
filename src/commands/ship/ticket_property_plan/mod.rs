//! 工单属性方案（ticket property plan）资源：`pc ship ticket-property-plan <operation>`。
//!
//! 对应「工单配置」中的属性方案 `/v1/ship/ticket_property_plans` 及其直接子路径
//! （方案内工单属性成员）的 REST 接口，scope 为 `pcp:(read|write):ship:configuration`。
//!
//! 方案本身只支持查询（list/get）；方案内的工单属性成员支持添加、查询与移除
//! （不支持更新）。工单属性字典本身的创建/修改见 `pc ship ticket-property`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketPropertyPlanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc ship ticket-property-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketPropertyPlanCommand {
    /// List ticket property plans (GET /v1/ship/ticket_property_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertyPlans
    List(ListArgs),

    /// Get a ticket property plan by id (GET /v1/ship/ticket_property_plans/{property_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertyPlansByPropertyPlanId
    Get(GetArgs),

    /// Add a ticket property to a property plan (POST /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketPropertyPlansByPropertyPlanIdTicketProperties
    AddProperty(AddPropertyArgs),

    /// List ticket properties in a property plan (GET /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertyPlansByPropertyPlanIdTicketProperties
    ListProperties(ListPropertiesArgs),

    /// Get a ticket property in a property plan (GET /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketPropertyPlansByPropertyPlanIdTicketPropertiesByPropertyId
    GetProperty(GetPropertyArgs),

    /// Remove a ticket property from a property plan (DELETE /v1/ship/ticket_property_plans/{property_plan_id}/ticket_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipTicketPropertyPlansByPropertyPlanIdTicketPropertiesByPropertyId
    RemoveProperty(RemovePropertyArgs),
}

pub async fn run(ctx: &Ctx, command: TicketPropertyPlanCommand) -> anyhow::Result<()> {
    match command {
        TicketPropertyPlanCommand::List(args) => list::run(ctx, &args).await,
        TicketPropertyPlanCommand::Get(args) => get::run(ctx, &args).await,
        TicketPropertyPlanCommand::AddProperty(args) => add_property::run(ctx, &args).await,
        TicketPropertyPlanCommand::ListProperties(args) => list_properties::run(ctx, &args).await,
        TicketPropertyPlanCommand::GetProperty(args) => get_property::run(ctx, &args).await,
        TicketPropertyPlanCommand::RemoveProperty(args) => remove_property::run(ctx, &args).await,
    }
}
