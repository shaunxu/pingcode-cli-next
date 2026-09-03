//! 测试计划类型资源：`pc testhub testplan-type <operation>`。
//!
//! 对应 `/v1/testhub/libraries/{library_id}/testplan_types` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestplanTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc testhub testplan-type` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestplanTypeCommand {
    /// List test plan types of a library (GET /v1/testhub/libraries/{library_id}/testplan_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplanTypes
    List(ListArgs),
    /// Get a test plan type by id (GET /v1/testhub/libraries/{library_id}/testplan_types/{testplan_type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplanTypesByTestplanTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: TestplanTypeCommand) -> anyhow::Result<()> {
    match command {
        TestplanTypeCommand::List(args) => list::run(ctx, &args).await,
        TestplanTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
