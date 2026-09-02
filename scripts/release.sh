#!/usr/bin/env bash
# 发布工具入口（等价于 npm run release 的手感）：
#   ./scripts/release.sh --dry-run        # 预览版本号与 changelog，零副作用
#   ./scripts/release.sh                  # 自动按 Conventional Commits 算版本并发布
#   ./scripts/release.sh --version 0.2.0  # 手动指定版本
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 不切换工作目录：release.py 通过 git rev-parse 自行定位仓库根，可在仓库内任意位置调用。
exec python3 "$script_dir/../tools/release.py" "$@"
