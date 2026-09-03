use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc wiki space create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个空间：`POST /v1/wiki/spaces`（scope: `pcp:write:wiki:space`）。
///
/// 请求体必填 `scope_type`（organization/user_group/user）、`name`、
/// `identifier`（企业内唯一，大写字母/数字/下划线/连接线，不超过 15 字符）；
/// 当 `scope_type` 为 `user_group` 时必填 `scope_id`（团队 id）；
/// 可选 `visibility`（public/private）、`description`、`members`。
/// 企业令牌不能创建 `scope_type` 为 `user` 的空间。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postWikiSpaces
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/wiki/spaces", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
