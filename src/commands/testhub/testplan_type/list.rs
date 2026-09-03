use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan-type list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Library id
    #[arg(value_name = "LIBRARY_ID")]
    pub library_id: String,
}

/// 获取测试库下的测试计划类型列表：`GET /v1/testhub/libraries/{library_id}/testplan_types`（scope: `pcp:read:testhub:testplan`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubLibrariesByLibraryIdTestplanTypes
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let path = format!("/v1/testhub/libraries/{}/testplan_types", args.library_id);
    let response: Value = ctx.client.get(path.as_str()).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
