use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc workload update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Workload id
    #[arg(value_name = "WORKLOAD_ID")]
    pub workload_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个工时：`PATCH /v1/workloads/{workload_id}`
/// （scope: `pcp:write:global:workload`）。
///
/// 用户令牌只能更新自己登记的工时，企业令牌不限。请求体可包含 `type_id`
/// （工时类型 id）、`duration`（时长，单位小时，0–24 之间、最多一位小数的
/// 正数）、`report_at`（登记日期，十位秒级时间戳，归一化到当天零点）、
/// `description`（工时说明）；工时主体、登记人等字段不可变更。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchWorkloadsByWorkloadId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/workloads/{}", args.workload_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
