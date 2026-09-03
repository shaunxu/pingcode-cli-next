use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-stage create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个发布阶段：`POST /v1/pjm/release_stages`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 请求体必填 `name`（企业内唯一）、`type`（阶段类型：
/// `pending` / `in_progress` / `published`），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postPjmReleaseStages
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/pjm/release_stages", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
