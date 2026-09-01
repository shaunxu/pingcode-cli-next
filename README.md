# pc — PingCode CLI

Command Line Interface for PingCode Open API.

## 环境要求

- Rust 1.75+（推荐使用 stable 最新版）
- Cargo

## 构建

```bash
./scripts/build.sh        # 等价于 cargo build
cargo build --release     # 发布构建
```

## 配置

通过命令行参数或环境变量提供 PingCode 访问凭据（凭据不要提交到仓库）。
默认使用 OAuth2 客户端凭据模式（Client Credentials），即用 Client ID / Client Secret 自动换取企业令牌；
也可直接提供访问令牌跳过换取：

| 参数                | 环境变量                 | 说明                                       |
| ------------------- | ------------------------ | ------------------------------------------ |
| `--base-url`        | `PC_OPEN_API_BASE_URL`   | Open API 基础地址，默认 `https://api.pingcode.com` |
| `--client-id`       | `PC_CLIENT_ID`           | 应用 Client ID（客户端凭据模式）           |
| `--client-secret`   | `PC_CLIENT_SECRET`       | 应用 Client Secret（客户端凭据模式）       |
| `--token`           | `PC_TOKEN`               | 直接指定访问令牌，跳过客户端凭据换取       |
| `-v/--verbose`      | -                        | 输出调试信息                               |
| `--json`            | -                        | 输出原始 JSON 而非人类可读文本             |
| `--dry-run`         | -                        | 只打印将要发出的 HTTP 请求，不实际发送；跳过鉴权，可离线运行 |

所有全局参数既可放在子命令前，也可放在子命令后（如 `pc pjm workitem create --dry-run`）。

也可直接在仓库根目录建 `.env` 文件（已被 gitignore），程序启动时会自动加载；
已设置的真实环境变量优先级更高，不会被 `.env` 覆盖：

```bash
PC_OPEN_API_BASE_URL=https://api.pingcode.com
PC_CLIENT_ID=<你的 Client ID>
PC_CLIENT_SECRET=<你的 Client Secret>
```

## 命令模式

绝大多数命令遵循三级结构：**模块（module）→ 资源（resource）→ 操作（operation）**。

```
pc <module> <resource> <operation> [选项]
```

例如创建工作项（模块 `pjm`、资源 `workitem`、操作 `create`）：

```bash
# 内联 JSON 请求体
cargo run -- pjm workitem create --data '{"project_id":"<项目ID>","type_id":"<类型ID>","title":"工作项标题"}'

# 从文件读取请求体
cargo run -- pjm workitem create --data @payload.json

# 从标准输入读取请求体
cat payload.json | cargo run -- pjm workitem create --data @-

# 离线预览请求：不换令牌、不发网络，无需凭据
cargo run -- --dry-run pjm workitem create --data '{"project_id":"p1","type_id":"t1","title":"x"}'
```

少数命令不遵循三级模式（如 `state`），作为自由命令直接挂在顶层：

```bash
cargo run -- --help
cargo run -- state                      # 展示认证状态、企业与用户信息
cargo run -- state --json               # 以 JSON 输出同样的信息
PC_CLIENT_ID=<ID> PC_CLIENT_SECRET=<SECRET> cargo run -- state
cargo run -- --token <TOKEN> state
```

## 开发脚本

| 脚本               | 作用                                       |
| ------------------ | ------------------------------------------ |
| `scripts/build.sh` | 编译调试版本                               |
| `scripts/dev.sh`   | 运行；若安装了 `cargo-watch` 则自动热重载  |
| `scripts/test.sh`  | 本地 CI：`fmt --check` + clippy + 测试     |
| `scripts/lint.sh`  | 自动格式化并运行 clippy 严格检查           |

也可使用 cargo aliases：`cargo lint`、`cargo check-fmt`。

## 项目结构

```
src/
  main.rs          # 入口
  cli.rs           # clap 命令行定义（全局参数 + 顶层子命令）
  config.rs        # 配置加载（参数 + 环境变量）
  output.rs        # 输出与 --data 输入辅助（JSON 打印、@file/@- 读取）
  client/          # PingCode REST 客户端封装（get/post/patch/put/delete + dry-run）
  commands/
    mod.rs         # 顶层分发：三级命令 → 模块目录；自由命令 → dynamic/
    context.rs     # Ctx：命令执行上下文（client + config）
    dynamic/       # 自由命令（不遵循三级模式，如 state）
    pjm/           # 三级命令示例模块（项目管理）
      workitem/    #   资源：工作项；每个操作一个文件（如 create.rs）
tests/             # 集成测试
scripts/           # 构建 / 开发 / 测试脚本
```

### 新增一个三级命令

以「pjm 模块下 sprint 资源的 list 操作」为例：

1. 在 `src/commands/pjm/` 下建资源目录 `sprint/`，其中 `mod.rs` 定义
   `SprintCommand` 枚举（操作变体持有 `clap::Args` 参数结构体）和 `run()` 分发；
2. 操作实现放 `sprint/list.rs`，签名 `async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()>`，
   通过 `ctx.client.get/post/...` 发请求；
3. 在 `src/commands/pjm/mod.rs` 的 `PjmCommand` 加资源变体并在 `run()` 加一行分发。

新增模块同理：在 `src/commands/` 下建模块目录，并在 `src/cli.rs` 的 `Command` 加变体。

新增自由命令：在 `src/commands/dynamic/` 下建文件，并在 `src/commands/mod.rs` 的顶层 match 加分支。

## 测试

```bash
./scripts/test.sh  # 或 cargo test
```
