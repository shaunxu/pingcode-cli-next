#!/usr/bin/env bash
# 本地开发运行：默认直接 cargo run；若安装了 cargo-watch 则热重载。
# 用法：./scripts/dev.sh [子命令及参数...]
set -euo pipefail
cd "$(dirname "$0")/.."

if cargo watch --version >/dev/null 2>&1; then
    exec cargo watch -x "run -- $*"
else
    exec cargo run -- "$@"
fi
