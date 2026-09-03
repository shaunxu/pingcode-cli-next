//! 需求属性（idea property / 自定义字段）资源：`pc ship idea-property <operation>`。
//!
//! 对应 `/v1/ship/idea_properties` 与 `/v1/ship/idea/properties` 的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`IdeaPropertyCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

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

/// `pc ship idea-property` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum IdeaPropertyCommand {
    /// List all idea properties (GET /v1/ship/idea_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaProperties
    List(ListArgs),

    /// List idea properties in a product (GET /v1/ship/idea/properties?product_id=...)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPropertiesByProductId
    ListForProduct(ListForProductArgs),

    /// Get an idea property by id (GET /v1/ship/idea_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPropertiesByPropertyId
    Get(GetArgs),

    /// Create an idea property (POST /v1/ship/idea_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postShipIdeaProperties
    Create(CreateArgs),

    /// Partially update an idea property (PATCH /v1/ship/idea_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchShipIdeaPropertiesByPropertyId
    Update(UpdateArgs),
}

pub async fn run(ctx: &Ctx, command: IdeaPropertyCommand) -> anyhow::Result<()> {
    match command {
        IdeaPropertyCommand::List(args) => list::run(ctx, &args).await,
        IdeaPropertyCommand::ListForProduct(args) => list_for_product::run(ctx, &args).await,
        IdeaPropertyCommand::Get(args) => get::run(ctx, &args).await,
        IdeaPropertyCommand::Create(args) => create::run(ctx, &args).await,
        IdeaPropertyCommand::Update(args) => update::run(ctx, &args).await,
    }
}
