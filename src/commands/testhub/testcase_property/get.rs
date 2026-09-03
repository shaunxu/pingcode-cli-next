use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-property get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Property id
    #[arg(value_name = "PROPERTY_ID")]
    pub property_id: String,
}

/// 获取一个用例属性：`GET /v1/testhub/testcase_properties/{property_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcasePropertiesByPropertyId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testcase_properties/{}", args.property_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
