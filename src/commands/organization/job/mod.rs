//! 职位（job，职务）资源：`pc organization job <operation>`。
//!
//! 对应 `/v1/directory/jobs` 及其子路径的 REST 接口。只读。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`JobCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod get;
pub mod list;

use get::GetArgs;
use list::ListArgs;

/// `pc organization job` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum JobCommand {
    /// List enterprise jobs / job titles, including built-in and custom (GET /v1/directory/jobs)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryJobs
    List(Box<ListArgs>),

    /// Get a job by id (GET /v1/directory/jobs/{job_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryJobsByJobId
    Get(GetArgs),
}

pub async fn run(ctx: &Ctx, command: JobCommand) -> anyhow::Result<()> {
    match command {
        JobCommand::List(args) => list::run(ctx, &args).await,
        JobCommand::Get(args) => get::run(ctx, &args).await,
    }
}
