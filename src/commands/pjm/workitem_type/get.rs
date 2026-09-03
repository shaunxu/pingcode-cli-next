use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item type id
    #[arg(value_name = "WORKITEM_TYPE_ID")]
    pub workitem_type_id: String,
}

/// 获取一个工作项类型：`GET /v1/pjm/workitem_types/{workitem_type_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypesByWorkitemTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_types/{}", args.workitem_type_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
