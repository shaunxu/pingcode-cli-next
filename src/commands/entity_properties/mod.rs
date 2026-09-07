//! 实体扩展属性（entity-properties）资源：`pc entity-properties <operation>`。
//!
//! 扩展属性用于在工作项（workitem）、需求（idea）、工单（ticket）、用例（testcase）
//! 等实体上存储自定义键值型 JSON 数据，对应 `/v1/entity_properties` 的 REST 接口，
//! 与评论、附件一样直接挂在命令顶层。
//!
//! 注意：扩展属性不参与 PingCode 属性管理，主要面向基于开放平台的应用集成/业务扩展；
//! 与各模块的自定义属性（properties）不同。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `create.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`EntityPropertiesCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::{Subcommand, ValueEnum};

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use list::ListArgs;

/// 扩展属性所属实体类型（查询/请求体参数 `entity_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum EntityType {
    /// Project management work item
    Workitem,
    /// Ship idea (requirement)
    Idea,
    /// Ship ticket
    Ticket,
    /// Testhub test case
    Testcase,
}

impl EntityType {
    /// 序列化为接口要求的 snake_case 字符串。
    pub fn as_str(self) -> &'static str {
        match self {
            EntityType::Workitem => "workitem",
            EntityType::Idea => "idea",
            EntityType::Ticket => "ticket",
            EntityType::Testcase => "testcase",
        }
    }
}

/// `pc entity-properties` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum EntityPropertiesCommand {
    /// List entity properties of an entity (GET /v1/entity_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getEntityPropertiesByEntityTypeAndEntityId
    List(ListArgs),

    /// Get an entity property by id (GET /v1/entity_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getEntityPropertiesByPropertyIdByEntityTypeAndEntityId
    Get(GetArgs),

    /// Add an entity property to an entity (POST /v1/entity_properties)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postEntityProperties
    Create(CreateArgs),

    /// Remove an entity property from an entity (DELETE /v1/entity_properties/{property_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteEntityPropertiesByPropertyIdByEntityTypeAndEntityId
    Delete(DeleteArgs),
}

pub async fn run(ctx: &Ctx, command: EntityPropertiesCommand) -> anyhow::Result<()> {
    match command {
        EntityPropertiesCommand::List(args) => list::run(ctx, &args).await,
        EntityPropertiesCommand::Get(args) => get::run(ctx, &args).await,
        EntityPropertiesCommand::Create(args) => create::run(ctx, &args).await,
        EntityPropertiesCommand::Delete(args) => delete::run(ctx, &args).await,
    }
}
