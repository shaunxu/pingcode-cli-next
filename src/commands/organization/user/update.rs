use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization user update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Enterprise member id
    #[arg(value_name = "USER_ID")]
    pub user_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个企业成员：`PATCH /v1/directory/users/{user_id}`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体可选 `name`、`display_name`、`email`、`mobile`、
/// `status`（`enabled` / `disabled`，禁用即停用成员）、
/// `employee_number`、`department_id`、`job_id`。
/// 开放平台未提供删除企业成员的端点。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryUsersByUserId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/directory/users/{}", args.user_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
