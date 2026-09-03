//! 企业成员（user）资源：`pc organization user <operation>`。
//!
//! 对应 `/v1/directory/users` 及其子路径的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`UserCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod bulk_update;
pub mod create;
pub mod get;
pub mod list;
pub mod update;

use bulk_update::BulkUpdateArgs;
use create::CreateArgs;
use get::GetArgs;
use list::ListArgs;
use update::UpdateArgs;

/// `pc organization user` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum UserCommand {
    /// List enterprise members (GET /v1/directory/users)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryUsers
    List(Box<ListArgs>),

    /// Get an enterprise member by id (GET /v1/directory/users/{user_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getDirectoryUsersByUserId
    Get(GetArgs),

    /// Create an enterprise member (POST /v1/directory/users)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postDirectoryUsers
    Create(CreateArgs),

    /// Partially update an enterprise member (PATCH /v1/directory/users/{user_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryUsersByUserId
    Update(UpdateArgs),

    /// Bulk update one property of many enterprise members (PATCH /v1/directory/users/bulk)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryUsersBulk
    BulkUpdate(BulkUpdateArgs),
}

pub async fn run(ctx: &Ctx, command: UserCommand) -> anyhow::Result<()> {
    match command {
        UserCommand::List(args) => list::run(ctx, &args).await,
        UserCommand::Get(args) => get::run(ctx, &args).await,
        UserCommand::Create(args) => create::run(ctx, &args).await,
        UserCommand::Update(args) => update::run(ctx, &args).await,
        UserCommand::BulkUpdate(args) => bulk_update::run(ctx, &args).await,
    }
}
