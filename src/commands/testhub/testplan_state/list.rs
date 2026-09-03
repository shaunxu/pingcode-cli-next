use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc testhub testplan-state list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {}

/// 分页获取测试计划状态列表：`GET /v1/testhub/testplan_states`（scope: `pcp:read:testhub:configuration`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getTesthubTestplanStates
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let _ = args;

    let response: Value = ctx.client.get("/v1/testhub/testplan_states").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
