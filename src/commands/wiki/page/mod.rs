//! 页面（page）资源：`pc wiki page <operation>`。
//!
//! 对应 `/v1/wiki/pages` 及其直接子路径（正文 content、版本 versions）的 REST 接口。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件，定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`PageCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod create;
pub mod delete;
pub mod get;
pub mod get_content;
pub mod get_version;
pub mod list;
pub mod list_versions;
pub mod restore_version;
pub mod update;
pub mod update_content;

use create::CreateArgs;
use delete::DeleteArgs;
use get::GetArgs;
use get_content::GetContentArgs;
use get_version::GetVersionArgs;
use list::ListArgs;
use list_versions::ListVersionsArgs;
use restore_version::RestoreVersionArgs;
use update::UpdateArgs;
use update_content::UpdateContentArgs;

/// `pc wiki page` 的操作级子命令。
#[derive(Debug, Subcommand)]
pub enum PageCommand {
    /// List wiki pages (GET /v1/wiki/pages)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPages
    List(ListArgs),

    /// Get a wiki page by id (GET /v1/wiki/pages/{page_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageId
    Get(GetArgs),

    /// Create a wiki page (POST /v1/wiki/pages)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postWikiPages
    Create(CreateArgs),

    /// Partially update a wiki page (PATCH /v1/wiki/pages/{page_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/patchWikiPagesByPageId
    Update(UpdateArgs),

    /// Delete a wiki page (DELETE /v1/wiki/pages/{page_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/deleteWikiPagesByPageId
    Delete(DeleteArgs),

    /// Get a page's content (GET /v1/wiki/pages/{page_id}/content)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdContent
    GetContent(GetContentArgs),

    /// Update a page's content (PUT /v1/wiki/pages/{page_id}/content)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/putWikiPagesByPageIdContent
    UpdateContent(UpdateContentArgs),

    /// List a page's versions (GET /v1/wiki/pages/{page_id}/versions)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdVersions
    ListVersions(ListVersionsArgs),

    /// Get a page version by id (GET /v1/wiki/pages/{page_id}/versions/{version_id})
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/getWikiPagesByPageIdVersionsByVersionId
    GetVersion(GetVersionArgs),

    /// Restore a page version (POST /v1/wiki/pages/{page_id}/versions/{version_id}/restore)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postWikiPagesByPageIdVersionsByVersionIdRestore
    RestoreVersion(RestoreVersionArgs),
}

pub async fn run(ctx: &Ctx, command: PageCommand) -> anyhow::Result<()> {
    match command {
        PageCommand::List(args) => list::run(ctx, &args).await,
        PageCommand::Get(args) => get::run(ctx, &args).await,
        PageCommand::Create(args) => create::run(ctx, &args).await,
        PageCommand::Update(args) => update::run(ctx, &args).await,
        PageCommand::Delete(args) => delete::run(ctx, &args).await,
        PageCommand::GetContent(args) => get_content::run(ctx, &args).await,
        PageCommand::UpdateContent(args) => update_content::run(ctx, &args).await,
        PageCommand::ListVersions(args) => list_versions::run(ctx, &args).await,
        PageCommand::GetVersion(args) => get_version::run(ctx, &args).await,
        PageCommand::RestoreVersion(args) => restore_version::run(ctx, &args).await,
    }
}
