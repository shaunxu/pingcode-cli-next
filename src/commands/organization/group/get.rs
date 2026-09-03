use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization group get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Team/group id
    #[arg(value_name = "GROUP_ID")]
    pub group_id: String,
}

/// 获取一个团队：`GET /v1/directory/groups/{group_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 按 id 获取团队全量信息（id、url、name、visibility、description）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryGroupsByGroupId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/groups/{}", args.group_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
