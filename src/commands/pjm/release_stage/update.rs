use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-stage update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Release stage id
    #[arg(value_name = "RELEASE_STAGE_ID")]
    pub release_stage_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个发布阶段：`PATCH /v1/pjm/release_stages/{release_stage_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体可选 `name`、`type`（`pending` / `in_progress` / `published`）、
/// `color`，完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmReleaseStagesByReleaseStageId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/pjm/release_stages/{}", args.release_stage_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
