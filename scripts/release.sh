#!/usr/bin/env bash
# 发版入口（等价于 npm run release 的手感）：
#   ./scripts/release.sh                 # 按 Conventional Commits 自动计算 Semver，
#                                        # 建 release 分支并开 PR（两阶段发布的第一阶段）
#   ./scripts/release.sh --version 0.4.0 # 手动指定版本号
#   ./scripts/release.sh --dry-run       # 只预览版本号/changelog/cargo-release 计划，零副作用
#
# 流程（main 受 branch protection 保护，不能直接 push main）：
#   1. 本地 prepare（本脚本）：tools/release.py compute 解析版本号 -> 建分支
#      release/vX.Y.Z -> cargo release 完成 bump Cargo.toml/Cargo.lock、
#      pre-release-hook 重写 CHANGELOG.md、提交 chore(release): vX.Y.Z
#      （--no-tag --no-push，不打 tag、不推送）-> push 分支并用 gh 开 PR。
#   2. PR 在 GitHub 上 review/approve/merge（建议 squash merge）。
#   3. merge 到 main 后 .github/workflows/release-tag.yml 自动打 annotated tag
#      vX.Y.Z 并用 RELEASE_PAT 推送（GITHUB_TOKEN 推的 tag 不会触发其他 workflow）；
#      tag 推送触发 cargo-dist 的 release workflow 交叉编译三平台、建 GitHub
#      Release、发布 shell/PowerShell 安装脚本并更新 Homebrew tap。
#
# 前置：cargo install cargo-release cargo-dist，且 gh 已登录（gh auth login）。
# 一次性 CI 配置：仓库 Settings → Secrets 添加 fine-grained PAT 为 RELEASE_PAT
# （Contents: Read and write，仅授权本仓库），供 release-tag.yml 推 tag。
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
      sed -n '2,21p' "$0" | sed 's/^# \{0,1\}//'
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
branch="release/${tag}"

echo "Releasing ${tag}"
echo "----------------------------------------"
printf '%s\n' "$preview"

if [[ "$dry_run" == true ]]; then
  echo "----------------------------------------"
  echo "[dry-run] cargo-release plan (no changes will be made):"
  cd "$repo_root"
  cargo release "$new_version" --no-push --no-confirm
  exit 0
fi

# ---- 第一阶段：prepare release PR ---------------------------------------

if ! gh auth status >/dev/null 2>&1; then
  echo "error: gh is not authenticated; run: gh auth login" >&2
  exit 1
fi

cd "$repo_root"

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$current_branch" != "main" ]]; then
  echo "error: release prepare must start from main (current branch: ${current_branch})" >&2
  exit 1
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "error: working tree is not clean; commit or stash changes before releasing" >&2
  exit 1
fi

# main 必须与远端同步，否则 CHANGELOG 会漏掉新合入的提交。
git fetch origin main
local_main="$(git rev-parse main)"
remote_main="$(git rev-parse origin/main)"
if [[ "$local_main" != "$remote_main" ]]; then
  echo "error: local main is out of sync with origin/main; run: git pull --ff-only" >&2
  exit 1
fi

if git show-ref --verify --quiet "refs/tags/${tag}"; then
  echo "error: tag ${tag} already exists; nothing to release" >&2
  exit 1
fi

git checkout -b "$branch"
# 失败时回到 main 并删除只在本地创建的 release 分支（已 push 的分支保留，便于排查）。
cleanup() {
  local rc=$?
  if [[ $rc -ne 0 ]] && git rev-parse --abbrev-ref HEAD | grep -qx "$branch" \
     && ! git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
    git checkout main
    git branch -D "$branch" >/dev/null 2>&1 || true
  fi
  exit $rc
}
trap cleanup EXIT

# cargo-release 只在 prepare 阶段使用：bump + CHANGELOG hook + 提交；
# release.toml 里 tag/push 均已关闭，--allow-branch 覆盖 main 限制（当前在 release 分支）。
cargo release "$new_version" --execute --no-confirm --no-tag --no-push --allow-branch '*'

git push -u origin "$branch"

pr_body="Release ${tag}.

This PR was created by \`scripts/release.sh\`. It bumps \`Cargo.toml\`/\`Cargo.lock\` and regenerates \`CHANGELOG.md\` from Conventional Commits since the last tag; no code changes are included.

After this PR is merged into main, \`.github/workflows/release-tag.yml\` automatically creates and pushes the annotated tag \`${tag}\`, which triggers the cargo-dist release workflow (cross-platform builds, GitHub Release, installers, Homebrew tap)."

gh pr create --base main --head "$branch" \
  --title "chore(release): ${tag}" \
  --body "$pr_body"

trap - EXIT
echo "----------------------------------------"
echo "Release PR created for ${tag}." >&2
echo "Review and merge it (squash recommended); the tag will be created and" >&2
echo "pushed automatically by release-tag.yml, then cargo-dist takes over." >&2
