#!/usr/bin/env bash
# 发版入口（等价于 npm run release 的手感）：
#   ./scripts/release.sh                 # 按 Conventional Commits 自动计算 Semver 并发布
#   ./scripts/release.sh --version 0.4.0 # 手动指定版本号
#   ./scripts/release.sh --dry-run       # 只预览版本号/changelog/cargo-release 计划，零副作用
#
# 流程：tools/release.py compute 解析版本号 -> cargo release <version>
# （bump Cargo.toml/Cargo.lock -> pre-release-hook 重写 CHANGELOG.md -> 提交
# chore(release): vX.Y.Z -> 打 tag vX.Y.Z -> push）-> tag 推送触发 cargo-dist
# 的 release workflow 交叉编译三平台、建 GitHub Release、发布 shell/PowerShell
# 安装脚本并更新 Homebrew tap。
#
# 前置：cargo install cargo-release cargo-dist
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"

dry_run=false
version=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      dry_run=true
      shift
      ;;
    --version)
      version="${2:?--version requires a value, e.g. --version 0.4.0}"
      shift 2
      ;;
    --version=*)
      version="${1#--version=}"
      shift
      ;;
    -h|--help)
      sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if ! cargo release --version >/dev/null 2>&1; then
  echo "error: cargo-release is not installed; run: cargo install cargo-release" >&2
  exit 1
fi

compute_args=()
if [[ -n "$version" ]]; then
  compute_args+=(--version "$version")
fi

# tools/release.py 负责版本号推断/校验，输出 JSON 供本脚本读取。
preview="$(python3 "$repo_root/tools/release.py" compute ${compute_args[@]+"${compute_args[@]}"})"
new_version="$(printf '%s' "$preview" | python3 -c 'import json,sys; print(json.load(sys.stdin)["version"])')"
tag="v${new_version}"

echo "Releasing ${tag}"
echo "----------------------------------------"
printf '%s\n' "$preview"

if [[ "$dry_run" == true ]]; then
  echo "----------------------------------------"
  echo "[dry-run] cargo-release plan (no changes will be made):"
  cargo release "$new_version" --no-push --no-confirm
  exit 0
fi

# 真实发版：cargo-release 完成 bump、CHANGELOG hook、提交、打 tag、push。
# tag 推送后 cargo-dist 的 GitHub Actions workflow 接管构建与分发。
cargo release "$new_version" --execute --no-confirm
echo "----------------------------------------"
echo "Tag ${tag} pushed; the cargo-dist release workflow will now build" >&2
echo "linux/macos/windows binaries, create the GitHub release, publish the" >&2
echo "shell/PowerShell installers and update the shaunxu/homebrew-tap tap." >&2
