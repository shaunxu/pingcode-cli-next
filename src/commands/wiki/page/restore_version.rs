use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc wiki page restore-version` 的参数。
#[derive(Debug, Args)]
pub struct RestoreVersionArgs {
    /// Page id
    #[arg(value_name = "PAGE_ID")]
    pub page_id: String,

    /// Page version id to restore from
    #[arg(value_name = "VERSION_ID")]
    pub version_id: String,
}

/// 恢复一个页面版本：`POST /v1/wiki/pages/{page_id}/versions/{version_id}/restore`
/// （scope: `pcp:write:wiki:page`）。
///
/// 该接口无请求体字段，基于指定历史版本新建当前版本，返回新创建的版本对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postWikiPagesByPageIdVersionsByVersionIdRestore
pub async fn run(ctx: &Ctx, args: &RestoreVersionArgs) -> anyhow::Result<()> {
    let body = json!({});

    let path = format!(
        "/v1/wiki/pages/{}/versions/{}/restore",
        args.page_id, args.version_id
    );
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
