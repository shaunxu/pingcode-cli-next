//! 工作项状态方案（workitem state plan）资源：
//! `pc pjm workitem-state-plan <operation>`。
//!
//! 对应「工作项配置」中的状态方案 `/v1/pjm/workitem_state_plans` 及其
//! 直接子路径的 REST 接口，scope 为 `pcp:(read|write):pjm:configuration`。
//!
//! 方案本身只支持查询（list/get）；方案内有两类成员：
//! - **工作项状态**（`workitem_states`）：add-state / list-states / get-state /
//!   remove-state（不支持更新）；
//! - **状态流转**（`workitem_state_flows`）：add-flow / list-flows / get-flow /
//!   remove-flow（不支持更新）。
//!
//! 工作项状态字典本身的创建/修改见 `pc pjm workitem-state`。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkitemStatePlanCommand`] 枚举加一个变体，并在 [`run`] 的
//!    match 中加一行分发。

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

/// `pc pjm workitem-state-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkitemStatePlanCommand {
    /// List work item state plans (GET /v1/pjm/workitem_state_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlans
    List(ListArgs),

    /// Get a work item state plan by id (GET /v1/pjm/workitem_state_plans/{state_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanId
    Get(GetArgs),

    /// Add a work item state to a state plan (POST /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStatePlansByStatePlanIdWorkitemStates
    AddState(AddStateArgs),

    /// List work item states in a state plan (GET /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanIdWorkitemStates
    ListStates(ListStatesArgs),

    /// Get a work item state in a state plan (GET /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanIdWorkitemStatesByStateId
    GetState(GetStateArgs),

    /// Remove a work item state from a state plan (DELETE /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemStatePlansByStatePlanIdWorkitemStatesByStateId
    RemoveState(RemoveStateArgs),

    /// Add a state transition (flow) to a state plan (POST /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postPjmWorkitemStatePlansByStatePlanIdWorkitemStateFlows
    AddFlow(AddFlowArgs),

    /// List state transitions (flows) in a state plan (GET /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanIdWorkitemStateFlows
    ListFlows(ListFlowsArgs),

    /// Get a state transition (flow) in a state plan (GET /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows/{flow_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemStatePlansByStatePlanIdWorkitemStateFlowsByFlowId
    GetFlow(GetFlowArgs),

    /// Remove a state transition (flow) from a state plan (DELETE /v1/pjm/workitem_state_plans/{state_plan_id}/workitem_state_flows/{flow_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemStatePlansByStatePlanIdWorkitemStateFlowsByFlowId
    RemoveFlow(RemoveFlowArgs),
}

pub async fn run(ctx: &Ctx, command: WorkitemStatePlanCommand) -> anyhow::Result<()> {
    match command {
        WorkitemStatePlanCommand::List(args) => list::run(ctx, &args).await,
        WorkitemStatePlanCommand::Get(args) => get::run(ctx, &args).await,
        WorkitemStatePlanCommand::AddState(args) => add_state::run(ctx, &args).await,
        WorkitemStatePlanCommand::ListStates(args) => list_states::run(ctx, &args).await,
        WorkitemStatePlanCommand::GetState(args) => get_state::run(ctx, &args).await,
        WorkitemStatePlanCommand::RemoveState(args) => remove_state::run(ctx, &args).await,
        WorkitemStatePlanCommand::AddFlow(args) => add_flow::run(ctx, &args).await,
        WorkitemStatePlanCommand::ListFlows(args) => list_flows::run(ctx, &args).await,
        WorkitemStatePlanCommand::GetFlow(args) => get_flow::run(ctx, &args).await,
        WorkitemStatePlanCommand::RemoveFlow(args) => remove_flow::run(ctx, &args).await,
    }
}
