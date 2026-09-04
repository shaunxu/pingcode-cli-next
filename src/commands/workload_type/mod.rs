//! 工时类型（workload type）资源：`pc workload-type <operation>`。
//!
//! 对应 `/v1/workload_types` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `list.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`WorkloadTypeCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc workload-type` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum WorkloadTypeCommand {
    /// List workload types (GET /v1/workload_types)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloadTypes
    List(ListArgs),

    /// Get a workload type by id (GET /v1/workload_types/{type_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloadTypesByTypeId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: WorkloadTypeCommand) -> anyhow::Result<()> {
    match command {
        WorkloadTypeCommand::List(args) => list::run(ctx, &args).await,
        WorkloadTypeCommand::Get(args) => get::run(ctx, &args).await,
    }
}
