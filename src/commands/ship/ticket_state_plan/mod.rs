//! 工单状态方案（ticket state plan）资源：`pc ship ticket-state-plan <operation>`。
//!
//! 对应「工单配置」中的状态方案 `/v1/ship/ticket_state_plans` 及其直接子路径的
//! REST 接口，scope 为 `pcp:(read|write):ship:configuration`。
//!
//! 方案本身只支持查询（list/get）；方案内有两类成员：
//! - **工单状态**（`ticket_states`）：add-state / list-states / get-state / remove-state；
//! - **状态流转**（`ticket_state_flows`）：add-flow / list-flows / get-flow / remove-flow。
//!
//! 工单状态字典本身的创建/修改见 `pc ship ticket-state`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketStatePlanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod add_flow;
pub mod add_state;
pub mod get;
pub mod get_flow;
pub mod get_state;
pub mod list;
pub mod list_flows;
pub mod list_states;
pub mod remove_flow;
pub mod remove_state;

use add_flow::AddFlowArgs;
use add_state::AddStateArgs;
use get::GetArgs;
use get_flow::GetFlowArgs;
use get_state::GetStateArgs;
use list::ListArgs;
use list_flows::ListFlowsArgs;
use list_states::ListStatesArgs;
use remove_flow::RemoveFlowArgs;
use remove_state::RemoveStateArgs;

/// `pc ship ticket-state-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketStatePlanCommand {
    /// List ticket state plans (GET /v1/ship/ticket_state_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlans
    List(ListArgs),

    /// Get a ticket state plan by id (GET /v1/ship/ticket_state_plans/{state_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanId
    Get(GetArgs),

    /// Add a ticket state to a state plan (POST /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketStatePlansByStatePlanIdTicketStates
    AddState(AddStateArgs),

    /// List ticket states in a state plan (GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStates
    ListStates(ListStatesArgs),

    /// Get a ticket state in a state plan (GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStatesByStateId
    GetState(GetStateArgs),

    /// Remove a ticket state from a state plan (DELETE /v1/ship/ticket_state_plans/{state_plan_id}/ticket_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipTicketStatePlansByStatePlanIdTicketStatesByStateId
    RemoveState(RemoveStateArgs),

    /// Add a state transition (flow) to a state plan (POST /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketStatePlansByStatePlanIdTicketStateFlows
    AddFlow(AddFlowArgs),

    /// List state transitions (flows) in a state plan (GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStateFlows
    ListFlows(ListFlowsArgs),

    /// Get a state transition (flow) in a state plan (GET /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows/{state_flow_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatePlansByStatePlanIdTicketStateFlowsByStateFlowId
    GetFlow(GetFlowArgs),

    /// Remove a state transition (flow) from a state plan (DELETE /v1/ship/ticket_state_plans/{state_plan_id}/ticket_state_flows/{state_flow_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteShipTicketStatePlansByStatePlanIdTicketStateFlowsByStateFlowId
    RemoveFlow(RemoveFlowArgs),
}

pub async fn run(ctx: &Ctx, command: TicketStatePlanCommand) -> anyhow::Result<()> {
    match command {
        TicketStatePlanCommand::List(args) => list::run(ctx, &args).await,
        TicketStatePlanCommand::Get(args) => get::run(ctx, &args).await,
        TicketStatePlanCommand::AddState(args) => add_state::run(ctx, &args).await,
        TicketStatePlanCommand::ListStates(args) => list_states::run(ctx, &args).await,
        TicketStatePlanCommand::GetState(args) => get_state::run(ctx, &args).await,
        TicketStatePlanCommand::RemoveState(args) => remove_state::run(ctx, &args).await,
        TicketStatePlanCommand::AddFlow(args) => add_flow::run(ctx, &args).await,
        TicketStatePlanCommand::ListFlows(args) => list_flows::run(ctx, &args).await,
        TicketStatePlanCommand::GetFlow(args) => get_flow::run(ctx, &args).await,
        TicketStatePlanCommand::RemoveFlow(args) => remove_flow::run(ctx, &args).await,
    }
}
