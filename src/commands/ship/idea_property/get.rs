use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-property get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Idea property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 获取一个需求属性：`GET /v1/ship/idea_properties/{property_id}`（scope: `pcp:read:ship:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getShipIdeaPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/ship/idea_properties/{property_id}",
        property_id = args.property_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
