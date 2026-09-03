//! 用例模块（测试库下的用例文件夹树）资源：`pc testhub library-suite <operation>`。
//!
//! 对应 `/v1/testhub/libraries/{library_id}/suites` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`LibrarySuiteCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod remove;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use remove::RemoveArgs;
use update::UpdateArgs;

/// `pc testhub library-suite` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum LibrarySuiteCommand {
    /// List suites of a library (GET /v1/testhub/libraries/{library_id}/suites)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdSuites
    List(ListArgs),
    /// Get a suite by id (GET /v1/testhub/libraries/{library_id}/suites/{suite_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdSuitesBySuiteId
    Get(GetArgs),
    /// Create a suite (POST /v1/testhub/libraries/{library_id}/suites)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdSuites
    Create(CreateArgs),
    /// Partially update a suite (PATCH /v1/testhub/libraries/{library_id}/suites/{suite_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryIdSuitesBySuiteId
    Update(UpdateArgs),
    /// Delete a suite (DELETE /v1/testhub/libraries/{library_id}/suites/{suite_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubLibrariesByLibraryIdSuitesBySuiteId
    Remove(RemoveArgs),
}

pub async fn run(ctx: &Ctx, command: LibrarySuiteCommand) -> anyhow::Result<()> {
    match command {
        LibrarySuiteCommand::List(args) => list::run(ctx, &args).await,
        LibrarySuiteCommand::Get(args) => get::run(ctx, &args).await,
        LibrarySuiteCommand::Create(args) => create::run(ctx, &args).await,
        LibrarySuiteCommand::Update(args) => update::run(ctx, &args).await,
        LibrarySuiteCommand::Remove(args) => remove::run(ctx, &args).await,
    }
}
