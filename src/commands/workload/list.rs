use anyhow::{bail, Result};
use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// 工时主体类型（查询参数 `principal_type`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PrincipalType {
    /// Project management work item
    Workitem,
    /// Ship idea (requirement)
    Idea,
    /// Ship ticket
    Ticket,
    /// Testhub test case
    Testcase,
}

impl PrincipalType {
    fn as_str(self) -> &'static str {
        match self {
            PrincipalType::Workitem => "workitem",
            PrincipalType::Idea => "idea",
            PrincipalType::Ticket => "ticket",
            PrincipalType::Testcase => "testcase",
        }
    }
}

/// `pc workload list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Type of the workload's principal; required when --pilot-id or --principal-id is set
    #[arg(long, value_enum)]
    pub principal_type: Option<PrincipalType>,

    /// Id of the project, product or test library the principal belongs to
    #[arg(long, value_name = "ID")]
    pub pilot_id: Option<String>,

    /// Id of the principal (work item, idea, ticket or test case)
    #[arg(long, value_name = "ID")]
    pub principal_id: Option<String>,

    /// Start of the report-date range as a Unix timestamp in seconds; must be used together with --end-at
    #[arg(long, value_name = "TIMESTAMP")]
    pub start_at: Option<u64>,

    /// End of the report-date range as a Unix timestamp in seconds; must be used together with --start-at
    #[arg(long, value_name = "TIMESTAMP")]
    pub end_at: Option<u64>,

    /// Filter by reporter user id
    #[arg(long, value_name = "ID")]
    pub report_by_id: Option<String>,

    /// Page index, starting from 0
    #[arg(long, value_name = "INDEX")]
    pub page_index: Option<u64>,

    /// Page size
    #[arg(long, value_name = "SIZE")]
    pub page_size: Option<u64>,
}

/// 获取工时列表：`GET /v1/workloads`（分页，scope: `pcp:read:global:workload`，
/// 同时依赖工时所属主体对应的作用域，如 workitem 需要 `pcp:read:pjm:workitem`）。
///
/// 查询参数：
/// - `principal_type`：工时主体类型（`workitem`/`idea`/`ticket`/`testcase`），
///   传 `pilot_id` 或 `principal_id` 时必填；不传时要求令牌具备全部四类主体的作用域；
/// - `pilot_id`：工时主体所在项目/产品/测试库的 id；以此过滤时
///   `start_at`–`end_at` 的跨度最长为 3 个月；
/// - `principal_id`：工时主体 id；
/// - `start_at`/`end_at`：登记日期范围（秒级时间戳，分别归一化到当天的起止时刻），
///   必须成对出现；
/// - `report_by_id`：登记人 id。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getWorkloads
pub async fn run(ctx: &Ctx, args: &ListArgs) -> Result<()> {
    if args.principal_type.is_none() && (args.pilot_id.is_some() || args.principal_id.is_some()) {
        bail!("--principal-type is required when --pilot-id or --principal-id is set");
    }
    if args.start_at.is_some() != args.end_at.is_some() {
        bail!("--start-at and --end-at must be provided together");
    }

    let mut query = serde_json::Map::new();
    if let Some(principal_type) = args.principal_type {
        query.insert("principal_type".into(), json!(principal_type.as_str()));
    }
    if let Some(pilot_id) = &args.pilot_id {
        query.insert("pilot_id".into(), json!(pilot_id));
    }
    if let Some(principal_id) = &args.principal_id {
        query.insert("principal_id".into(), json!(principal_id));
    }
    if let Some(start_at) = args.start_at {
        query.insert("start_at".into(), json!(start_at));
    }
    if let Some(end_at) = args.end_at {
        query.insert("end_at".into(), json!(end_at));
    }
    if let Some(report_by_id) = &args.report_by_id {
        query.insert("report_by_id".into(), json!(report_by_id));
    }
    if let Some(page_index) = args.page_index {
        query.insert("page_index".into(), json!(page_index));
    }
    if let Some(page_size) = args.page_size {
        query.insert("page_size".into(), json!(page_size));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/workloads", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
