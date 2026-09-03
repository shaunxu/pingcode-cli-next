use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testcase-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Test case type id
    #[arg(value_name = "TYPE_ID")]
    pub type_id: String,
}

/// 获取一个用例类型：`GET /v1/testhub/testcase_types/{type_id}`（scope: `pcp:read:testhub:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestcaseTypesByTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/testcase_types/{}", args.type_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
