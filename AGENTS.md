# AGENTS.md

## 项目

`pc` — PingCode Open API 的命令行客户端。单个 Rust 二进制 crate（edition 2021，Rust 1.75+），无 workspace。

## 验证命令

提交/交付前按顺序运行（本地 CI，`scripts/test.sh`）：

```bash
./scripts/test.sh        # cargo fmt --check → clippy -D warnings → cargo test
```

也可单独用 cargo aliases（定义在 `.cargo/config.toml`）：

- `cargo lint` — clippy 全目标 + `-D warnings`（任何 warning 即失败）
- `cargo check-fmt` — 仅检查格式
- `./scripts/lint.sh` — 先 `cargo fmt` 写入再 clippy

单测：`cargo test`；单个集成测试：`cargo test --test cli <test_name>`。

## 提交信息规范

所有 commit message 必须符合 [Conventional Commits](https://www.conventionalcommits.org/)：`<type>[optional scope]: <description>`。

- 类型集：`feat`、`fix`、`docs`、`style`、`refactor`、`perf`、`test`、`build`、`ci`、`chore`、`revert`。示例：`feat(pjm): add workitem update command`。
- 校验工具为 Rust 原生的 [committed](https://github.com/crate-ci/committed)（`cargo install committed`），不引入 Node 工具链。
- **本地钩子**：克隆后运行一次 `./scripts/install-hooks.sh`（设置 `core.hooksPath=scripts/hooks`，钩子脚本在 `scripts/hooks/` 下随仓库版本化）。提交时 `commit-msg` 钩子即时校验；未安装 `committed` 时钩子只警告不阻断。`fixup!`/`squash!`/`wip!` 临时提交放行。
- **CI 兜底**：`.github/workflows/commitlint.yml` 在每个 pull request 上用 `crate-ci/committed` action 校验 PR 内全部 commit（不合规则 CI 失败）；push 到 main 不拦截。历史 commit 不受影响。

## 发布（维护者）

发布是**项目级工具**，不是 `pc` 的子命令（类似 `npm run release`），用 `scripts/release.sh` 触发（本地需先 `cargo install cargo-release cargo-dist`）：

```bash
./scripts/release.sh --dry-run        # 仅预览新版本号与 CHANGELOG 条目，零副作用（输出 JSON）
./scripts/release.sh                  # 自动按 Conventional Commits 计算 Semver 并发布
./scripts/release.sh --version 0.2.0  # 手动指定版本号
```

- 发版工具链分三层：
  - `tools/release.py`（纯 Python 3 标准库，与 `tools/search_nexus_docs.py` 同风格，单测为 `tools/test_release.py`：`python3 -m unittest tools/test_release.py -v`）：`compute` 子命令取最近 git tag、解析基线后的 commits 定版本（0.x 阶段 feat/BREAKING→minor、fix/perf→patch；1.0+ 按标准 Semver），支持 `--version` 手动指定与 `--dry-run` 预览；`changelog` 子命令是 cargo-release 的 pre-release-hook，依据 `PREV_VERSION..HEAD` 的提交重写 `CHANGELOG.md`（Keep a Changelog，只收录 feat/fix/perf 及 BREAKING）。
  - [cargo-release](https://github.com/crate-ci/cargo-release)（配置在根目录 `release.toml`）：`publish = false`（不上 crates.io）、bump `Cargo.toml`/`Cargo.lock`、运行 changelog hook、commit `chore(release): vX.Y.Z`、打 annotated tag `vX.Y.Z`、push。**不要设 `tag-prefix = "v"`**：默认 tag-name 是 `{{prefix}}v{{version}}`，根 crate prefix 为空，设了会得到 `vvX.Y.Z`。
  - [cargo-dist](https://github.com/axodotdev/cargo-dist)（配置在根目录 `dist-workspace.toml`，workflow 由 `dist generate` 生成到 `.github/workflows/release.yml`，**不要手改该文件**）：tag 推送后在三平台（`x86_64-unknown-linux-gnu`、`aarch64-apple-darwin`、`x86_64-pc-windows-msvc`）编译，产物为 `.tar.xz`/`.zip` + `.sha256`，创建 GitHub Release、发布 `pc-installer.sh`/`pc-installer.ps1` 一键安装脚本，并把 Homebrew formula 推送到独立 tap 仓库 `shaunxu/homebrew-tap`（用 secret `HOMEBREW_TAP_TOKEN`；`tap`/`publish-jobs` 配在 `dist-workspace.toml`）。
- 改了 `dist-workspace.toml` 后必须运行 `dist generate` 重新生成 workflow。
- 用户安装方式见 README「安装」：shell/PowerShell 一键脚本、`brew tap shaunxu/tap && brew install pc`、或直接下载 Release 资产。

## 代码约定

- **CLI 面向用户的输出文字一律用英文**：clap 帮助文本（`#[command(about)]` / `#[arg(help)]` / doc comment）、错误消息（`bail!` / `anyhow` / `thiserror` 的 `#[error(...)]`）、`println!` / `eprintln!` 输出、`.expect()` 消息。跟随现有 `cli.rs` / `config.rs` / `client/error.rs` 的写法，不要写成中文。代码内部的注释和 doc comment 仍可用中文。
- **命令分两类**：三级命令（`pc <module> <resource> <operation>`，如 `pc pjm workitem create`）按模块/资源目录组织；自由命令（如 `state`）放 `commands/dynamic/`。新增方式见下方"架构"。
- 全局参数（`--dry-run`、`-v` 等）在 `cli.rs` 的 `Cli` 上用 `global = true` 声明，子命令前后均可放置。**所有命令的返回结果统一以 pretty-print JSON 输出到 stdout**（用 `output::print_json`），没有人类可读文本模式，也没有 `--json` 开关。`--dry-run` 不换令牌、不发网络、允许无凭据；写操作的请求体统一通过 `--data` 传入（内联 / `@file` / `@-` stdin），用 `output::read_data` + `output::ensure_object` 解析。
- rustfmt：`max_width = 100`、Unix 换行（`rustfmt.toml`）。
- 错误处理：库层用 `thiserror`（`ClientError`），应用层用 `anyhow`；`main()` 返回 `anyhow::Result`。
- **每个命令入口必须写明对应的官方文档地址**：新建命令（包括仅创建入口、`run()` 还是 `todo!()` 桩的情况）时，在操作文件的 `run` 函数 doc comment 中写入该端点的 REST 方法/路径与文档页面 URL（形如 `文档：https://developer.alpha.pingcode.live/restapi/pingcode/<pageName>`），同时在资源/模块 `mod.rs` 的枚举变体 doc comment 中附同一 URL（`Docs: <url>`），方便后续实现时引用核对。样板见 `src/commands/pjm/project/`。注意 `todo!()` 是格式化宏，消息文本中的路径参数用 `<project_id>` 形式而不是 `{project_id}`，避免被当成 format 占位符。

## 架构

- `src/main.rs` — 入口：解析 CLI → `Config::from_cli` → `commands::run`。
- `src/cli.rs` — clap derive 定义。全局参数 `--base-url` / `--client-id` / `--client-secret` / `--token` / `-v`，对应环境变量 `PC_OPEN_API_BASE_URL` / `PC_CLIENT_ID` / `PC_CLIENT_SECRET` / `PC_TOKEN`。
- `src/config.rs` — 参数与环境变量合并。认证二选一：客户端凭据模式（`PC_CLIENT_ID` + `PC_CLIENT_SECRET` 成对出现，缺一报错）或直接给令牌（`--token` / `PC_TOKEN`）；都没有即报错。base-url 默认 `https://api.pingcode.com`，必须以 `http(s)://` 开头。
- `src/client/mod.rs` — `PingCodeClient::new()` 为 **async**：客户端凭据模式先 `GET /v1/auth/token?grant_type=client_credentials&client_id=...&client_secret=...` 换取企业令牌（见 `fetch_enterprise_token`），再以 Bearer token 鉴权；`--dry-run` 时跳过换取。`get`/`get_with_query`（带查询参数，值拼为 `?k=v` 并百分号编码，dry-run 预览含完整 URL）/`post`/`patch`/`put`/`delete` 都基于私有 `request(method, path, query, body)`：请求 `{base_url}{path}`，响应 JSON 反序列化为 `T`，非 2xx 返回 `ClientError::Api { status, body }`；dry-run 时通过 `output::print_dry_run` 向 stderr 打印方法/URL/请求体并返回空值，不发网络。`Team`（`/v1/directory/team`，企业令牌可用）、`User`（`/v1/myself`，仅用户令牌可用）等响应模型也定义在此。
- `src/output.rs` — `print_json`（pretty-print 到 stdout）、`read_data(spec)`（解析 `--data`：内联 JSON / `@file` / `@-` stdin，`@@` 转义字面量 `@`）、`ensure_object`（写操作请求体必须是 JSON object）、`print_dry_run`。
- `src/commands/mod.rs` — 顶层分发（一个 `match`）：三级命令 `Command::Pjm { command }` → 模块目录的 `run()`；自由命令 `Command::State` → `commands/dynamic/`。
- `src/commands/context.rs` — `Ctx { client, config }`，所有命令的执行上下文，命令签名统一为 `async fn run(ctx: &Ctx, args: &XxxArgs) -> anyhow::Result<()>`。
- `src/commands/dynamic/` — 自由命令（`state.rs` 以 JSON 展示认证状态/企业/用户信息，支持 `--dry-run`）。
- `src/commands/pjm/` — 三级命令样板：`pjm/mod.rs` 定义模块枚举 `PjmCommand`（资源变体用 struct variant + `#[command(subcommand)]`）；`pjm/workitem/mod.rs` 定义资源枚举 `WorkitemCommand`（操作变体持有 `clap::Args` 参数结构体）；`pjm/workitem/create.rs` 是操作样板：`--data` → `POST /v1/pjm/workitems`。
- `tests/` — assert_cmd 集成测试（全部离线；dry-run 用例无凭据运行）。crate 根为 `tests/cli.rs`（应用级测试 + 模块声明），公共 helper `pc()` 在 `tests/common/mod.rs`；测试按命令模块分目录组织，粒度到资源一级（同一资源的所有操作测试放同一文件）：`tests/dynamic/state.rs` 对应自由命令 `state`，`tests/pjm/<resource>.rs`（如 `pjm/workitem.rs`、`pjm/project.rs`）对应三级命令资源，文件间用 `mod` 声明串联。

**新增三级命令**（module/resource/operation）：
1. 资源目录下新建操作文件，如 `src/commands/pjm/workitem/create.rs`：`#[derive(Args)]` 参数结构体 + `pub async fn run(ctx: &Ctx, args: &XxxArgs) -> anyhow::Result<()>`，写操作用 `output::read_data`/`ensure_object` 解析 `--data`，用 `ctx.client.<method>(path, ...)` 发请求，响应用 `output::print_json` 透传；
2. 在资源 `mod.rs` 的操作枚举加变体（`Xxx(XxxArgs)`）并在 `run()` match 加分支；
3. 若是新资源，在模块 `mod.rs` 的枚举加 struct 变体 `Foo { #[command(subcommand)] command: FooCommand }` 并加分发；
4. 若是新模块，在 `src/cli.rs` 的 `Command` 加同样的 struct 变体，并在 `src/commands/mod.rs` 的顶层 match 加分支。

**新增自由命令**：在 `src/commands/dynamic/` 建文件实现 `pub async fn run(ctx: &Ctx) -> Result<()>`，在 `src/cli.rs` 的 `Command` 加单元变体，在 `src/commands/mod.rs` 顶层 match 加分支。

## 测试注意事项

- 现有测试全部离线，不打真实 API。`tests/common/mod.rs` 的 `pc()` helper 会主动 `env_remove` 掉 `PC_TOKEN` / `PC_CLIENT_ID` / `PC_CLIENT_SECRET` / `PC_OPEN_API_BASE_URL`，避免宿主环境污染断言——新增 CLI 测试沿用该模式：测试文件头部 `use crate::common::pc;`，按所属资源放进 `tests/pjm/<resource>.rs`（新资源需在 `tests/pjm/mod.rs` 加 `mod` 声明），自由命令放进 `tests/dynamic/`。
- 目前没有 mock server 或测试 fixtures；要测真实端点需要有效凭据（`PC_CLIENT_ID=xxx PC_CLIENT_SECRET=yyy cargo run -- state`）。

## 凭据与本地配置

- 凭据不要提交。可在仓库根目录建 `.env`（已 gitignore）：`main()` 启动时通过 `dotenvy::dotenv()` 自动加载工作目录下的 `.env`，已存在的真实环境变量优先（不会被覆盖）。注意 `tests/common/mod.rs` 的 `pc()` helper 会 `current_dir(std::env::temp_dir())`，避免仓库根目录的 `.env` 污染断言。

## 在线文档检索（查 PingCode Open API 事实）

需要确认 PingCode Open API 的端点路径、请求/响应字段、鉴权 scope、参数或版本行为时，**不要凭记忆或猜测编写代码**。

**先读代码注释，不要先搜索**：每个命令的文档地址已经写在代码里——操作文件 `run` 函数的 doc comment 中有 `文档：https://developer.alpha.pingcode.live/restapi/pingcode/<pageName>`，资源/模块 `mod.rs` 的枚举变体 doc comment 中有同一 URL（`Docs: <url>`，约定见上方"代码约定"）。实现或修改命令时，直接打开注释中的 URL 核对接口细节即可（可用 webfetch 直接抓取该页面），**无需**再用搜索脚本找页面。

只有以下情况才运行 `tools/search_nexus_docs.py` 在线检索（数据源：`https://developer.alpha.pingcode.live/sitemap.xml`）：

- 代码注释里**没有**写文档 URL（注释缺失、或要新增的端点归属不明）。
- 注释中的页面未覆盖所需细节（如折叠的嵌套数据结构、通用分页/约定），需要找其他相关页面。
- 不确定某个端点是否存在、或不确定该调用哪个 REST 路径。
- 任何本地代码与注释未覆盖、可能编造的 API 细节。

```bash
# 在仓库根目录运行；纯标准库，无第三方依赖，不需要网络代理之外的任何配置
python3 tools/search_nexus_docs.py "<英文关键词>" [--max-pages 3] [--max-snippet 600] [--json]
```

- 脚本流程：抓取 sitemap → 按 URL/标题/描述对关键词打分 → 下载排名靠前页面，提取 `<main>` 正文片段。**无本地缓存**，每次实时检索；无结果时退出码为 2。
- **关键词必须用英文**：sitemap 中的 URL 路径是英文（如 `workitem`、`permissions`、`rest-api`），首轮打分依赖 URL 匹配，中文关键词命中率极低；返回的正文片段可以是中文。多词用空格分隔并加引号，例如：
  ```bash
  python3 tools/search_nexus_docs.py "work item rest api"
  python3 tools/search_nexus_docs.py "project list" --max-pages 5
  python3 tools/search_nexus_docs.py "oauth scope permissions" --json
  ```
- 检索后以返回结果中的**页面 URL 为依据**再动手写请求路径和反序列化结构体；查不到就如实说明，不要编造端点。
- **只关注 URL 路径以 `restapi/pingcode/` 开头的结果**（如 `https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects`）——这些才是 PingCode Open API 的 REST 端点页面。`restapi/nexus/...`（Nexus 扩展接口）、`reference/resource/...`（数据模型/扩展点参考）等其他前缀的结果与本 CLI 无关，命中后直接忽略，不要据其编写命令。
- 该脚本不属于构建/测试流程，`./scripts/test.sh` 不涉及它。
