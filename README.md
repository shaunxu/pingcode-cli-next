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

通过命令行参数或环境变量提供 PingCode 访问凭据（令牌不要提交到仓库）：

| 参数          | 环境变量             | 说明                          |
| ------------- | -------------------- | ----------------------------- |
| `--base-url`  | `PINGCODE_BASE_URL`  | API 基础地址，默认官方地址    |
| `--token`     | `PINGCODE_TOKEN`     | 访问令牌（必填）              |
| `-v/--verbose`| -                    | 输出调试信息                  |

可将环境变量写入本地 `.env`（已被 gitignore）后 `source .env` 使用。

## 使用

```bash
cargo run -- --help
cargo run -- --token <TOKEN> whoami
PINGCODE_TOKEN=<TOKEN> cargo run -- whoami
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
