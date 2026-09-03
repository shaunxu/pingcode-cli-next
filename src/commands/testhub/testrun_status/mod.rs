//! 执行结果状态（只读）资源：`pc testhub testrun-status <operation>`。
//!
//! 对应 `/v1/testhub/testrun_statuses` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestrunStatusCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_library;

use get::GetArgs;
use list::ListArgs;
use list_for_library::ListForLibraryArgs;

/// `pc testhub testrun-status` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestrunStatusCommand {
    /// List test run result statuses (GET /v1/testhub/testrun_statuses)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunStatuses
    List(ListArgs),
    /// List test run statuses available in a library (GET /v1/testhub/testrun/statuses)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunStatusesByLibraryId
    ListForLibrary(ListForLibraryArgs),
    /// Get a test run status by id (GET /v1/testhub/testrun_statuses/{status_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunStatusesByStatusId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestrunStatusCommand) -> anyhow::Result<()> {
    match command {
        TestrunStatusCommand::List(args) => list::run(ctx, &args).await,
        TestrunStatusCommand::ListForLibrary(args) => list_for_library::run(ctx, &args).await,
        TestrunStatusCommand::Get(args) => get::run(ctx, &args).await,
    }
}
