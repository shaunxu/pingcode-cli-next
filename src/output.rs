use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use serde_json::Value;

/// Pretty-print a JSON value to stdout.
pub fn print_json(value: &Value) -> Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    println!("{text}");
    Ok(())
}

/// Read a `--data` specification and parse it as JSON.
///
/// Forms:
/// - inline JSON: `--data '{"title":"x"}'`
/// - from a file:  `--data @payload.json`
/// - from stdin:   `--data @-`
pub fn read_data(spec: &str) -> Result<Value> {
    let raw = if let Some(path) = spec.strip_prefix("@@") {
        // 转义：以字面量 @@ 开头表示内容本身以 @ 开头
        format!("@{}", path)
    } else if let Some(path) = spec.strip_prefix('@') {
        if path == "-" {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read request body from stdin")?;
            buf
        } else {
            std::fs::read_to_string(path)
                .with_context(|| format!("failed to read request body from file: {path}"))?
        }
    } else {
        spec.to_string()
    };

    let value: Value = serde_json::from_str(raw.trim())
        .context("invalid JSON in --data: expected a JSON value (object, array, string, ...)")?;
    Ok(value)
}

/// Print a dry-run request preview to stderr.
pub fn print_dry_run(method: &str, url: &str, body: Option<&Value>) -> Result<()> {
    let mut err = std::io::stderr().lock();
    writeln!(err, "[dry-run] {method} {url}")?;
    if let Some(body) = body {
        writeln!(err, "[dry-run] request body:")?;
        writeln!(err, "{}", serde_json::to_string_pretty(body)?)?;
    }
    Ok(())
}

/// Bail unless `value` is a JSON object; `--data` for write endpoints must be an object.
pub fn ensure_object(value: Value) -> Result<Value> {
    if !value.is_object() {
        bail!(
            "invalid --data: expected a JSON object, got {}",
            value_kind(&value)
        );
    }
    Ok(value)
}

/// Print an outgoing HTTP request to stderr (verbose mode).
///
/// 格式：`[<UTC 时间戳>] REQUEST <method> <url>`，随后以 `Headers` / `Body` 分段
/// 输出 pretty JSON；`body` 为 `None` 时（GET/DELETE）省略 Body 段。
pub fn log_http_request(method: &str, url: &str, headers: &Value, body: Option<&Value>) {
    let mut err = std::io::stderr().lock();
    if writeln!(err, "[{}] REQUEST {method} {url}", utc_timestamp()).is_err() {
        return;
    }
    let _ = writeln!(err, "Headers");
    let _ = write_pretty_json(&mut err, headers);
    if let Some(body) = body {
        let _ = writeln!(err, "Body");
        let _ = write_pretty_json(&mut err, body);
    }
}

/// Print a received HTTP response to stderr (verbose mode), including status and elapsed time.
///
/// 格式：`[<UTC 时间戳>] RESPONSE <status> <url> (<elapsed>ms)`，随后以 `Headers` /
/// `Body` 分段输出；Body 为 JSON 时 pretty-print，否则原样输出；空响应体省略 Body 段。
pub fn log_http_response(status: u16, url: &str, elapsed_ms: u128, headers: &Value, body: &str) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(
        err,
        "[{}] RESPONSE {status} {url} ({elapsed_ms}ms)",
        utc_timestamp()
    );
    let _ = writeln!(err, "Headers");
    let _ = write_pretty_json(&mut err, headers);
    if !body.is_empty() {
        let _ = writeln!(err, "Body");
        match serde_json::from_str::<Value>(body) {
            Ok(value) => {
                let _ = write_pretty_json(&mut err, &value);
            }
            Err(_) => {
                let _ = writeln!(err, "{body}");
            }
        }
    }
}

/// 将 JSON 值 pretty-print 为一个带尾随换行的块。
fn write_pretty_json(w: &mut impl Write, value: &Value) -> std::io::Result<()> {
    match serde_json::to_string_pretty(value) {
        Ok(pretty) => writeln!(w, "{pretty}"),
        Err(_) => writeln!(w, "{value}"),
    }
}

/// 生成当前 UTC 时间的 RFC 3339 风格时间戳（毫秒精度，`Z` 后缀），不依赖第三方时间库。
fn utc_timestamp() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format_unix_millis(millis)
}

/// 将 Unix 毫秒时间戳格式化为 `YYYY-MM-DDTHH:MM:SS.mmmZ`（UTC）。
///
/// 日期换算基于 Howard Hinnant 的 days-from-civil 算法。
fn format_unix_millis(millis: u128) -> String {
    let total_seconds = millis / 1000;
    let ms = millis % 1000;
    let seconds_of_day = total_seconds % 86_400;
    let hour = seconds_of_day / 3600;
    let minute = (seconds_of_day % 3600) / 60;
    let second = seconds_of_day % 60;

    let days_since_epoch = (total_seconds / 86_400) as i64;
    let (year, month, day) = civil_from_days(days_since_epoch);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{ms:03}Z")
}

/// days-from-civil 的逆运算：1970-01-01 起的天数 -> (年, 月, 日)。
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (year + i64::from(month <= 2), month, day)
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::format_unix_millis;

    #[test]
    fn formats_unix_epoch() {
        assert_eq!(format_unix_millis(0), "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn formats_known_timestamps() {
        // 2021-01-01T00:00:00.000Z
        assert_eq!(
            format_unix_millis(1_609_459_200_000),
            "2021-01-01T00:00:00.000Z"
        );
        // 2026-09-08T12:34:56.789Z
        assert_eq!(
            format_unix_millis(1_788_870_896_789),
            "2026-09-08T12:34:56.789Z"
        );
    }

    #[test]
    fn preserves_millisecond_part() {
        assert_eq!(format_unix_millis(1), "1970-01-01T00:00:00.001Z");
        assert_eq!(format_unix_millis(999), "1970-01-01T00:00:00.999Z");
    }

    #[test]
    fn formats_last_millisecond_of_epoch_day() {
        // 纪元第一天的最后一毫秒
        assert_eq!(format_unix_millis(86_399_999), "1970-01-01T23:59:59.999Z");
    }
}
