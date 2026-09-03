use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization user create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 创建一个企业成员：`POST /v1/directory/users`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体必填 `name`（企业内唯一）、`display_name`；`email` 与 `mobile`
/// 至少提供一个（均企业内唯一）；可选 `password`（6～200 字符）、
/// `department_id`、`job_id`、`employee_number`。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryUsers
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/directory/users", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
