#!/usr/bin/env bash
# 格式化并修复（会写入文件），随后运行 clippy 严格检查
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --all
cargo clippy --all-targets -- -D warnings
