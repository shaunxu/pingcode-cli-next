//! 测试计划资源：`pc testhub testplan <operation>`。
//!
//! 对应 `/v1/testhub/libraries/{library_id}/testplans` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestplanCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc testhub testplan` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestplanCommand {
    /// List test plans of a library (GET /v1/testhub/libraries/{library_id}/testplans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplans
    List(ListArgs),
    /// Get a test plan by id or short id (GET /v1/testhub/libraries/{library_id}/testplans/{testplan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplansByTestplanId
    Get(GetArgs),
    /// Create a test plan (POST /v1/testhub/libraries/{library_id}/testplans)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdTestplans
    Create(CreateArgs),
    /// Partially update a test plan (PATCH /v1/testhub/libraries/{library_id}/testplans/{testplan_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryIdTestplansByTestplanId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: TestplanCommand) -> anyhow::Result<()> {
    match command {
        TestplanCommand::List(args) => list::run(ctx, &args).await,
        TestplanCommand::Get(args) => get::run(ctx, &args).await,
        TestplanCommand::Create(args) => create::run(ctx, &args).await,
        TestplanCommand::Update(args) => update::run(ctx, &args).await,
    }
}
