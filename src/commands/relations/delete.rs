use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc relations delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Relation id
    #[arg(value_name = "RELATION_ID")]
    pub relation_id: String,
}

/// 删除一个关联：`DELETE /v1/relations/{relation_id}`（scope 依赖关联主体和
/// 目标的作用域）。返回被删除的关联对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteRelationsByRelationId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/relations/{}", args.relation_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
