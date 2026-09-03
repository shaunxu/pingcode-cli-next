//! 用例属性方案（属性与测试库的绑定配置）资源：`pc testhub testcase-property-plan <operation>`。
//!
//! 对应 `/v1/testhub/testcase_property_plans` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcasePropertyPlanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc testhub testcase-property-plan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcasePropertyPlanCommand {
    /// List test case property plans (GET /v1/testhub/testcase_property_plans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlans
    List(ListArgs),
    /// Get a test case property plan by id (GET /v1/testhub/testcase_property_plans/{property_plan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanId
    Get(GetArgs),
    /// List properties in a property plan (GET /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanIdTestcaseProperties
    ListProperties(ListPropertiesArgs),
    /// Get a property association in a plan (GET /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertyPlansByPropertyPlanIdTestcasePropertiesByPropertyId
    GetProperty(GetPropertyArgs),
    /// Add a property to a property plan (POST /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasePropertyPlansByPropertyPlanIdTestcaseProperties
    AddProperty(AddPropertyArgs),
    /// Remove a property from a property plan (DELETE /v1/testhub/testcase_property_plans/{property_plan_id}/testcase_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubTestcasePropertyPlansByPropertyPlanIdTestcasePropertiesByPropertyId
    RemoveProperty(RemovePropertyArgs),
}

pub async fn run(ctx: &Ctx, command: TestcasePropertyPlanCommand) -> anyhow::Result<()> {
    match command {
        TestcasePropertyPlanCommand::List(args) => list::run(ctx, &args).await,
        TestcasePropertyPlanCommand::Get(args) => get::run(ctx, &args).await,
        TestcasePropertyPlanCommand::ListProperties(args) => list_properties::run(ctx, &args).await,
        TestcasePropertyPlanCommand::GetProperty(args) => get_property::run(ctx, &args).await,
        TestcasePropertyPlanCommand::AddProperty(args) => add_property::run(ctx, &args).await,
        TestcasePropertyPlanCommand::RemoveProperty(args) => remove_property::run(ctx, &args).await,
    }
}
