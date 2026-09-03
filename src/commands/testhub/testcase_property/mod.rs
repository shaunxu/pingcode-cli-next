//! 用例属性（自定义字段）资源：`pc testhub testcase-property <operation>`。
//!
//! 对应 `/v1/testhub/testcase_properties` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcasePropertyCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod list_for_library;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_library::ListForLibraryArgs;
use update::UpdateArgs;

/// `pc testhub testcase-property` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcasePropertyCommand {
    /// List all test case properties in the enterprise (GET /v1/testhub/testcase_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseProperties
    List(ListArgs),
    /// List test case properties available in a library (GET /v1/testhub/testcase/properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertiesByLibraryId
    ListForLibrary(ListForLibraryArgs),
    /// Get a test case property by id (GET /v1/testhub/testcase_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertiesByPropertyId
    Get(GetArgs),
    /// Create a test case property (POST /v1/testhub/testcase_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcaseProperties
    Create(CreateArgs),
    /// Partially update a test case property (PATCH /v1/testhub/testcase_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasePropertiesByPropertyId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: TestcasePropertyCommand) -> anyhow::Result<()> {
    match command {
        TestcasePropertyCommand::List(args) => list::run(ctx, &args).await,
        TestcasePropertyCommand::ListForLibrary(args) => list_for_library::run(ctx, &args).await,
        TestcasePropertyCommand::Get(args) => get::run(ctx, &args).await,
        TestcasePropertyCommand::Create(args) => create::run(ctx, &args).await,
        TestcasePropertyCommand::Update(args) => update::run(ctx, &args).await,
    }
}
