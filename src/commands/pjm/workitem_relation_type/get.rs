use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Relation type id
    #[arg(value_name = "RELATION_TYPE_ID")]
    pub relation_type_id: String,
}

/// 获取一个工作项关联类型：`GET /v1/pjm/workitem_relation_types/{relation_type_id}`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemRelationTypesByRelationTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_relation_types/{}", args.relation_type_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
