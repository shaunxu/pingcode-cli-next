//! 工单（ticket）资源：`pc ship ticket <operation>`。
//!
//! 对应 `/v1/ship/tickets` 及其直接子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc ship ticket` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketCommand {
    /// List tickets (GET /v1/ship/tickets)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTickets
    List(Box<ListArgs>),

    /// Get a ticket by id (GET /v1/ship/tickets/{ticket_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketsByTicketId
    Get(GetArgs),

    /// Create a ticket (POST /v1/ship/tickets)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTickets
    Create(CreateArgs),

    /// Partially update a ticket (PATCH /v1/ship/tickets/{ticket_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipTicketsByTicketId
    Update(UpdateArgs),

    /// Search tickets with structured filters (POST /v1/ship/tickets/search)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketsSearch
    Search(SearchArgs),
}

pub async fn run(ctx: &Ctx, command: TicketCommand) -> anyhow::Result<()> {
    match command {
        TicketCommand::List(args) => list::run(ctx, &args).await,
        TicketCommand::Get(args) => get::run(ctx, &args).await,
        TicketCommand::Create(args) => create::run(ctx, &args).await,
        TicketCommand::Update(args) => update::run(ctx, &args).await,
        TicketCommand::Search(args) => search::run(ctx, &args).await,
    }
}
