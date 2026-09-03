use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-suite remove` 的参数。
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Suite id
    #[arg(value_name = "SUITE_ID")]
    pub suite_id: String,
}

/// 删除用例模块（级联删除其全部子模块）：`DELETE /v1/testhub/libraries/{library_id}/suites/{suite_id}`（scope: `pcp:write:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteTesthubLibrariesByLibraryIdSuitesBySuiteId
pub async fn run(ctx: &Ctx, args: &RemoveArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/libraries/{}/suites/{}",
        args.library_id, args.suite_id
    );
    let response: Value = ctx.client.delete(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
