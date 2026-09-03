//! 用例状态（只读）资源：`pc testhub testcase-state <operation>`。
//!
//! 对应 `/v1/testhub/testcase_states` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcaseStateCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_library;

use get::GetArgs;
use list::ListArgs;
use list_for_library::ListForLibraryArgs;

/// `pc testhub testcase-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcaseStateCommand {
    /// List all test case states (GET /v1/testhub/testcase_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseStates
    List(ListArgs),
    /// List test case states available in a library (GET /v1/testhub/testcase/states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseStatesByLibraryId
    ListForLibrary(ListForLibraryArgs),
    /// Get a test case state by id (GET /v1/testhub/testcase_states/{state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseStatesByStateId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestcaseStateCommand) -> anyhow::Result<()> {
    match command {
        TestcaseStateCommand::List(args) => list::run(ctx, &args).await,
        TestcaseStateCommand::ListForLibrary(args) => list_for_library::run(ctx, &args).await,
        TestcaseStateCommand::Get(args) => get::run(ctx, &args).await,
    }
}
