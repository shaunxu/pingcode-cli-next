#!/usr/bin/env bash
# 安装仓库内版本化的 git 钩子（设置 core.hooksPath 指向 scripts/hooks）
set -euo pipefail
cd "$(dirname "$0")/.."

chmod +x scripts/hooks/*
git config core.hooksPath scripts/hooks

echo "git hooks installed: core.hooksPath -> scripts/hooks"
echo ""
if command -v committed >/dev/null 2>&1; then
  echo "committed found: commit messages will be checked on every commit."
else
  echo "commit message lint requires 'committed':"
  echo "  cargo install committed"
  echo "(without it, the commit-msg hook only warns; CI enforces the rule on PRs)"
fi
