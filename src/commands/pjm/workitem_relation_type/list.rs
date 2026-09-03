use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-relation-type list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {}

/// 获取工作项关联类型列表：`GET /v1/pjm/workitem_relation_types`
/// （分页，scope: `pcp:read:pjm:workitem`）。
///
/// 分页获取企业内全部工作项关联类型（含系统预设与自定义类型）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemRelationTypes
pub async fn run(ctx: &Ctx, _args: &ListArgs) -> anyhow::Result<()> {
    let response: Value = ctx.client.get("/v1/pjm/workitem_relation_types").await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
