//! 工时（workload）资源：`pc workload <operation>`。
//!
//! 对应 `/v1/workloads` 的 REST 接口。工时是跨产品的全局资源，
//! 可登记在项目管理的工作项、Ship 的需求/工单、测试管理的测试用例上。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `create.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkloadCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc workload` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkloadCommand {
    /// List workloads (GET /v1/workloads)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloads
    // 参数结构体较大（众多过滤项），装箱以避免枚举变体间体积差异过大。
    List(Box<ListArgs>),

    /// Get a workload by id (GET /v1/workloads/{workload_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloadsByWorkloadId
    Get(GetArgs),

    /// Create a workload (POST /v1/workloads)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postWorkloads
    Create(CreateArgs),

    /// Partially update a workload (PATCH /v1/workloads/{workload_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchWorkloadsByWorkloadId
    Update(UpdateArgs),

    /// Delete a workload (DELETE /v1/workloads/{workload_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteWorkloadsByWorkloadId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: WorkloadCommand) -> anyhow::Result<()> {
    match command {
        WorkloadCommand::List(args) => list::run(ctx, &args).await,
        WorkloadCommand::Get(args) => get::run(ctx, &args).await,
        WorkloadCommand::Create(args) => create::run(ctx, &args).await,
        WorkloadCommand::Update(args) => update::run(ctx, &args).await,
        WorkloadCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
