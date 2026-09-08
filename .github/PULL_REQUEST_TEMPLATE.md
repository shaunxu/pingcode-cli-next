<!--
感谢贡献！请填写以下内容，并确保 commit message 符合 Conventional Commits 规范。
开发与测试约定见 CONTRIBUTING.md 与 AGENTS.md。
-->

## 改动内容

<!-- 这个 PR 做了什么？为什么？ -->

## 关联 issue

<!-- Closes #123 -->

## 检查清单

- [ ] `./scripts/test.sh` 本地通过（`cargo fmt --check` + clippy `-D warnings` + 测试）
- [ ] commit message 符合 [Conventional Commits](https://www.conventionalcommits.org/)（如 `feat(pjm): add sprint list command`）
- [ ] 新增/修改命令的 `run` 函数 doc comment 与 `mod.rs` 枚举变体上写明了官方文档 URL
- [ ] 面向用户的输出文字（help、错误消息、println）均为英文
- [ ] 新增命令补充了离线测试（`tests/offline/`）；如适用也考虑了 live 旅程测试
- [ ] 文档已更新（README 面向使用者、CONTRIBUTING.md 面向开发者）
