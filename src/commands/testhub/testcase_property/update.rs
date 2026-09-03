use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 更新用例属性（options 为整列表替换，仅选择类属性可改）：`PATCH /v1/testhub/testcase_properties/{property_id}`（scope: `pcp:write:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubTestcasePropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/testhub/testcase_properties/{}", args.property_id);
    let response: Value = ctx.client.patch(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
