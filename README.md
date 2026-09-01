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

也可直接在仓库根目录建 `.env` 文件（已被 gitignore），程序启动时会自动加载；
已设置的真实环境变量优先级更高，不会被 `.env` 覆盖：

```bash
PC_OPEN_API_BASE_URL=https://api.pingcode.com
PC_CLIENT_ID=<你的 Client ID>
PC_CLIENT_SECRET=<你的 Client Secret>
```

## 使用

```bash
cargo run -- --help
cargo run -- state                      # 展示认证状态、企业与用户信息
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
  cli.rs           # clap 命令行定义
  config.rs        # 配置加载（参数 + 环境变量）
  client/          # PingCode REST 客户端封装
  commands/        # 子命令实现
tests/             # 集成测试
scripts/           # 构建 / 开发 / 测试脚本
```

## 测试

```bash
./scripts/test.sh  # 或 cargo test
```
