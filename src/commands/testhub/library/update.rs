use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新测试库：`PATCH /v1/testhub/libraries/{library_id}`（scope: `pcp:write:testhub:library`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchTesthubLibrariesByLibraryId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/testhub/libraries/{}", args.library_id);
    let response: Value = ctx.client.patch(path.as_str(), &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
