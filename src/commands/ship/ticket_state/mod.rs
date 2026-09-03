//! 工单状态（ticket-state）资源：`pc ship ticket-state <operation>`。
//!
//! 对应 `/v1/ship/ticket_states` 的 REST 接口（scope 为 `pcp:(read|write):ship:configuration`）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`TicketStateCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod get;
pub mod list;
pub mod list_for_product;
pub mod update;

use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use list_for_product::ListForProductArgs;
use update::UpdateArgs;

/// `pc ship ticket-state` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum TicketStateCommand {
    /// List all 工单状态 (GET /v1/ship/ticket_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStates
    List(ListArgs),
    /// List 工单状态 in a product (GET /v1/ship/ticket/states?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatesByProductId
    ListForProduct(ListForProductArgs),
    /// Get a 工单状态 by id (GET /v1/ship/ticket_states/{ticket_state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipTicketStatesByTicketStateId
    Get(GetArgs),
    /// Create a 工单状态 (POST /v1/ship/ticket_states)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipTicketStates
    Create(CreateArgs),
    /// Partially update a 工单状态 (PATCH /v1/ship/ticket_states/{ticket_state_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipTicketStatesByTicketStateId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: TicketStateCommand) -> anyhow::Result<()> {
    match command {
        TicketStateCommand::List(args) => list::run(ctx, &args).await,
        TicketStateCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        TicketStateCommand::Get(args) => get::run(ctx, &args).await,
        TicketStateCommand::Create(args) => create::run(ctx, &args).await,
        TicketStateCommand::Update(args) => update::run(ctx, &args).await,
    }
}
