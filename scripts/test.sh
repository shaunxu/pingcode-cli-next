#!/usr/bin/env bash
# 本地 CI 流水线：格式检查 + clippy 严格模式 + 全部测试
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> cargo fmt --check"
cargo fmt --all -- --check

echo "==> cargo clippy -D warnings"
cargo clippy --all-targets -- -D warnings

echo "==> cargo test"
cargo test
