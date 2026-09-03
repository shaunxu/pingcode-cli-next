use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Test plan id or short id
    #[arg(value_name = "TESTPLAN_ID")]
    pub testplan_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新测试计划：`PATCH /v1/testhub/libraries/{library_id}/testplans/{testplan_id}`（scope: `pcp:write:testhub:testplan`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryIdTestplansByTestplanId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/testhub/libraries/{}/testplans/{}",
        args.library_id, args.testplan_id
    );
    let response: Value = ctx.client.patch(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
