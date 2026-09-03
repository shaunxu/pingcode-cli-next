use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Relation id
    #[arg(value_name = "RELATION_ID")]
    pub relation_id: String,
}

/// 取消关联一个工作项：`DELETE /v1/pjm/workitems/{workitem_id}/relations/{relation_id}`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 返回被删除的关联对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemIdRelationsByRelationId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitems/{}/relations/{}",
        args.workitem_id, args.relation_id
    );
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
