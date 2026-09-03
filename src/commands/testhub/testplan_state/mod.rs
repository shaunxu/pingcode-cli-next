//! 测试计划状态（只读）资源：`pc testhub testplan-state <operation>`。
//!
//! 对应 `/v1/testhub/testplan_states` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestplanStateCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc testhub testplan-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestplanStateCommand {
    /// List test plan states (GET /v1/testhub/testplan_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestplanStates
    List(ListArgs),
    /// Get a test plan state by id (GET /v1/testhub/testplan_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestplanStatesByStateId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestplanStateCommand) -> anyhow::Result<()> {
    match command {
        TestplanStateCommand::List(args) => list::run(ctx, &args).await,
        TestplanStateCommand::Get(args) => get::run(ctx, &args).await,
    }
}
