# 贡献指南

面向开发者：从源码构建、测试、提交规范与发布流程。架构与命令实现的详细约定（命令组织、输出规范、错误处理、Live 测试门控等）见 [AGENTS.md](AGENTS.md)。

参与本项目即表示你同意遵守 [贡献者公约（CODE_OF_CONDUCT.md）](CODE_OF_CONDUCT.md)。

## 前置条件

- Rust 1.75+（推荐 stable 最新版）与 Cargo
- Python 3（仅运行发版工具 `tools/release.py` 及其单测需要，纯标准库）
- 可选工具：
  - [cargo-watch](https://github.com/watchexec/cargo-watch)：`scripts/dev.sh` 检测到变更自动热重载
  - [committed](https://github.com/crate-ci/committed)：本地 commit message 校验（`cargo install committed`）
  - [cargo-release](https://github.com/crate-ci/cargo-release) 与 [cargo-dist](https://github.com/axodotdev/cargo-dist)：维护者发版用（`cargo install cargo-release cargo-dist`）

## 构建与运行

```bash
./scripts/build.sh        # 等价于 cargo build
cargo build --release     # 发布构建
./scripts/dev.sh          # 运行；安装了 cargo-watch 时自动热重载
```

## 本地配置

可在仓库根目录建 `.env` 文件（已 gitignore，**凭据不要提交**），程序启动时通过 `dotenvy` 自动加载；已设置的真实环境变量优先级更高，不会被 `.env` 覆盖：

```bash
PC_OPEN_API_BASE_URL=https://api.pingcode.com
PC_CLIENT_ID=<你的 Client ID>
PC_CLIENT_SECRET=<你的 Client Secret>
```

Live 测试的 harness 同样从 `.env` 读取凭据（见下方「测试」）。

## 测试

```bash
./scripts/test.sh   # 本地 CI：cargo fmt --check → clippy -D warnings → cargo test
```

也可单独运行：

- `cargo lint` — clippy 全目标 + `-D warnings`（任何 warning 即失败）
- `cargo check-fmt` — 仅检查格式
- `./scripts/lint.sh` — 先 `cargo fmt` 写入再 clippy

测试分两类：

- **离线集成测试**（`tests/offline/`）：dry-run / help / 参数校验用例，无需凭据、不打真实 API。单独运行：`cargo test --test offline`。
- **Live 旅程测试**（`tests/live/`）：对真实 PingCode Open API 发请求的端到端旅程，**会在目标租户创建真实数据，务必使用专用测试租户的凭据**。默认全部跳过；通过 `./scripts/live-test.sh` 运行（自动加载 `.env`、武装门控、校验凭据）。门控变量、凭据与清理约定见 [AGENTS.md](AGENTS.md) 的「Live 测试」一节。
- 发版工具自身的离线单测：`python3 -m unittest tools/test_release.py -v`

## 提交信息规范

commit message 遵循 [Conventional Commits](https://www.conventionalcommits.org/)：`<type>[optional scope]: <description>`，类型为 `feat`/`fix`/`docs`/`style`/`refactor`/`perf`/`test`/`build`/`ci`/`chore`/`revert`，例如 `feat(pjm): add workitem update command`。

克隆仓库后建议运行一次以启用本地提交校验（基于 Rust 原生的 [committed](https://github.com/crate-ci/committed)）：

```bash
cargo install committed      # 校验工具
./scripts/install-hooks.sh   # 启用 commit-msg 钩子（core.hooksPath=scripts/hooks）
```

未安装 `committed` 时钩子只警告不阻断；CI（`.github/workflows/commitlint.yml`）会在每个 pull request 上强制校验 PR 内全部 commit。

## 项目结构

```
src/
  main.rs          # 入口：解析 CLI → Config::from_cli → commands::run
  cli.rs           # clap 命令行定义（全局参数 + 顶层子命令）
  config.rs        # 配置加载（命令行参数 + 环境变量合并）
  output.rs        # 输出与 --data 输入辅助（JSON 打印、@file/@- 读取、dry-run 预览）
  client/          # PingCode REST 客户端封装（鉴权 + get/post/patch/put/delete + dry-run）
  commands/
    mod.rs         # 顶层分发：三级命令 → 模块目录；自由命令 → dynamic/
    context.rs     # Ctx：命令执行上下文（client + config）
    dynamic/       # 自由命令（不遵循三级模式，如 state）
    pjm/           # 三级命令模块示例（项目管理）
      workitem/    #   资源：工作项；每个操作一个文件（如 create.rs）
tests/
  offline/         # 离线集成测试（dry-run / help / 参数校验）
  live/            # Live 旅程测试（真实 API，默认跳过）
scripts/           # 构建 / 开发 / 测试 / 发版脚本
tools/             # 发版工具（release.py）与在线文档检索脚本（search_nexus_docs.py）
```

### 新增命令

三级命令（`pc <module> <resource> <operation>`）以「pjm 模块下 sprint 资源的 list 操作」为例：

1. 在 `src/commands/pjm/` 下建资源目录 `sprint/`，其中 `mod.rs` 定义 `SprintCommand` 枚举（操作变体持有 `clap::Args` 参数结构体）和 `run()` 分发；
2. 操作实现放 `sprint/list.rs`，签名 `async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()>`，通过 `ctx.client.get/post/...` 发请求，结果用 `output::print_json` 输出；
3. 在 `src/commands/pjm/mod.rs` 的 `PjmCommand` 加资源变体并在 `run()` 加一行分发。

新增模块同理：在 `src/commands/` 下建模块目录，并在 `src/cli.rs` 的 `Command` 与 `src/commands/mod.rs` 的顶层 match 各加一个分支。新增自由命令则在 `src/commands/dynamic/` 下建文件并在顶层 match 加分发。

注意：每个命令入口必须在 doc comment 中写明对应官方文档地址；CLI 面向用户的输出文字一律用英文。完整约定见 [AGENTS.md](AGENTS.md) 的「代码约定」与「架构」两节。

## 发布（维护者）

发版是项目级工具（类似 `npm run release`）。main 受 branch protection 保护、不能直接 push，发布分两阶段：本地 prepare 出 release PR → PR 合并后 CI 自动打 tag 触发构建分发。

```bash
./scripts/release.sh --dry-run        # 仅预览新版本号与 CHANGELOG 条目，零副作用（输出 JSON）
./scripts/release.sh                  # 自动计算 Semver，建 release 分支并开 PR（本地需 gh 已登录）
./scripts/release.sh --version 0.4.0  # 手动指定版本号
```

1. 本地跑 `scripts/release.sh`（本地需先 `cargo install cargo-release cargo-dist` 且 `gh auth login`）：脚本校验在 main 上、工作区干净、与 `origin/main` 同步，然后建分支 `release/vX.Y.Z`，由 cargo-release 完成 bump、CHANGELOG、提交（不打 tag、不 push），最后 push 分支并用 `gh` 开标题为 `chore(release): vX.Y.Z` 的 PR。
2. PR review/approve 后合并（建议 squash merge）。
3. merge 到 main 后 `.github/workflows/release-tag.yml` 自动创建并推送 annotated tag `vX.Y.Z`，tag 推送触发 cargo-dist 构建分发。该 workflow 有两道门禁：tag 已存在则幂等成功；head 提交不是 release 提交（消息不含 `chore(release): vX.Y.Z`）则跳过——普通 feat/fix PR 合并不打 tag。失败可在 Actions 页面 re-run，或用 Actions 的 **Run workflow**（`workflow_dispatch`）手动补发。**该 workflow 需要仓库 secret `RELEASE_PAT`**（fine-grained PAT，仅授权本仓库、Contents: Read and write）——用 Actions 默认 `GITHUB_TOKEN` 推的 tag 不会触发其他 workflow。
4. 如果 release PR 已合并但 tag 没打上（CI 失败等），**不要重跑 `scripts/release.sh`**——脚本检测到 `Cargo.toml` 版本领先最新 tag 会直接报错并提示补发 tag，避免生成重复 PR。补发用 Actions 的 Run workflow，或本地 `git tag -a vX.Y.Z -m "chore(release): vX.Y.Z" && git push origin vX.Y.Z`。一次性配置细节见 [AGENTS.md](AGENTS.md) 的「发布」一节。

工具链分四层：

1. `tools/release.py`（纯 Python 3 标准库，单测为 `tools/test_release.py`）：取最近 git tag、解析之后的提交推断版本（0.x 阶段 breaking/feat → minor、fix/perf → patch；1.0+ 按标准 Semver）；`changelog` 子命令作为 cargo-release 的 pre-release-hook，依据提交重写 `CHANGELOG.md`（只收录 feat/fix/perf 及 BREAKING）。
2. [cargo-release](https://github.com/crate-ci/cargo-release)（配置见根目录 `release.toml`）：仅 prepare 阶段使用——更新 `Cargo.toml`/`Cargo.lock`、运行 changelog hook、提交 `chore(release): vX.Y.Z`；`tag`/`push` 均关闭（交给 CI）。
3. `.github/workflows/release-tag.yml`（手写，可自由修改）：release PR 合并后从 `Cargo.toml` 读版本、打 annotated tag 并用 `RELEASE_PAT` 推送。
4. [cargo-dist](https://github.com/axodotdev/cargo-dist)（配置见 `dist-workspace.toml`）：tag 推送触发 GitHub Actions workflow（`.github/workflows/release.yml`，由 `dist generate` 生成，**不要手改该文件**），在 Linux/macOS/Windows 三平台交叉编译、打包（`.tar.xz`/`.zip` + `.sha256`），创建 GitHub Release、发布 shell/PowerShell 一键安装脚本，并把 Homebrew formula 推送到 [shaunxu/homebrew-tap](https://github.com/shaunxu/homebrew-tap)。

改了 `dist-workspace.toml` 后必须运行 `dist generate` 重新生成 workflow（`release-tag.yml` 是手写的，不受影响）。
