use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Work item type id
    #[arg(value_name = "WORKITEM_TYPE_ID")]
    pub workitem_type_id: String,
}

/// 删除一个工作项类型：`DELETE /v1/pjm/workitem_types/{workitem_type_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 属于「工作项配置」。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTypesByWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_types/{}", args.workitem_type_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
