use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem get-tag` 的参数。
#[derive(Debug, Args)]
pub struct GetTagArgs {
    /// Work item id
    #[arg(value_name = "WORKITEM_ID")]
    pub workitem_id: String,

    /// Tag id
    #[arg(value_name = "TAG_ID")]
    pub tag_id: String,
}

/// 获取工作项中的一个标签：`GET /v1/pjm/workitems/{workitem_id}/tags/{tag_id}`
/// （scope: `pcp:read:pjm:workitem`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitemsByWorkitemIdTagsByTagId
pub async fn run(ctx: &Ctx, args: &GetTagArgs) -> anyhow::Result<()> {
    let path = format!(
        "/v1/pjm/workitems/{}/tags/{}",
        args.workitem_id, args.tag_id
    );
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
