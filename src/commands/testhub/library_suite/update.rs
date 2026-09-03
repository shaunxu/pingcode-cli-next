use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-suite update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Suite id
    #[arg(value_name = "SUITE_ID")]
    pub suite_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 更新用例模块（名称或父模块）：`PATCH /v1/testhub/libraries/{library_id}/suites/{suite_id}`（scope: `pcp:write:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryIdSuitesBySuiteId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/testhub/libraries/{}/suites/{}",
        args.library_id, args.suite_id
    );
    let response: Value = ctx.client.patch(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
