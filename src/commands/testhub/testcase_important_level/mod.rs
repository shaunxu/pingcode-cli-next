//! 用例重要程度（只读）资源：`pc testhub testcase-important-level <operation>`。
//!
//! 对应 `/v1/testhub/testcase_important_levels` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcaseImportantLevelCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc testhub testcase-important-level` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcaseImportantLevelCommand {
    /// List test case important levels (GET /v1/testhub/testcase_important_levels)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseImportantLevels
    List(ListArgs),
    /// Get a test case important level by id (GET /v1/testhub/testcase_important_levels/{important_level_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseImportantLevelsByImportantLevelId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestcaseImportantLevelCommand) -> anyhow::Result<()> {
    match command {
        TestcaseImportantLevelCommand::List(args) => list::run(ctx, &args).await,
        TestcaseImportantLevelCommand::Get(args) => get::run(ctx, &args).await,
    }
}
