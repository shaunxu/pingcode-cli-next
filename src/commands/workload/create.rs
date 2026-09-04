use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc workload create` 的参数。
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 登记一个工时：`POST /v1/workloads`（scope: `pcp:write:global:workload`，
/// 同时依赖工时所属主体对应的写作用域，如 principal_type 为 workitem 时
/// 还需要 `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `principal_id`（工时主体 id）、`principal_type`
/// （`workitem`/`idea`/`ticket`/`testcase`）、`duration`（时长，单位小时，
/// 0–24 之间、最多一位小数的正数）、`report_at`（登记日期，十位秒级时间戳，
/// 会被归一化到当天零点）；使用企业令牌鉴权时还必填 `report_by_id`（登记人
/// 用户 id，用户令牌不传该字段，默认为当前用户）；可选 `type_id`（工时类型
/// id）、`recorded_at`（登记时间，字符串）、`description`（工时说明）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postWorkloads
pub async fn run(ctx: &Ctx, args: &CreateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/workloads", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
