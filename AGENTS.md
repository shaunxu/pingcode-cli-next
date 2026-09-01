# AGENTS.md

## 项目

`pc` — PingCode Open API 的命令行客户端。单个 Rust 二进制 crate（edition 2021，Rust 1.75+），无 workspace、无 CI 工作流。

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

## 代码约定

- **CLI 面向用户的输出文字一律用英文**：clap 帮助文本（`#[command(about)]` / `#[arg(help)]` / doc comment）、错误消息（`bail!` / `anyhow` / `thiserror` 的 `#[error(...)]`）、`println!` / `eprintln!` 输出、`.expect()` 消息。跟随现有 `cli.rs` / `config.rs` / `client/error.rs` 的写法，不要写成中文。代码内部的注释和 doc comment 仍可用中文。
- **命令分两类**：三级命令（`pc <module> <resource> <operation>`，如 `pc pjm workitem create`）按模块/资源目录组织；自由命令（如 `state`）放 `commands/dynamic/`。新增方式见下方"架构"。
- 全局参数（`--json`、`--dry-run`、`-v` 等）在 `cli.rs` 的 `Cli` 上用 `global = true` 声明，子命令前后均可放置。`--dry-run` 不换令牌、不发网络、允许无凭据；写操作的请求体统一通过 `--data` 传入（内联 / `@file` / `@-` stdin），用 `output::read_data` + `output::ensure_object` 解析。
- rustfmt：`max_width = 100`、Unix 换行（`rustfmt.toml`）。
- 错误处理：库层用 `thiserror`（`ClientError`），应用层用 `anyhow`；`main()` 返回 `anyhow::Result`。

## 架构

- `src/main.rs` — 入口：解析 CLI → `Config::from_cli` → `commands::run`。
- `src/cli.rs` — clap derive 定义。全局参数 `--base-url` / `--client-id` / `--client-secret` / `--token` / `-v`，对应环境变量 `PC_OPEN_API_BASE_URL` / `PC_CLIENT_ID` / `PC_CLIENT_SECRET` / `PC_TOKEN`。
- `src/config.rs` — 参数与环境变量合并。认证二选一：客户端凭据模式（`PC_CLIENT_ID` + `PC_CLIENT_SECRET` 成对出现，缺一报错）或直接给令牌（`--token` / `PC_TOKEN`）；都没有即报错。base-url 默认 `https://api.pingcode.com`，必须以 `http(s)://` 开头。
- `src/client/mod.rs` — `PingCodeClient::new()` 为 **async**：客户端凭据模式先 `GET /v1/auth/token?grant_type=client_credentials&client_id=...&client_secret=...` 换取企业令牌（见 `fetch_enterprise_token`），再以 Bearer token 鉴权；`--dry-run` 时跳过换取。`get/post/patch/put/delete` 都基于私有 `request(method, path, body)`：请求 `{base_url}{path}`，响应 JSON 反序列化为 `T`，非 2xx 返回 `ClientError::Api { status, body }`；dry-run 时通过 `output::print_dry_run` 向 stderr 打印方法/URL/请求体并返回空值，不发网络。`Team`（`/v1/directory/team`，企业令牌可用）、`User`（`/v1/myself`，仅用户令牌可用）等响应模型也定义在此。
- `src/output.rs` — `print_json`（pretty-print 到 stdout）、`read_data(spec)`（解析 `--data`：内联 JSON / `@file` / `@-` stdin，`@@` 转义字面量 `@`）、`ensure_object`（写操作请求体必须是 JSON object）、`print_dry_run`。
- `src/commands/mod.rs` — 顶层分发（一个 `match`）：三级命令 `Command::Pjm { command }` → 模块目录的 `run()`；自由命令 `Command::State` → `commands/dynamic/`。
- `src/commands/context.rs` — `Ctx { client, config }`，所有命令的执行上下文，命令签名统一为 `async fn run(ctx: &Ctx, args: &XxxArgs) -> anyhow::Result<()>`。
- `src/commands/dynamic/` — 自由命令（`state.rs` 展示认证状态/企业/用户信息，支持 `--json` 与 `--dry-run`）。
- `src/commands/pjm/` — 三级命令样板：`pjm/mod.rs` 定义模块枚举 `PjmCommand`（资源变体用 struct variant + `#[command(subcommand)]`）；`pjm/workitem/mod.rs` 定义资源枚举 `WorkitemCommand`（操作变体持有 `clap::Args` 参数结构体）；`pjm/workitem/create.rs` 是操作样板：`--data` → `POST /v1/pjm/workitems`。
- `tests/cli.rs` — assert_cmd 集成测试（全部离线；dry-run 用例无凭据运行）。

**新增三级命令**（module/resource/operation）：
1. 资源目录下新建操作文件，如 `src/commands/pjm/workitem/create.rs`：`#[derive(Args)]` 参数结构体 + `pub async fn run(ctx: &Ctx, args: &XxxArgs) -> anyhow::Result<()>`，写操作用 `output::read_data`/`ensure_object` 解析 `--data`，用 `ctx.client.<method>(path, ...)` 发请求，响应用 `output::print_json` 透传；
2. 在资源 `mod.rs` 的操作枚举加变体（`Xxx(XxxArgs)`）并在 `run()` match 加分支；
3. 若是新资源，在模块 `mod.rs` 的枚举加 struct 变体 `Foo { #[command(subcommand)] command: FooCommand }` 并加分发；
4. 若是新模块，在 `src/cli.rs` 的 `Command` 加同样的 struct 变体，并在 `src/commands/mod.rs` 的顶层 match 加分支。

**新增自由命令**：在 `src/commands/dynamic/` 建文件实现 `pub async fn run(ctx: &Ctx) -> Result<()>`，在 `src/cli.rs` 的 `Command` 加单元变体，在 `src/commands/mod.rs` 顶层 match 加分支。

## 测试注意事项

- 现有测试全部离线，不打真实 API。`tests/cli.rs` 的 `pc()` helper 会主动 `env_remove` 掉 `PC_TOKEN` / `PC_CLIENT_ID` / `PC_CLIENT_SECRET` / `PC_OPEN_API_BASE_URL`，避免宿主环境污染断言——新增 CLI 测试沿用该模式。
- 目前没有 mock server 或测试 fixtures；要测真实端点需要有效凭据（`PC_CLIENT_ID=xxx PC_CLIENT_SECRET=yyy cargo run -- state`）。

## 凭据与本地配置

- 凭据不要提交。可在仓库根目录建 `.env`（已 gitignore）：`main()` 启动时通过 `dotenvy::dotenv()` 自动加载工作目录下的 `.env`，已存在的真实环境变量优先（不会被覆盖）。注意 `tests/cli.rs` 的 `pc()` helper 会 `current_dir(std::env::temp_dir())`，避免仓库根目录的 `.env` 污染断言。

## 在线文档检索（查 PingCode Open API 事实）

需要确认 PingCode Open API 的端点路径、请求/响应字段、鉴权 scope、参数或版本行为时，**不要凭记忆或猜测编写代码**，先运行 `tools/search_nexus_docs.py` 在线检索官方开发者文档（数据源：`https://developer.alpha.pingcode.live/sitemap.xml`）。典型场景：

- 新增子命令、不确定该调用哪个 REST 路径（例如工作项、项目、用户相关接口）。
- 不确定请求参数、响应 JSON 字段结构或分页方式。
- 需要确认接口所需的权限 scope 名称。
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
- 该脚本不属于构建/测试流程，`./scripts/test.sh` 不涉及它。
