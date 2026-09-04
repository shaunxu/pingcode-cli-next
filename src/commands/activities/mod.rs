//! 活动记录（activities）资源：`pc activities <operation>`。
//!
//! 活动记录是跨模块的只读通用资源（工作项、测试用例/测试计划、需求、工单的
//! 操作历史），对应 `/v1/activities` 的 REST 接口，因此与评论、工时一样
//! 直接挂在命令顶层。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ActivitiesCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// 活动记录主体类型（查询参数 `principal_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Testhub test run
    Testrun,
    /// Testhub test case
    Testcase,
    /// Ship idea (requirement)
    Idea,
    /// Ship ticket
    Ticket,
}

impl PrincipalType {
    /// 序列化为接口要求的 snake_case 字符串。
    pub fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Workitem => "workitem",
            PrincipalType::Testrun => "testrun",
            PrincipalType::Testcase => "testcase",
            PrincipalType::Idea => "idea",
            PrincipalType::Ticket => "ticket",
        }
    }
}

/// `pc activities` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum ActivitiesCommand {
    /// List activity records of a principal (GET /v1/activities)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getActivitiesByPrincipalTypeAndPrincipalId
    List(ListArgs),

    /// Get an activity record by id (GET /v1/activities/{activity_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getActivitiesByActivityId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: ActivitiesCommand) -> anyhow::Result<()> {
    match command {
        ActivitiesCommand::List(args) => list::run(ctx, &args).await,
        ActivitiesCommand::Get(args) => get::run(ctx, &args).await,
    }
}
