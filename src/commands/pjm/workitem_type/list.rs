use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-type list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {}

/// 获取全部工作项类型列表：`GET /v1/pjm/workitem_types`（分页，
/// scope: `pcp:read:pjm:configuration`）。
///
/// 分页获取企业内全部工作项类型（9 种系统类型与自定义类型）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTypes
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/workitem_types").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
