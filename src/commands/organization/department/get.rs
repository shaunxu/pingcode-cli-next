use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization department get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Department id
    #[arg(value_name = "DEPARTMENT_ID")]
    pub department_id: String,
}

/// 获取一个部门：`GET /v1/directory/departments/{department_id}`
/// （scope: `pcp:read:global:team`）。
///
/// 按 id 获取部门全量信息，含负责人（head）与父部门（parent）引用。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryDepartmentsByDepartmentId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/departments/{}", args.department_id);
    let response: Value = ctx.client.get(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
