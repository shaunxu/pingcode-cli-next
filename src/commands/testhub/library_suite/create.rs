use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-suite create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建用例模块：`POST /v1/testhub/libraries/{library_id}/suites`（scope: `pcp:write:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postTesthubLibrariesByLibraryIdSuites
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/testhub/libraries/{}/suites", args.library_id);
    let response: Value = ctx.client.post(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
