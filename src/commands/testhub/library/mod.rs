//! 测试库资源：`pc testhub library <operation>`。
//!
//! 对应 `/v1/testhub/libraries` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`LibraryCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc testhub library` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum LibraryCommand {
    /// List test libraries (GET /v1/testhub/libraries)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibraries
    List(ListArgs),
    /// Get a test library by id (GET /v1/testhub/libraries/{library_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryId
    Get(GetArgs),
    /// Create a test library (POST /v1/testhub/libraries)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibraries
    Create(CreateArgs),
    /// Partially update a test library (PATCH /v1/testhub/libraries/{library_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: LibraryCommand) -> anyhow::Result<()> {
    match command {
        LibraryCommand::List(args) => list::run(ctx, &args).await,
        LibraryCommand::Get(args) => get::run(ctx, &args).await,
        LibraryCommand::Create(args) => create::run(ctx, &args).await,
        LibraryCommand::Update(args) => update::run(ctx, &args).await,
    }
}
