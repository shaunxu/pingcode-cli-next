use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,
}

/// 删除一个工作项：`DELETE /v1/pjm/workitems/{workitem_id}`
/// （scope: `pcp:write:pjm:workitem`）。
///
/// 按 id 删除工作项，返回被删除的工作项对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemsByWorkitemId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitems/{}", args.workitem_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
