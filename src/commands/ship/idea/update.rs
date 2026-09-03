use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc ship idea update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Idea id
    #[arg(value_name = "IDEA_ID")]
    pub idea_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个需求：`PATCH /v1/ship/ideas/{idea_id}`（scope: `pcp:write:ship:idea`）。
///
/// 请求体可选 `title`、`description`、`state_id`、`priority_id`、`assignee_id`、
/// `progress`（0 到 1 的两位小数）、`plan_at`/`real_at`（整体更新，
/// 含 `from`/`to`/`granularity`）、`plan_id`（产品排期 id）、`suite_id`
/// （产品模块 id）、`properties`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchShipIdeasByIdeaId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/ship/ideas/{}", args.idea_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
