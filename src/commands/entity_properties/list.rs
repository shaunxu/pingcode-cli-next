use clap::Args;
use serde_json::{json, Value};

use crate::commands::entity_properties::EntityType;
use crate::commands::Ctx;
use crate::output;

/// `pc entity-properties list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the entity the properties belong to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub entity_type: EntityType,

    /// Id of the entity (work item, idea, ticket, test case)
    #[arg(long, value_name = "ID")]
    pub entity_id: String,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取实体中的扩展属性列表：`GET /v1/entity_properties`
/// （分页，scope 依赖所属实体，如 workitem 需要 `pcp:read:pjm:workitem`）。
///
/// 查询参数：
/// - `entity_type`：实体类型（`workitem`/`idea`/`ticket`/`testcase`）；
/// - `entity_id`：实体 id；
/// - `page_index`/`page_size`：分页参数。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getEntityPropertiesByEntityTypeAndEntityId
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("entity_type".into(), json!(args.entity_type.as_str()));
    query.insert("entity_id".into(), json!(&args.entity_id));
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/entity_properties", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
