#!/usr/bin/env python3
"""发布工具：根据 Conventional Commits 计算 Semver 版本号、生成 CHANGELOG.md、
更新 Cargo.toml / Cargo.lock，提交并打 tag，推送到 origin；tag 推送后由
.github/workflows/release.yml 交叉编译三平台二进制并创建 GitHub Release。

纯标准库实现，无第三方依赖。用法：

    python3 tools/release.py --dry-run        # 仅预览版本号与 changelog 内容
    python3 tools/release.py                  # 自动按 commits 确定版本并发布
    python3 tools/release.py --version 0.2.0  # 手动指定版本
"""

from __future__ import annotations

import argparse
import datetime as _dt
import json
import re
import subprocess
import sys
from pathlib import Path

TAG_PREFIX = "v"

# 进入 CHANGELOG 的 Conventional Commit 类型（仅用户可见变更）及其分组标题。
CHANGELOG_GROUPS = [
    ("feat", "Features"),
    ("fix", "Bug Fixes"),
    ("perf", "Performance Improvements"),
]

SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
CARGO_VERSION_RE = re.compile(r'^version\s*=\s*"(\d+\.\d+\.\d+)"\s*$', re.MULTILINE)
# Cargo.lock 中本 crate 所在段落：name = "pc" 之后紧跟 version = "..."
CARGO_LOCK_VERSION_RE = re.compile(
    r'(name\s*=\s*"pc"\s*\nversion\s*=\s*")(\d+\.\d+\.\d+)(")'
)
COMMIT_RE = re.compile(
    r"^(?P<type>[a-z]+)(?:\((?P<scope>[^)]*)\))?(?P<bang>!)?:\s*(?P<subject>.+)$"
)


def run_git(args: list[str], cwd: Path | None = None, capture: bool = True) -> str:
    """执行 git 命令；失败时把 stderr 包成 RuntimeError 抛出。"""
    result = subprocess.run(
        ["git", *args],
        cwd=str(cwd) if cwd else None,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout


def parse_semver(version: str) -> tuple[int, int, int]:
    match = SEMVER_RE.match(version.strip())
    if not match:
        raise ValueError(f"invalid semver version: {version!r} (expected MAJOR.MINOR.PATCH)")
    return tuple(int(g) for g in match.groups())  # type: ignore[return-value]


def format_semver(version: tuple[int, int, int]) -> str:
    return f"{version[0]}.{version[1]}.{version[2]}"


def determine_bump(commits: list[dict]) -> str:
    """根据 commits 判定版本提升级别：major > minor > patch。

    0.x 阶段由调用方单独处理（major 降级为 minor）。
    """
    bump = None
    for commit in commits:
        if commit["breaking"]:
            return "major"
        if commit["type"] == "feat":
            bump = "minor"
        elif commit["type"] in ("fix", "perf") and bump != "minor":
            bump = "patch"
    if bump is None:
        raise ValueError(
            "no user-facing changes (feat/fix/perf) since the last release; "
            "pass --version to release anyway"
        )
    return bump


def bump_version(current: str, bump: str) -> str:
    major, minor, patch = parse_semver(current)
    # 0.x 阶段：不发布 major，breaking change 只升 minor。
    if major == 0:
        if bump == "major":
            bump = "minor"
    if bump == "major":
        return format_semver((major + 1, 0, 0))
    if bump == "minor":
        return format_semver((major, minor + 1, 0))
    if bump == "patch":
        return format_semver((major, minor, patch + 1))
    raise ValueError(f"unknown bump level: {bump!r}")


def parse_commit(hash_: str, subject: str, body: str) -> dict | None:
    """解析单行 commit message；不符合 Conventional Commits 的提交返回 None。"""
    match = COMMIT_RE.match(subject.strip())
    if not match:
        return None
    breaking = bool(match.group("bang")) or "BREAKING CHANGE:" in body
    return {
        "hash": hash_,
        "type": match.group("type"),
        "scope": match.group("scope"),
        "subject": match.group("subject").strip(),
        "breaking": breaking,
    }


def collect_commits(repo: Path, baseline_tag: str | None) -> list[dict]:
    """收集基线 tag 之后（无 tag 则全部历史）的提交并按时间正序返回。"""
    rev = f"{baseline_tag}..HEAD" if baseline_tag else "HEAD"
    sep = "\x1e"
    log = run_git(
        ["log", rev, f"--pretty=format:%h%x1f%s%x1f%b{sep}"], cwd=repo
    )
    commits = []
    for record in log.split(sep):
        record = record.strip("\n")
        if not record:
            continue
        fields = record.split("\x1f")
        if len(fields) < 3:
            continue
        parsed = parse_commit(fields[0], fields[1], fields[2])
        if parsed:
            commits.append(parsed)
    commits.reverse()  # git log 为新到旧，翻转为时间正序
    return commits


def latest_tag(repo: Path) -> str | None:
    result = subprocess.run(
        ["git", "describe", "--tags", "--abbrev=0"],
        cwd=str(repo),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def tag_version(tag: str | None) -> str | None:
    if not tag:
        return None
    return tag[len(TAG_PREFIX) :] if tag.startswith(TAG_PREFIX) else tag


def current_version(repo: Path) -> str:
    tag = latest_tag(repo)
    version = tag_version(tag)
    if version:
        return version
    cargo_toml = (repo / "Cargo.toml").read_text()
    match = CARGO_VERSION_RE.search(cargo_toml)
    if not match:
        raise RuntimeError("cannot find version in Cargo.toml")
    return match.group(1)


def render_changelog_entry(version: str, commits: list[dict], date: str) -> str:
    """渲染 Keep a Changelog 格式的单个版本条目。"""
    lines = [f"## [{TAG_PREFIX}{version}] - {date}", ""]

    breaking = [c for c in commits if c["breaking"]]
    if breaking:
        lines.append("### BREAKING CHANGES")
        lines.append("")
        for commit in breaking:
            lines.append(f"- {commit['subject']} ({commit['hash']})")
        lines.append("")

    for commit_type, heading in CHANGELOG_GROUPS:
        group = [c for c in commits if not c["breaking"] and c["type"] == commit_type]
        if not group:
            continue
        lines.append(f"### {heading}")
        lines.append("")
        for commit in group:
            lines.append(f"- {commit['subject']} ({commit['hash']})")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def upsert_changelog(repo: Path, entry: str) -> None:
    """把新版本条目插入 CHANGELOG.md 顶部；文件不存在则新建。"""
    path = repo / "CHANGELOG.md"
    if not path.exists():
        path.write_text(f"# Changelog\n\n{entry}")
        return

    content = path.read_text()
    lines = content.splitlines(keepends=True)
    insert_at = 0
    for index, line in enumerate(lines):
        if line.startswith("# "):
            insert_at = index + 1
            while insert_at < len(lines) and lines[insert_at].strip() == "":
                insert_at += 1
            break
    new_lines = lines[:insert_at] + ["\n", entry, "\n"] + lines[insert_at:]
    path.write_text("".join(new_lines))


def update_cargo_toml(repo: Path, new_version: str) -> None:
    path = repo / "Cargo.toml"
    content = path.read_text()
    updated, count = CARGO_VERSION_RE.subn(f'version = "{new_version}"', content, count=1)
    if count != 1:
        raise RuntimeError("cannot find version in Cargo.toml")
    path.write_text(updated)


def update_cargo_lock(repo: Path, new_version: str) -> None:
    path = repo / "Cargo.lock"
    if not path.exists():
        return
    content = path.read_text()
    updated, count = CARGO_LOCK_VERSION_RE.subn(rf"\g<1>{new_version}\g<3>", content, count=1)
    if count != 1:
        raise RuntimeError('cannot find package "pc" in Cargo.lock')
    path.write_text(updated)


def find_repo_root() -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError("not inside a git repository; run this tool from the project repo")
    return Path(result.stdout.strip())


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Release the pc CLI: bump version, update CHANGELOG.md, commit, tag and push."
    )
    parser.add_argument(
        "--version",
        dest="version",
        help="version to release (MAJOR.MINOR.PATCH); defaults to bumping from commits",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="print the resolved version and changelog entry without touching files or git",
    )
    args = parser.parse_args()

    repo = find_repo_root()
    baseline_tag = latest_tag(repo)
    version_str = current_version(repo)
    current = parse_semver(version_str)
    commits = collect_commits(repo, baseline_tag)

    if args.version:
        new_version = format_semver(parse_semver(args.version))
        if parse_semver(new_version) <= current:
            raise ValueError(
                f"--version {new_version} must be greater than the current version {version_str}"
            )
        bump = "manual"
    else:
        bump = determine_bump(commits)
        # 0.x 阶段 breaking change 只升 minor，展示实际提升级别。
        if current[0] == 0 and bump == "major":
            bump = "minor"
        new_version = bump_version(version_str, bump)

    date = _dt.datetime.now(_dt.timezone.utc).strftime("%Y-%m-%d")
    tag = f"{TAG_PREFIX}{new_version}"
    changelog_entry = render_changelog_entry(new_version, commits, date)

    if args.dry_run:
        preview = {
            "version": new_version,
            "tag": tag,
            "baseline": baseline_tag,
            "current_version": version_str,
            "bump": bump,
            "commits": commits,
            "changelog_entry": changelog_entry,
            "actions": [
                "update Cargo.toml version",
                "update Cargo.lock version (if present)",
                "write CHANGELOG.md",
                f"git add Cargo.toml Cargo.lock CHANGELOG.md",
                f"git commit -m 'chore(release): {tag}'",
                f"git tag -a {tag} -m '<changelog entry>'",
                f"git push origin <current branch>",
                f"git push origin {tag}",
            ],
        }
        print(json.dumps(preview, indent=2, ensure_ascii=False))
        return 0

    # 真实发布前的前置校验。
    if run_git(["status", "--porcelain"], cwd=repo).strip():
        raise RuntimeError("working tree is not clean; commit or stash changes before releasing")
    branch = run_git(["rev-parse", "--abbrev-ref", "HEAD"], cwd=repo).strip()
    if branch == "HEAD":
        raise RuntimeError("detached HEAD; check out a branch before releasing")
    existing = run_git(["tag", "--list", tag], cwd=repo).strip()
    if existing:
        raise RuntimeError(f"tag {tag} already exists")

    update_cargo_toml(repo, new_version)
    update_cargo_lock(repo, new_version)
    upsert_changelog(repo, changelog_entry)

    run_git(["add", "Cargo.toml", "Cargo.lock", "CHANGELOG.md"], cwd=repo)
    run_git(["commit", "-m", f"chore(release): {tag}"], cwd=repo)
    run_git(["tag", "-a", tag, "-m", changelog_entry], cwd=repo)
    run_git(["push", "origin", branch], cwd=repo)
    run_git(["push", "origin", tag], cwd=repo)

    print(
        json.dumps(
            {
                "version": new_version,
                "tag": tag,
                "branch": branch,
                "commits_included": len(commits),
                "pushed": True,
            },
            indent=2,
        )
    )
    print(
        f"Tag {tag} pushed; the release workflow will build binaries and create the GitHub release.",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, ValueError) as err:
        print(f"error: {err}", file=sys.stderr)
        sys.exit(1)
