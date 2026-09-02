"""tools/release.py 的离线单元测试：运行方式

    python3 -m unittest tools/test_release.py -v
"""

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import release  # noqa: E402


class ParseSemverTests(unittest.TestCase):
    def test_valid(self):
        self.assertEqual(release.parse_semver("1.2.3"), (1, 2, 3))
        self.assertEqual(release.parse_semver("v0.10.2".lstrip("v")), (0, 10, 2))

    def test_invalid(self):
        for bad in ["1.2", "1.2.3.4", "x.y.z", "1.2-alpha", "", "v1.2.3"]:
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
        commit = release.parse_commit("aa1bb2c", "feat: add release command", "")
        assert commit is not None
        self.assertEqual(commit["type"], "feat")
        self.assertFalse(commit["breaking"])
        self.assertEqual(commit["subject"], "add release command")
        self.assertIsNone(commit["scope"])

    def test_scoped_fix(self):
        commit = release.parse_commit("aa1bb2c", "fix(client): retry on timeout", "")
        assert commit is not None
        self.assertEqual(commit["type"], "fix")
        self.assertEqual(commit["scope"], "client")

    def test_bang_is_breaking(self):
        commit = release.parse_commit("aa1bb2c", "feat!: drop old auth flag", "")
        assert commit is not None
        self.assertTrue(commit["breaking"])

    def test_breaking_change_footer(self):
        commit = release.parse_commit(
            "aa1bb2c",
            "feat: change auth flow",
            "some body\n\nBREAKING CHANGE: PC_TOKEN is now required",
        )
        assert commit is not None
        self.assertTrue(commit["breaking"])

    def test_non_conventional_returns_none(self):
        self.assertIsNone(release.parse_commit("aa1bb2c", "fix a typo", ""))
        self.assertIsNone(release.parse_commit("aa1bb2c", "initial commit", ""))

    def test_chore_is_parsed_but_not_changelog_worthy(self):
        commit = release.parse_commit("aa1bb2c", "chore: bump deps", "")
        assert commit is not None
        self.assertEqual(commit["type"], "chore")
        self.assertFalse(commit["breaking"])


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
        features_pos = entry.index("### Bug Fixes")
        self.assertLess(breaking_pos, features_pos)
        self.assertIn("- drop flag (aaa1111)", entry)
        # breaking 提交不在普通分组里重复出现
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


class CargoVersionTests(unittest.TestCase):
    def test_update_cargo_toml(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            (repo / "Cargo.toml").write_text('[package]\nname = "pc"\nversion = "0.1.0"\n')
            release.update_cargo_toml(repo, "0.2.0")
            self.assertIn('version = "0.2.0"', (repo / "Cargo.toml").read_text())

    def test_update_cargo_lock(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            (repo / "Cargo.lock").write_text(
                '[[package]]\nname = "pc"\nversion = "0.1.0"\n'
                'dependencies = []\n\n[[package]]\nname = "other"\nversion = "0.1.0"\n'
            )
            release.update_cargo_lock(repo, "0.2.0")
            content = (repo / "Cargo.lock").read_text()
            self.assertIn('name = "pc"\nversion = "0.2.0"', content)
            self.assertIn('name = "other"\nversion = "0.1.0"', content)

    def test_missing_cargo_lock_is_ok(self):
        with tempfile.TemporaryDirectory() as tmp:
            release.update_cargo_lock(Path(tmp), "1.0.0")


if __name__ == "__main__":
    unittest.main()
