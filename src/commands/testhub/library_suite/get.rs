use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-suite get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Suite id
    #[arg(value_name = "SUITE_ID")]
    pub suite_id: String,
}

/// 获取一个用例模块：`GET /v1/testhub/libraries/{library_id}/suites/{suite_id}`（scope: `pcp:read:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdSuitesBySuiteId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/testhub/libraries/{}/suites/{}",
        args.library_id, args.suite_id
    );
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
