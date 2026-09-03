use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan-type get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Test plan type id
    #[arg(value_name = "TESTPLAN_TYPE_ID")]
    pub testplan_type_id: String,
}

/// 获取一个测试计划类型：`GET /v1/testhub/libraries/{library_id}/testplan_types/{testplan_type_id}`（scope: `pcp:read:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplanTypesByTestplanTypeId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/libraries/{}/testplan_types/{}",
        args.library_id, args.testplan_type_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
