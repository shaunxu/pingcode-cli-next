use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-tag get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,
}

/// 获取一个工作项标签：`GET /v1/pjm/workitem_tags/{tag_id}`
/// （scope: `pcp:read:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemTagsByTagId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_tags/{}", args.tag_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
