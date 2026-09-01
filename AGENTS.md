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

- **面向用户的输出文字一律用中文**：clap 帮助文本、错误消息、`eprintln!` 提示。跟随现有 `cli.rs` / `config.rs` / `client/error.rs` 的写法，不要改成英文。
- rustfmt：`max_width = 100`、Unix 换行（`rustfmt.toml`）。
- 错误处理：库层用 `thiserror`（`ClientError`），应用层用 `anyhow`；`main()` 返回 `anyhow::Result`。

## 架构

- `src/main.rs` — 入口：解析 CLI → `Config::from_cli` → `commands::run`。
- `src/cli.rs` — clap derive 定义。全局参数 `--base-url` / `--token` / `-v`。
- `src/config.rs` — 参数与环境变量合并；token 必填（`--token` 或 `PINGCODE_TOKEN`，缺失即报错）；base-url 默认 `https://api.pingcode.com`，必须以 `http(s)://` 开头。
- `src/client/mod.rs` — `PingCodeClient`：reqwest + rustls，Bearer token 鉴权；`get(path)` 请求 `{base_url}{path}`，响应 JSON 反序列化为 `T`，非 2xx 返回 `ClientError::Api { status, body }`。
- `src/commands/mod.rs` — 子命令分发（一个 `match`）。
- `tests/cli.rs` — assert_cmd 集成测试。

**新增子命令**：在 `src/cli.rs` 的 `Command` 加变体，在 `src/commands/mod.rs` 的 `run()` 加分支，通过 `PingCodeClient` 发请求。

## 测试注意事项

- 现有测试全部离线，不打真实 API。`tests/cli.rs` 的 `pc()` helper 会主动 `env_remove` 掉 `PINGCODE_TOKEN` / `PINGCODE_BASE_URL`，避免宿主环境污染断言——新增 CLI 测试沿用该模式。
- 目前没有 mock server 或测试 fixtures；要测真实端点需要有效 token（`PINGCODE_TOKEN=xxx cargo run -- whoami`）。
- `whoami` 调用的 `/v1/user` 是**占位实现**，端点未经 PingCode API 文档确认，响应按 `serde_json::Value` 透传——不要假设其字段结构。

## 凭据与本地配置

- 令牌不要提交。可在仓库根目录建 `.env`（已 gitignore），写 `export PINGCODE_TOKEN=...` 后 `source .env`；`cargo run` 不自动加载 `.env`。

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
