use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea-property update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Idea property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个需求属性：`PATCH /v1/ship/idea_properties/{property_id}`（scope: `pcp:write:ship:configuration`）。
///
/// 请求体可选 `name`、`options` 等，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipIdeaPropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;
    let path = format!(
        "/v1/ship/idea_properties/{property_id}",
        property_id = args.property_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
