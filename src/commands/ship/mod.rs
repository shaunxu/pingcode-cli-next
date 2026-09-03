//! SHIP（产品管理）模块：`pc ship <resource> <operation>`。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/ship/` 下新建资源目录（如 `product/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`ShipCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod product;
pub mod product_channel;
pub mod product_customer;
pub mod product_member;
pub mod product_plan;
pub mod product_suite;
pub mod product_tag;
pub mod product_ticket_type;
pub mod product_user;
pub mod ticket;
pub mod ticket_channel;
pub mod ticket_priority;
pub mod ticket_property;
pub mod ticket_property_plan;
pub mod ticket_solution;
pub mod ticket_state;
pub mod ticket_state_plan;
pub mod ticket_tag;
pub mod ticket_transition;
pub mod ticket_type;

use product::ProductCommand;
use product_channel::ProductChannelCommand;
use product_customer::ProductCustomerCommand;
use product_member::ProductMemberCommand;
use product_plan::ProductPlanCommand;
use product_suite::ProductSuiteCommand;
use product_tag::ProductTagCommand;
use product_ticket_type::ProductTicketTypeCommand;
use product_user::ProductUserCommand;
use ticket::TicketCommand;
use ticket_channel::TicketChannelCommand;
use ticket_priority::TicketPriorityCommand;
use ticket_property::TicketPropertyCommand;
use ticket_property_plan::TicketPropertyPlanCommand;
use ticket_solution::TicketSolutionCommand;
use ticket_state::TicketStateCommand;
use ticket_state_plan::TicketStatePlanCommand;
use ticket_tag::TicketTagCommand;
use ticket_transition::TicketTransitionCommand;
use ticket_type::TicketTypeCommand;

/// `pc ship` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum ShipCommand {
    /// Products
    Product {
        #[command(subcommand)]
        command: ProductCommand,
    },
    /// Product members
    ProductMember {
        #[command(subcommand)]
        command: ProductMemberCommand,
    },
    /// Product requirement modules (suites)
    ProductSuite {
        #[command(subcommand)]
        command: ProductSuiteCommand,
    },
    /// Product tags
    ProductTag {
        #[command(subcommand)]
        command: ProductTagCommand,
    },
    /// Product external users
    ProductUser {
        #[command(subcommand)]
        command: ProductUserCommand,
    },
    /// Product customers
    ProductCustomer {
        #[command(subcommand)]
        command: ProductCustomerCommand,
    },
    /// Product ticket channels (read-only)
    ProductChannel {
        #[command(subcommand)]
        command: ProductChannelCommand,
    },
    /// Product requirement plans (read-only)
    ProductPlan {
        #[command(subcommand)]
        command: ProductPlanCommand,
    },
    /// Product ticket types (read-only)
    ProductTicketType {
        #[command(subcommand)]
        command: ProductTicketTypeCommand,
    },
    /// Tickets (customer feedback / support tickets)
    Ticket {
        #[command(subcommand)]
        command: TicketCommand,
    },
    /// Ticket transition histories (read-only)
    TicketTransition {
        #[command(subcommand)]
        command: TicketTransitionCommand,
    },
    /// Ticket types (ticket configuration)
    TicketType {
        #[command(subcommand)]
        command: TicketTypeCommand,
    },
    /// Ticket states (ticket configuration)
    TicketState {
        #[command(subcommand)]
        command: TicketStateCommand,
    },
    /// Ticket state plans and state transitions (ticket configuration)
    TicketStatePlan {
        #[command(subcommand)]
        command: TicketStatePlanCommand,
    },
    /// Ticket properties / custom fields (ticket configuration)
    TicketProperty {
        #[command(subcommand)]
        command: TicketPropertyCommand,
    },
    /// Ticket property plans (ticket configuration)
    TicketPropertyPlan {
        #[command(subcommand)]
        command: TicketPropertyPlanCommand,
    },
    /// Ticket priorities (read-only)
    TicketPriority {
        #[command(subcommand)]
        command: TicketPriorityCommand,
    },
    /// Ticket solutions (read-only)
    TicketSolution {
        #[command(subcommand)]
        command: TicketSolutionCommand,
    },
    /// Ticket tags (read-only, per product)
    TicketTag {
        #[command(subcommand)]
        command: TicketTagCommand,
    },
    /// Ticket channels (read-only, per product)
    TicketChannel {
        #[command(subcommand)]
        command: TicketChannelCommand,
    },
}

pub async fn run(ctx: &Ctx, command: ShipCommand) -> anyhow::Result<()> {
    match command {
        ShipCommand::Product { command } => product::run(ctx, command).await,
        ShipCommand::ProductMember { command } => product_member::run(ctx, command).await,
        ShipCommand::ProductSuite { command } => product_suite::run(ctx, command).await,
        ShipCommand::ProductTag { command } => product_tag::run(ctx, command).await,
        ShipCommand::ProductUser { command } => product_user::run(ctx, command).await,
        ShipCommand::ProductCustomer { command } => product_customer::run(ctx, command).await,
        ShipCommand::ProductChannel { command } => product_channel::run(ctx, command).await,
        ShipCommand::ProductPlan { command } => product_plan::run(ctx, command).await,
        ShipCommand::ProductTicketType { command } => product_ticket_type::run(ctx, command).await,
        ShipCommand::Ticket { command } => ticket::run(ctx, command).await,
        ShipCommand::TicketTransition { command } => ticket_transition::run(ctx, command).await,
        ShipCommand::TicketType { command } => ticket_type::run(ctx, command).await,
        ShipCommand::TicketState { command } => ticket_state::run(ctx, command).await,
        ShipCommand::TicketStatePlan { command } => ticket_state_plan::run(ctx, command).await,
        ShipCommand::TicketProperty { command } => ticket_property::run(ctx, command).await,
        ShipCommand::TicketPropertyPlan { command } => {
            ticket_property_plan::run(ctx, command).await
        }
        ShipCommand::TicketPriority { command } => ticket_priority::run(ctx, command).await,
        ShipCommand::TicketSolution { command } => ticket_solution::run(ctx, command).await,
        ShipCommand::TicketTag { command } => ticket_tag::run(ctx, command).await,
        ShipCommand::TicketChannel { command } => ticket_channel::run(ctx, command).await,
    }
}
