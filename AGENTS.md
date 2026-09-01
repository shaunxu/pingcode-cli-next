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

## 工具脚本

- `tools/search_nexus_docs.py` — 纯标准库 Python 脚本，在线检索 PingCode/Nexus 开发者文档（developer.alpha.pingcode.live），用于查 API 事实：
  ```bash
  python3 tools/search_nexus_docs.py "custom ui react" [--max-pages 3] [--json]
  ```
  不属于构建/测试流程，无第三方依赖。
