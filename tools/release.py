#!/usr/bin/env python3
"""Release helper for the `pc` CLI.

Two modes, driven by scripts/release.sh and cargo-release:

1. ``compute`` (called by scripts/release.sh before cargo-release):
   resolves the next SemVer version from Conventional Commits since the
   latest tag (or from --version), validates it and prints a JSON preview.
   scripts/release.sh then runs ``cargo release <version>`` which bumps
   Cargo.toml/Cargo.lock, commits, tags and pushes.

2. ``changelog`` (the cargo-release ``pre-release-hook``):
   regenerates the CHANGELOG.md section for NEW_VERSION from the commits
   between the PREV_VERSION tag and HEAD. Invoked with DRY_RUN by
   cargo-release; when DRY_RUN=true it only prints the entry.

Git operations (commit/tag/push) and version bumping are intentionally
left to cargo-release; this script only computes versions and maintains
CHANGELOG.md. Pure standard library, no third-party dependencies.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import json
import os
import re
import subprocess
import sys
from pathlib import Path

TAG_PREFIX = "v"

# Conventional Commit types that go into the user-facing changelog, with
# their Keep a Changelog section headings.
CHANGELOG_GROUPS = [
    ("feat", "Features"),
    ("fix", "Bug Fixes"),
    ("perf", "Performance Improvements"),
]

SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
COMMIT_RE = re.compile(
    r"^(?P<type>[a-z]+)(?:\((?P<scope>[^)]*)\))?(?P<bang>!)?:\s*(?P<subject>.+)$"
)


def run_git(args: list[str], cwd: Path | None = None) -> str:
    """Run a git command, raising RuntimeError on failure."""
    result = subprocess.run(
        ["git", *args],
        cwd=str(cwd) if cwd else None,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout


def find_repo_root() -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError("not inside a git repository; run this tool from the project repo")
    return Path(result.stdout.strip())


def parse_semver(version: str) -> tuple[int, int, int]:
    match = SEMVER_RE.match(version.strip().lstrip(TAG_PREFIX))
    if not match:
        raise ValueError(f"invalid semver version: {version!r} (expected MAJOR.MINOR.PATCH)")
    return tuple(int(group) for group in match.groups())  # type: ignore[return-value]


def format_semver(version: tuple[int, int, int]) -> str:
    return f"{version[0]}.{version[1]}.{version[2]}"


def parse_commit(hash_: str, subject: str, body: str) -> dict | None:
    """Parse one commit subject; return None when it is not Conventional Commits."""
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
    """Collect parsed commits after baseline_tag (all history when no tag),
    in chronological order."""
    rev = f"{baseline_tag}..HEAD" if baseline_tag else "HEAD"
    sep = "\x1e"
    log = run_git(["log", rev, f"--pretty=format:%h%x1f%s%x1f%b{sep}"], cwd=repo)
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
    commits.reverse()  # git log is newest-first; flip to chronological order
    return commits


def latest_tag(repo: Path) -> str | None:
    """Return the most recent annotated/lightweight tag reachable from HEAD."""
    result = subprocess.run(
        ["git", "describe", "--tags", "--abbrev=0"],
        cwd=str(repo),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def tag_for_version(version: str) -> str:
    return f"{TAG_PREFIX}{version}"


def determine_bump(commits: list[dict]) -> str:
    """Decide the bump level from commits: major > minor > patch.

    Callers handle the 0.x rule (breaking never leaves 0.x as major).
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
    # Pre-1.0: never publish a major bump; breaking changes bump minor.
    if major == 0 and bump == "major":
        bump = "minor"
    if bump == "major":
        return format_semver((major + 1, 0, 0))
    if bump == "minor":
        return format_semver((major, minor + 1, 0))
    if bump == "patch":
        return format_semver((major, minor, patch + 1))
    raise ValueError(f"unknown bump level: {bump!r}")


def render_changelog_entry(version: str, commits: list[dict], date: str) -> str:
    """Render one Keep a Changelog style release section."""
    lines = [f"## [{tag_for_version(version)}] - {date}", ""]

    breaking = [commit for commit in commits if commit["breaking"]]
    if breaking:
        lines.append("### BREAKING CHANGES")
        lines.append("")
        for commit in breaking:
            lines.append(f"- {commit['subject']} ({commit['hash']})")
        lines.append("")

    for commit_type, heading in CHANGELOG_GROUPS:
        group = [
            commit
            for commit in commits
            if not commit["breaking"] and commit["type"] == commit_type
        ]
        if not group:
            continue
        lines.append(f"### {heading}")
        lines.append("")
        for commit in group:
            lines.append(f"- {commit['subject']} ({commit['hash']})")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def upsert_changelog(repo: Path, entry: str) -> None:
    """Insert a new release section at the top of CHANGELOG.md (create if absent)."""
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
    path.write_text("".join(lines[:insert_at] + ["\n", entry, "\n"] + lines[insert_at:]))


def today() -> str:
    return _dt.datetime.now(_dt.timezone.utc).strftime("%Y-%m-%d")


def compute_next_version(repo: Path, manual: str | None) -> dict:
    """Resolve current version, commits since the last tag and the next version."""
    baseline_tag = latest_tag(repo)
    current_version = (
        baseline_tag[len(TAG_PREFIX) :]
        if baseline_tag and baseline_tag.startswith(TAG_PREFIX)
        else baseline_tag
    )
    commits = collect_commits(repo, baseline_tag)

    if manual:
        new_version = format_semver(parse_semver(manual))
        if current_version and parse_semver(new_version) <= parse_semver(current_version):
            raise ValueError(
                f"--version {new_version} must be greater than the current version "
                f"{current_version}"
            )
        bump = "manual"
    else:
        bump = determine_bump(commits)
        if not current_version:
            raise RuntimeError(
                "no git tag found and no --version given; use --version X.Y.Z for the "
                "first release"
            )
        new_version = bump_version(current_version, bump)
        # Report the effective bump level for 0.x breaking changes.
        if parse_semver(current_version)[0] == 0 and bump == "major":
            bump = "minor"

    return {
        "current_version": current_version,
        "baseline_tag": baseline_tag,
        "version": new_version,
        "tag": tag_for_version(new_version),
        "bump": bump,
        "commits": commits,
    }


def cmd_compute(args: argparse.Namespace) -> int:
    repo = find_repo_root()
    result = compute_next_version(repo, args.version)
    preview = {
        **result,
        "date": today(),
        "actions": [
            "cargo release bumps Cargo.toml/Cargo.lock",
            "pre-release-hook regenerates CHANGELOG.md",
            f"git commit -m 'chore(release): {result['tag']}'",
            f"git tag {result['tag']} (annotated)",
            "git push origin <branch> and tag",
            "cargo-dist CI builds linux/macos/windows, creates the GitHub release, "
            "publishes shell/PowerShell installers and the Homebrew formula",
        ],
    }
    print(json.dumps(preview, indent=2, ensure_ascii=False))
    return 0


def cmd_changelog(_args: argparse.Namespace) -> int:
    """cargo-release pre-release-hook entry point.

    Reads NEW_VERSION / PREV_VERSION / DRY_RUN from the environment that
    cargo-release provides to hooks.
    """
    new_version = os.environ.get("NEW_VERSION")
    if not new_version:
        raise RuntimeError("NEW_VERSION is not set; this command is meant to run as a "
                           "cargo-release pre-release-hook")
    prev_version = os.environ.get("PREV_VERSION") or None
    dry_run = os.environ.get("DRY_RUN", "false").lower() == "true"
    repo = Path(os.environ.get("WORKSPACE_ROOT") or os.environ.get("CRATE_ROOT") or find_repo_root())

    baseline_tag = tag_for_version(prev_version) if prev_version else latest_tag(repo)
    commits = collect_commits(repo, baseline_tag)
    entry = render_changelog_entry(new_version, commits, today())

    if dry_run:
        print(f"[release.py dry-run] would write this CHANGELOG.md section:\n{entry}")
        return 0

    upsert_changelog(repo, entry)
    print(f"[release.py] CHANGELOG.md updated for {tag_for_version(new_version)}")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Compute the next release version and maintain CHANGELOG.md for pc."
    )
    sub = parser.add_subparsers(dest="command", required=True)

    compute = sub.add_parser(
        "compute",
        help="resolve the next version from Conventional Commits (or --version)",
    )
    compute.add_argument(
        "--version",
        help="version to release (MAJOR.MINOR.PATCH); defaults to bumping from commits",
    )
    compute.set_defaults(func=cmd_compute)

    changelog = sub.add_parser(
        "changelog",
        help="regenerate the CHANGELOG.md section (cargo-release pre-release-hook)",
    )
    changelog.set_defaults(func=cmd_changelog)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    return args.func(args)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, ValueError) as err:
        print(f"error: {err}", file=sys.stderr)
        sys.exit(1)
