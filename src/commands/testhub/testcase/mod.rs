//! 测试用例资源：`pc testhub testcase <operation>`。
//!
//! 对应 `/v1/testhub/testcases` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestcaseCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod bulk_create;
pub mod bulk_update;
pub mod create;
pub mod delete;
pub mod get;
pub mod histories;
pub mod list;
pub mod search;
pub mod update;

use bulk_create::BulkCreateArgs;
use bulk_update::BulkUpdateArgs;
use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use histories::HistoriesArgs;
use list::ListArgs;
use search::SearchArgs;
use update::UpdateArgs;

/// `pc testhub testcase` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestcaseCommand {
    /// List test cases (GET /v1/testhub/testcases)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcases
    List(ListArgs),
    /// Get a test case by id or short id (GET /v1/testhub/testcases/{testcase_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasesByTestcaseId
    Get(GetArgs),
    /// Create a test case (POST /v1/testhub/testcases)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcases
    Create(CreateArgs),
    /// Partially update a test case (PATCH /v1/testhub/testcases/{testcase_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasesByTestcaseId
    Update(UpdateArgs),
    /// Delete a test case (DELETE /v1/testhub/testcases/{testcase_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubTestcasesByTestcaseId
    Delete(DeleteArgs),
    /// Search test cases with structured filters (POST /v1/testhub/testcases/search)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasesSearch
    Search(SearchArgs),
    /// Bulk create test cases (POST /v1/testhub/testcases/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestcasesBulk
    BulkCreate(BulkCreateArgs),
    /// Bulk update test cases (PATCH /v1/testhub/testcases/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasesBulk
    BulkUpdate(BulkUpdateArgs),
    /// Get execution histories of a test case (GET /v1/testhub/testcases/{testcase_id}/histories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasesByTestcaseIdHistories
    Histories(HistoriesArgs),
}

pub async fn run(ctx: &Ctx, command: TestcaseCommand) -> anyhow::Result<()> {
    match command {
        TestcaseCommand::List(args) => list::run(ctx, &args).await,
        TestcaseCommand::Get(args) => get::run(ctx, &args).await,
        TestcaseCommand::Create(args) => create::run(ctx, &args).await,
        TestcaseCommand::Update(args) => update::run(ctx, &args).await,
        TestcaseCommand::Delete(args) => delete::run(ctx, &args).await,
        TestcaseCommand::Search(args) => search::run(ctx, &args).await,
        TestcaseCommand::BulkCreate(args) => bulk_create::run(ctx, &args).await,
        TestcaseCommand::BulkUpdate(args) => bulk_update::run(ctx, &args).await,
        TestcaseCommand::Histories(args) => histories::run(ctx, &args).await,
    }
}
