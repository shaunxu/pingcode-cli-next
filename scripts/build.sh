#!/usr/bin/env bash
# 编译调试版本
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build "$@"
