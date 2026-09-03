//! 执行用例（测试执行记录）资源：`pc testhub testrun <operation>`。
//!
//! 对应 `/v1/testhub/testruns` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TestrunCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod bulk_create;
pub mod bulk_update;
pub mod create;
pub mod get;
pub mod get_history;
pub mod histories;
pub mod list;
pub mod plan_batch;
pub mod replace;
pub mod search;
pub mod update;

use bulk_create::BulkCreateArgs;
use bulk_update::BulkUpdateArgs;
use create::CreateArgs;
use get::GetArgs;
use get_history::GetHistoryArgs;
use histories::HistoriesArgs;
use list::ListArgs;
use plan_batch::PlanBatchArgs;
use replace::ReplaceArgs;
use search::SearchArgs;
use update::UpdateArgs;

/// `pc testhub testrun` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TestrunCommand {
    /// List test runs (GET /v1/testhub/testruns)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestruns
    List(ListArgs),
    /// Get a test run by id or short id (GET /v1/testhub/testruns/{testrun_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunId
    Get(GetArgs),
    /// Create a test run (POST /v1/testhub/testruns)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestruns
    Create(CreateArgs),
    /// Partially update a test run result (PATCH /v1/testhub/testruns/{testrun_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestrunsByTestrunId
    Update(UpdateArgs),
    /// Fully replace a test run result and steps (PUT /v1/testhub/testruns/{testrun_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/putTesthubTestrunsByTestrunId
    Replace(ReplaceArgs),
    /// Search test runs with structured filters (POST /v1/testhub/testruns/search)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestrunsSearch
    Search(SearchArgs),
    /// Bulk create test runs (POST /v1/testhub/testruns/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubTestrunsBulk
    BulkCreate(BulkCreateArgs),
    /// Bulk update test runs (PATCH /v1/testhub/testruns/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestrunsBulk
    BulkUpdate(BulkUpdateArgs),
    /// List result histories of a test run (GET /v1/testhub/testruns/{testrun_id}/histories)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunIdHistories
    Histories(HistoriesArgs),
    /// Get a result history record (GET /v1/testhub/testruns/{testrun_id}/histories/{history_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestrunsByTestrunIdHistoriesByHistoryId
    GetHistory(GetHistoryArgs),
    /// Batch insert/update/delete test runs in a plan (POST /v1/testhub/libraries/{library_id}/testplans/{testplan_id}/testruns/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdTestplansByTestplanIdTestrunsBulk
    PlanBatch(PlanBatchArgs),
}

pub async fn run(ctx: &Ctx, command: TestrunCommand) -> anyhow::Result<()> {
    match command {
        TestrunCommand::List(args) => list::run(ctx, &args).await,
        TestrunCommand::Get(args) => get::run(ctx, &args).await,
        TestrunCommand::Create(args) => create::run(ctx, &args).await,
        TestrunCommand::Update(args) => update::run(ctx, &args).await,
        TestrunCommand::Replace(args) => replace::run(ctx, &args).await,
        TestrunCommand::Search(args) => search::run(ctx, &args).await,
        TestrunCommand::BulkCreate(args) => bulk_create::run(ctx, &args).await,
        TestrunCommand::BulkUpdate(args) => bulk_update::run(ctx, &args).await,
        TestrunCommand::Histories(args) => histories::run(ctx, &args).await,
        TestrunCommand::GetHistory(args) => get_history::run(ctx, &args).await,
        TestrunCommand::PlanBatch(args) => plan_batch::run(ctx, &args).await,
    }
}
