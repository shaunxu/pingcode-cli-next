use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc relations get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Relation id
    #[arg(value_name = "RELATION_ID")]
    pub relation_id: String,

    /// Include soft-deleted relations in the response
    #[arg(long)]
    pub include_deleted: bool,
}

/// 获取一个关联：`GET /v1/relations/{relation_id}`（scope 依赖关联主体和目标
/// 的作用域）。
///
/// 查询参数 `include_deleted`（布尔，默认 false）可在响应中包含已被删除的
/// 关联。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getRelationsByRelationId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }

    let path = format!("/v1/relations/{}", args.relation_id);
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
