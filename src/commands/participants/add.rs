use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc participants add` 的参数。
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: principal_type, principal_id, type (user|user_group),
    /// participant_id (user id or team id).
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 添加一个关注人：`POST /v1/participants`（scope 依赖关注人所属主体，
/// 如 workitem 需要 `pcp:write:pjm:workitem`）。
///
/// 请求体必填 `principal_type`（主体类型：`workitem`/`workitem_review`/
/// `testcase`/`testcase_review`/`idea`/`idea_review`/`ticket`/`page`）、
/// `principal_id`（主体 id）、`type`（关注人类型：`user` 用户或
/// `user_group` 团队）、`participant_id`（用户 id 或团队 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postParticipants
pub async fn run(ctx: &Ctx, args: &AddArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let response: Value = ctx.client.post("/v1/participants", &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
