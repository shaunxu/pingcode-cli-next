use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem-tag delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,
}

/// 删除一个工作项标签：`DELETE /v1/pjm/workitem_tags/{tag_id}`
/// （scope: `pcp:write:pjm:configuration`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deletePjmWorkitemTagsByTagId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/pjm/workitem_tags/{}", args.tag_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
