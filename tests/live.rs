//! Live 集成测试入口：对真实 PingCode Open API 发起请求的端到端旅程测试。
//!
//! 默认全部跳过（不设置门控环境变量时，这些测试直接 return，不访问网络），
//! 因此 `cargo test` / `./scripts/test.sh` 在无凭据环境下依然全绿。
//!
//! 推荐运行方式（凭据可写在仓库根 `.env`，gitignored；务必使用专用测试租户）：
//!
//! ```bash
//! ./scripts/live-test.sh
//! ```
//!
//! 手动运行：
//!
//! ```bash
//! PC_LIVE_TESTS=1 PC_CLIENT_ID=xxx PC_CLIENT_SECRET=yyy \
//!   cargo test --test live -- --nocapture
//! ```
//!
//! 门控 `PC_LIVE_TESTS=1` 只认真实环境变量（`.env` 中的值不会武装测试，防止误跑）；
//! 门控通过后 harness 会加载 `.env` 读取凭据/可选配置。
//!
//! 可选环境变量（真实环境变量或 `.env`）：
//! - `PC_OPEN_API_BASE_URL`：指向其他环境（默认 https://api.pingcode.com）
//! - `PC_LIVE_PROJECT_ID`：复用已有测试项目，不新建项目
//! - `PC_LIVE_KEEP=1`：保留创建的资源，便于排查
//!
//! 详见 AGENTS.md「Live 测试」章节。

#[path = "live/common.rs"]
mod common;
#[path = "live/journeys/mod.rs"]
mod journeys;
