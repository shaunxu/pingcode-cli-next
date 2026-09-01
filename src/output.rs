use std::io::{Read, Write};

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
