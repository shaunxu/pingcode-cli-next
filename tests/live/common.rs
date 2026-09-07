//! Live integration tests 的公共 harness。
//!
//! 这些测试会对真实的 PingCode Open API 发起网络请求，属于有副作用的跨步骤旅程测试。
//! 默认全部跳过：只有显式设置 `PC_LIVE_TESTS=1` 且提供了有效凭据
//! （`PC_CLIENT_ID`+`PC_CLIENT_SECRET` 成对，或 `PC_TOKEN`）时才运行。
//! 建议在专用测试租户上运行，详见 AGENTS.md「Live 测试」章节。

use std::process::Output;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use assert_cmd::Command;
use serde_json::Value;

/// 单条 CLI 命令的超时时间。
const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

/// 每个旅程内创建资源时的自增序号，配合时间戳/PID 生成唯一名字。
static NAME_SEQ: AtomicU64 = AtomicU64::new(0);

/// Live 测试上下文。通过 `LiveCtx::new()` 获取：门控条件不满足时返回 `None`，
/// 调用方应直接 return（计为跳过，不失败）。
pub struct LiveCtx {
    /// 复用已有项目 id（`PC_LIVE_PROJECT_ID`）时不创建新项目。
    pub reuse_project_id: Option<String>,
    /// 保留创建的资源不清理（`PC_LIVE_KEEP=1`）。
    pub keep: bool,
}

impl LiveCtx {
    /// 检查门控环境变量。条件不满足时打印跳过原因并返回 `None`。
    pub fn new() -> Option<Self> {
        // 门控只认真实环境变量：.env 里的 PC_LIVE_TESTS 不会武装 live 测试，
        // 避免 ./scripts/test.sh / 裸跑 cargo test 因 .env 误触发真实 API 请求。
        if std::env::var("PC_LIVE_TESTS").as_deref() != Ok("1") {
            eprintln!("skip live tests: run ./scripts/live-test.sh or set PC_LIVE_TESTS=1");
            return None;
        }
        // 门控通过后加载仓库根目录 .env（cargo test 的 cwd 即 crate 根），
        // 凭据/可选配置可写在 .env 中不必每次手动传入；已存在的环境变量优先、不被覆盖。
        let _ = dotenvy::dotenv();
        let has_client =
            std::env::var("PC_CLIENT_ID").is_ok() && std::env::var("PC_CLIENT_SECRET").is_ok();
        let has_token = std::env::var("PC_TOKEN").is_ok();
        if !has_client && !has_token {
            eprintln!(
                "skip live tests: PC_CLIENT_ID+PC_CLIENT_SECRET or PC_TOKEN is required (env or .env)"
            );
            return None;
        }
        Some(Self {
            reuse_project_id: std::env::var("PC_LIVE_PROJECT_ID")
                .ok()
                .filter(|v| !v.is_empty()),
            keep: std::env::var("PC_LIVE_KEEP").as_deref() == Ok("1"),
        })
    }

    /// 运行 `pc <args...>`，不接受额外 stdin，返回原始输出（不做断言）。
    /// 凭据等环境变量从当前进程继承；cwd 切到临时目录以避免加载仓库 `.env`。
    pub fn run_raw(&self, args: &[&str]) -> Output {
        let mut cmd = Command::cargo_bin("pc").expect("pc binary built");
        cmd.args(args)
            .current_dir(std::env::temp_dir())
            .timeout(COMMAND_TIMEOUT);
        cmd.output().expect("failed to execute pc")
    }

    /// 运行命令并断言成功，将 stdout 解析为 JSON 返回。
    pub fn run_ok(&self, args: &[&str]) -> Value {
        let output = self.run_raw(args);
        assert!(
            output.status.success(),
            "expected command success: pc {}\nstdout: {}\nstderr: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(stdout.trim()).unwrap_or_else(|err| {
            panic!(
                "stdout is not valid JSON: pc {}\nstdout: {}\nerror: {err}",
                args.join(" "),
                stdout,
            )
        })
    }

    /// best-effort 执行：清理阶段用，忽略退出码与输出，失败不 panic。
    pub fn run_ok_ignored(&self, args: &[&str]) {
        let _ = self.run_raw(args);
    }

    /// 尝试执行：成功返回 `Some(Value)`，失败返回 `None`（并打印 stderr 摘要）。
    /// 用于受租户预置数据/未公开服务端规则约束的步骤，前置不满足时优雅跳过整个旅程。
    pub fn run_try(&self, args: &[&str]) -> Option<Value> {
        let output = self.run_raw(args);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            serde_json::from_str(stdout.trim()).ok()
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "skip: pc {} failed ({}); this step needs tenant-specific preconfiguration",
                args.join(" "),
                stderr.trim().lines().last().unwrap_or("")
            );
            None
        }
    }

    /// 运行命令并断言失败（非零退出），返回 stderr 文本（用于 404/参数错误等负面用例）。
    pub fn run_fail(&self, args: &[&str]) -> String {
        let output = self.run_raw(args);
        assert!(
            !output.status.success(),
            "expected command failure but succeeded: pc {}\nstdout: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
        );
        String::from_utf8_lossy(&output.stderr).into_owned()
    }

    /// 生成唯一资源名：`pcl-<pid>-<ts>-<seq>`。
    ///
    /// 注意 PingCode 多数资源 name 上限 32 字符，因此 base 名用短前缀 `pcl`
    /// （pc-live 的缩写），带后缀后总长 ≤22；调用方不要再叠加 `pc-live-xxx` 长前缀，
    /// 用场景短名作为 suffix，如 `unique_name("board")` → `pcl-board-123-45-6`。
    pub fn unique_name(&self, suffix: &str) -> String {
        let seq = NAME_SEQ.fetch_add(1, Ordering::Relaxed);
        let short_pid = std::process::id() % 1000;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_secs()
            % 100_000;
        if suffix.is_empty() {
            format!("pcl-{short_pid}-{ts}-{seq}")
        } else {
            format!("pcl-{suffix}-{short_pid}-{ts}-{seq}")
        }
    }

    /// 标识字段（project/space identifier）：≤15 字符、大写字母/数字/连字符。
    pub fn unique_identifier(&self, prefix: &str) -> String {
        let seq = NAME_SEQ.fetch_add(1, Ordering::Relaxed);
        let short_pid = std::process::id() % 1000;
        format!("{}L{:03}{:02}", prefix.to_uppercase(), short_pid, seq % 100)
    }
}

/// 从分页响应信封 `{page_index, page_size, total, values}` 中取 `values` 数组。
pub fn values(value: &Value) -> Vec<Value> {
    value
        .get("values")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

/// 在列表中按 `id` 字段查找。
pub fn find_by_id<'a>(items: &'a [Value], id: &str) -> Option<&'a Value> {
    items
        .iter()
        .find(|item| item.get("id").and_then(Value::as_str) == Some(id))
}

/// 取 JSON 对象的字符串字段。
pub fn str_field<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}
