"""tools/release.py 的离线单元测试：运行方式

    python3 -m unittest tools/test_release.py -v
"""

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import release  # noqa: E402


def git(repo: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=str(repo),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise AssertionError(f"git {' '.join(args)} failed: {result.stderr}")
    return result.stdout


def init_repo() -> Path:
    """Create a throwaway git repo with a committer identity configured."""
    tmp = Path(tempfile.mkdtemp())
    git(tmp, "init", "-q")
    git(tmp, "config", "user.email", "test@example.com")
    git(tmp, "config", "user.name", "Test")
    git(tmp, "config", "commit.gpgsign", "false")
    git(tmp, "config", "tag.gpgsign", "false")
    return tmp


def commit(repo: Path, message: str) -> str:
    (repo / "f.txt").write_text(message + "\n")
    git(repo, "add", "f.txt")
    git(repo, "commit", "-q", "-m", message)
    return git(repo, "rev-parse", "--short", "HEAD").strip()


class ParseSemverTests(unittest.TestCase):
    def test_valid(self):
        self.assertEqual(release.parse_semver("1.2.3"), (1, 2, 3))
        self.assertEqual(release.parse_semver("v0.10.2"), (0, 10, 2))

    def test_invalid(self):
        for bad in ["1.2", "1.2.3.4", "x.y.z", "1.2-alpha", ""]:
            with self.assertRaises(ValueError, msg=bad):
                release.parse_semver(bad)


class BumpVersionTests(unittest.TestCase):
    def test_0x_breaking_bumps_minor(self):
        self.assertEqual(release.bump_version("0.1.0", "major"), "0.2.0")

    def test_0x_feature_bumps_minor(self):
        self.assertEqual(release.bump_version("0.2.4", "minor"), "0.3.0")

    def test_0x_fix_bumps_patch(self):
        self.assertEqual(release.bump_version("0.1.5", "patch"), "0.1.6")

    def test_1x_breaking_bumps_major(self):
        self.assertEqual(release.bump_version("1.2.3", "major"), "2.0.0")

    def test_1x_feature_bumps_minor(self):
        self.assertEqual(release.bump_version("2.0.9", "minor"), "2.1.0")

    def test_1x_fix_bumps_patch(self):
        self.assertEqual(release.bump_version("3.4.5", "patch"), "3.4.6")


class DetermineBumpTests(unittest.TestCase):
    def commits(self, *types_and_bangs):
        return [
            {"type": t, "breaking": bang, "subject": "s", "hash": "abc1234"}
            for t, bang in types_and_bangs
        ]

    def test_breaking_wins(self):
        self.assertEqual(
            release.determine_bump(self.commits(("feat", False), ("fix", True))),
            "major",
        )

    def test_feat_minor(self):
        self.assertEqual(release.determine_bump(self.commits(("feat", False))), "minor")

    def test_fix_patch(self):
        self.assertEqual(release.determine_bump(self.commits(("fix", False))), "patch")

    def test_perf_patch(self):
        self.assertEqual(release.determine_bump(self.commits(("perf", False))), "patch")

    def test_internal_only_raises(self):
        with self.assertRaises(ValueError):
            release.determine_bump(self.commits(("chore", False), ("ci", False)))

    def test_empty_raises(self):
        with self.assertRaises(ValueError):
            release.determine_bump([])


class ParseCommitTests(unittest.TestCase):
    def test_feature(self):
        parsed = release.parse_commit("aa1bb2c", "feat: add release command", "")
        assert parsed is not None
        self.assertEqual(parsed["type"], "feat")
        self.assertFalse(parsed["breaking"])
        self.assertEqual(parsed["subject"], "add release command")
        self.assertIsNone(parsed["scope"])

    def test_scoped_fix(self):
        parsed = release.parse_commit("aa1bb2c", "fix(client): retry on timeout", "")
        assert parsed is not None
        self.assertEqual(parsed["type"], "fix")
        self.assertEqual(parsed["scope"], "client")

    def test_bang_is_breaking(self):
        parsed = release.parse_commit("aa1bb2c", "feat!: drop old auth flag", "")
        assert parsed is not None
        self.assertTrue(parsed["breaking"])

    def test_breaking_change_footer(self):
        parsed = release.parse_commit(
            "aa1bb2c",
            "feat: change auth flow",
            "some body\n\nBREAKING CHANGE: PC_TOKEN is now required",
        )
        assert parsed is not None
        self.assertTrue(parsed["breaking"])

    def test_non_conventional_returns_none(self):
        self.assertIsNone(release.parse_commit("aa1bb2c", "fix a typo", ""))
        self.assertIsNone(release.parse_commit("aa1bb2c", "initial commit", ""))

    def test_chore_is_parsed_but_not_changelog_worthy(self):
        parsed = release.parse_commit("aa1bb2c", "chore: bump deps", "")
        assert parsed is not None
        self.assertEqual(parsed["type"], "chore")
        self.assertFalse(parsed["breaking"])


class ChangelogEntryTests(unittest.TestCase):
    def test_render_groups_and_hashes(self):
        commits = [
            {"hash": "aaa1111", "type": "feat", "breaking": False, "subject": "add x"},
            {"hash": "bbb2222", "type": "fix", "breaking": False, "subject": "fix y"},
            {"hash": "ccc3333", "type": "perf", "breaking": False, "subject": "speed up z"},
        ]
        entry = release.render_changelog_entry("0.2.0", commits, "2026-09-02")
        self.assertIn("## [v0.2.0] - 2026-09-02", entry)
        self.assertIn("### Features", entry)
        self.assertIn("- add x (aaa1111)", entry)
        self.assertIn("### Bug Fixes", entry)
        self.assertIn("- fix y (bbb2222)", entry)
        self.assertIn("### Performance Improvements", entry)
        self.assertIn("- speed up z (ccc3333)", entry)
        self.assertNotIn("### BREAKING CHANGES", entry)

    def test_breaking_section_first(self):
        commits = [
            {"hash": "aaa1111", "type": "feat", "breaking": True, "subject": "drop flag"},
            {"hash": "bbb2222", "type": "fix", "breaking": False, "subject": "fix y"},
        ]
        entry = release.render_changelog_entry("1.0.0", commits, "2026-09-02")
        breaking_pos = entry.index("### BREAKING CHANGES")
        fixes_pos = entry.index("### Bug Fixes")
        self.assertLess(breaking_pos, fixes_pos)
        self.assertIn("- drop flag (aaa1111)", entry)
        # breaking commits are not repeated in the regular groups
        self.assertNotIn("### Features", entry)


class ChangelogFileTests(unittest.TestCase):
    def test_creates_new_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            release.upsert_changelog(repo, "## [v0.2.0] - 2026-09-02\n\n- x\n")
            content = (repo / "CHANGELOG.md").read_text()
            self.assertTrue(content.startswith("# Changelog"))
            self.assertIn("## [v0.2.0]", content)

    def test_inserts_above_previous_entries(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            (repo / "CHANGELOG.md").write_text(
                "# Changelog\n\n## [v0.1.0] - 2026-08-01\n\n- old\n"
            )
            release.upsert_changelog(repo, "## [v0.2.0] - 2026-09-02\n\n- new\n")
            content = (repo / "CHANGELOG.md").read_text()
            self.assertLess(content.index("v0.2.0"), content.index("v0.1.0"))
            self.assertIn("- old", content)
            self.assertIn("- new", content)


class GitIntegrationTests(unittest.TestCase):
    def test_compute_auto_bump_from_commits(self):
        repo = init_repo()
        commit(repo, "feat: initial feature")
        git(repo, "tag", "-a", "v0.1.0", "-m", "release v0.1.0")
        commit(repo, "fix: patch a bug")
        commit(repo, "chore: tidy up")

        result = release.compute_next_version(repo, None)
        self.assertEqual(result["current_version"], "0.1.0")
        self.assertEqual(result["version"], "0.1.1")
        self.assertEqual(result["bump"], "patch")
        self.assertEqual(result["tag"], "v0.1.1")

    def test_compute_0x_breaking_is_minor(self):
        repo = init_repo()
        commit(repo, "feat: initial feature")
        git(repo, "tag", "-a", "v0.1.0", "-m", "release v0.1.0")
        commit(repo, "feat!: rework auth")

        result = release.compute_next_version(repo, None)
        self.assertEqual(result["version"], "0.2.0")
        self.assertEqual(result["bump"], "minor")

    def test_compute_manual_version(self):
        repo = init_repo()
        commit(repo, "feat: initial feature")
        git(repo, "tag", "-a", "v0.3.0", "-m", "release v0.3.0")
        commit(repo, "fix: bug")

        result = release.compute_next_version(repo, "1.0.0")
        self.assertEqual(result["version"], "1.0.0")
        self.assertEqual(result["bump"], "manual")

    def test_compute_manual_version_must_be_greater(self):
        repo = init_repo()
        commit(repo, "feat: initial feature")
        git(repo, "tag", "-a", "v0.3.0", "-m", "release v0.3.0")
        with self.assertRaises(ValueError):
            release.compute_next_version(repo, "0.3.0")

    def test_compute_no_changes_raises(self):
        repo = init_repo()
        commit(repo, "feat: initial feature")
        git(repo, "tag", "-a", "v0.1.0", "-m", "release v0.1.0")
        commit(repo, "chore: just chores")
        with self.assertRaises(ValueError):
            release.compute_next_version(repo, None)

    def test_collect_commits_after_tag(self):
        repo = init_repo()
        commit(repo, "feat: before tag")
        git(repo, "tag", "-a", "v0.1.0", "-m", "release v0.1.0")
        first = commit(repo, "feat: after tag one")
        second = commit(repo, "fix: after tag two")

        commits = release.collect_commits(repo, "v0.1.0")
        subjects = [c["subject"] for c in commits]
        self.assertEqual(subjects, ["after tag one", "after tag two"])
        self.assertEqual([c["hash"] for c in commits], [first, second])


if __name__ == "__main__":
    unittest.main()
