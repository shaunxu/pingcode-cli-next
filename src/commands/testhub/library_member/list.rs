use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub library-member list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
}

/// 分页获取测试库成员列表：`GET /v1/testhub/libraries/{library_id}/members`（scope: `pcp:read:testhub:library`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdMembers
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/libraries/{}/members", args.library_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
