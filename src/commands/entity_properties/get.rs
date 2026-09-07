use clap::Args;
use serde_json::{json, Value};

use crate::commands::entity_properties::EntityType;
use crate::commands::Ctx;
use crate::output;

/// `pc entity-properties get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Entity property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,

    /// Type of the entity the property belongs to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub entity_type: EntityType,

    /// Id of the entity (work item, idea, ticket, test case)
    #[arg(long, value_name = "ID")]
    pub entity_id: String,
}

/// 获取实体中的一个扩展属性：`GET /v1/entity_properties/{property_id}`
/// （scope 依赖所属实体，如 workitem 需要 `pcp:read:pjm:workitem`）。
///
/// 查询参数 `entity_type`（实体类型：`workitem`/`idea`/`ticket`/`testcase`）、
/// `entity_id`（实体 id）必填。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getEntityPropertiesByPropertyIdByEntityTypeAndEntityId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("entity_type".into(), json!(args.entity_type.as_str()));
    query.insert("entity_id".into(), json!(&args.entity_id));

    let path = format!("/v1/entity_properties/{}", args.property_id);
    let response: Value = ctx
        .client
        .get_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
