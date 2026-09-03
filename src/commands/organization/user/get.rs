use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization user get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Enterprise member id
    #[arg(value_name = "USER_ID")]
    pub user_id: String,
}

/// 获取一个企业成员：`GET /v1/directory/users/{user_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 按 id 获取企业成员全量信息（含部门、职位引用）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryUsersByUserId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/users/{}", args.user_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
