use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-property get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Work item property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 获取一个工作项属性：`GET /v1/pjm/workitem_properties/{property_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_properties/{}", args.property_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
