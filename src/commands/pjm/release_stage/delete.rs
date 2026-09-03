use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-stage delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Release stage id
    #[arg(value_name = "RELEASE_STAGE_ID")]
    pub release_stage_id: String,

    /// Replacement release stage id; required when the stage is already used
    /// by a release. Request body as JSON: inline string, @file.json, or @-
    /// for stdin
    #[arg(long, value_name = "JSON")]
    pub data: Option<String>,
}

/// 删除一个发布阶段：`DELETE /v1/pjm/release_stages/{release_stage_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 若被删除的阶段已被发布使用，需通过请求体提供 `replace_id`
/// （替换阶段 id），引用该阶段的发布会被迁移到替换阶段；否则请求体可为空对象。
///
/// 返回被删除的发布阶段对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmReleaseStagesByReleaseStageId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    // 未提供 --data 时发送空对象；提供时按 JSON object 解析（如
    // {"replace_id": "<stage_id>"}）。
    let body = match &args.data {
        Some(spec) => output::ensure_object(output::read_data(spec)?)?,
        None => json!({}),
    };

    let path = format!("/v1/pjm/release_stages/{}", args.release_stage_id);
    let response: Value = ctx.client.delete_with_body(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
