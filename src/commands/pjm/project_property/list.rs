use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-property list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs;

/// 分页获取企业内全部项目属性（全局属性定义）：`GET /v1/pjm/project_properties`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectProperties
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/project_properties").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
