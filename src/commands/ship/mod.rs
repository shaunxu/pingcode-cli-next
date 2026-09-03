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

use product::ProductCommand;
use product_channel::ProductChannelCommand;
use product_customer::ProductCustomerCommand;
use product_member::ProductMemberCommand;
use product_plan::ProductPlanCommand;
use product_suite::ProductSuiteCommand;
use product_tag::ProductTagCommand;
use product_ticket_type::ProductTicketTypeCommand;
use product_user::ProductUserCommand;

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
    }
}
