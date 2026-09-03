//! 用例类型（只读）资源：`pc testhub testcase-type <operation>`。
//!
//! 对应 `/v1/testhub/testcase_types` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcaseTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;
pub mod list_for_library;

use get::GetArgs;
use list::ListArgs;
use list_for_library::ListForLibraryArgs;

/// `pc testhub testcase-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcaseTypeCommand {
    /// List all test case types (GET /v1/testhub/testcase_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseTypes
    List(ListArgs),
    /// List test case types available in a library (GET /v1/testhub/testcase/types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseTypesByLibraryId
    ListForLibrary(ListForLibraryArgs),
    /// Get a test case type by id (GET /v1/testhub/testcase_types/{type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseTypesByTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestcaseTypeCommand) -> anyhow::Result<()> {
    match command {
        TestcaseTypeCommand::List(args) => list::run(ctx, &args).await,
        TestcaseTypeCommand::ListForLibrary(args) => list_for_library::run(ctx, &args).await,
        TestcaseTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
