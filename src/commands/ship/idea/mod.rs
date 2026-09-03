//! 需求（idea，反馈需求）资源：`pc ship idea <operation>`。
//!
//! 对应 `/v1/ship/ideas` 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod search;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use search::SearchArgs;
use update::UpdateArgs;

/// `pc ship idea` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaCommand {
    /// List ideas (GET /v1/ship/ideas)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeas
    List(Box<ListArgs>),

    /// Get an idea by id (GET /v1/ship/ideas/{idea_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeasByIdeaId
    Get(GetArgs),

    /// Create an idea (POST /v1/ship/ideas)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeas
    Create(CreateArgs),

    /// Partially update an idea (PATCH /v1/ship/ideas/{idea_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipIdeasByIdeaId
    Update(UpdateArgs),

    /// Search ideas with structured filters (POST /v1/ship/ideas/search)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeasSearch
    Search(SearchArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaCommand) -> anyhow::Result<()> {
    match command {
        IdeaCommand::List(args) => list::run(ctx, &args).await,
        IdeaCommand::Get(args) => get::run(ctx, &args).await,
        IdeaCommand::Create(args) => create::run(ctx, &args).await,
        IdeaCommand::Update(args) => update::run(ctx, &args).await,
        IdeaCommand::Search(args) => search::run(ctx, &args).await,
    }
}
