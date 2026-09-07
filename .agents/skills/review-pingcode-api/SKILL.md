---
name: review-pingcode-api
description: 对照官方 PingCode REST API 文档审计 pc CLI 的 `src/commands` 并修正偏差。当用户想"对照 REST 文档检查命令"、审计/检查 src/commands 有没有遗漏或错误的端点、核对 PingCode Open API 的路径/HTTP 方法/文档 URL，或说"review/audit PingCode API"时使用。先只读审计、输出分类清单并等待人工批准，批准前不改任何文件。
---

# Review PingCode API

让 CLI 命令与官方 PingCode Open API 保持一致。工作分**两个阶段，阶段之间硬性停顿**——阶段 1 绝不修改文件。

每次编辑都要遵守的约定见 [`reference.md`](reference.md)：阶段 1 前读一遍，阶段 2 动手前再读一遍。仓库编码规范（CLI 输出用英文、doc comment 必须写文档地址、`output::print_json`、命令注册步骤、测试目录结构）见 `AGENTS.md`。

## 阶段 1 — 审计（只读）

1. **盘点 CLI。** 从 `src/commands/**/*.rs` 中，按操作文件提取：doc comment 里的页面名（`文档：.../pingcode/<pageName>`）、HTTP 方法、以及实际调用的 `/v1/...` 路径字面量。
   完成标准：每个操作文件都已登记，无遗漏。

2. **拉取官方接口面。** 下载 `https://developer.alpha.pingcode.live/sitemap.xml`，只保留含 `/restapi/pingcode/` 的 `<loc>`，取末尾的 `<pageName>`。
   完成标准：得到一份排序后的官方页面名清单。

3. **做 diff。** 把"已实现页面名"与"官方页面名"对比，得到两个候选集：文档有但未实现（可能缺失命令）、已实现但文档没有（可能文档 URL 404）。
   完成标准：两个集合都列出来。

4. **抓真实页面核对候选——不要相信页面名。** sitemap 页面名不等于 REST 路径。对每个候选项用 `webfetch` 抓 `https://developer.alpha.pingcode.live/restapi/pingcode/<pageName>`，读取页面里真实的 `METHOD /v1/...`（页面 404 说明 CLI 引用的页面名错了）。套用 `reference.md` 里的命名规律与排除项。
   完成标准：每个候选都有抓取到的页面作为依据。

5. **汇报并停止。** 输出分类清单，每条给出文件路径与正确的"方法+路径"：
   - ① **路径/方法错误**——运行时会真实失败。
   - ② **文档 URL 404**——调用本身正确，仅 comment 里的 URL 错。
   - ③ **命令缺失**——非 DevOps、看起来应实现但 CLI 没有的端点。
   - ④ **误报**——看着可疑但其实正确（注明原因）。
   DevOps、OAuth/内部流程、通用说明页按 `reference.md` 明确排除。然后**等待用户批准**，不要动手改。

## 阶段 2 — 修复（仅在批准后）

6. **先问做到哪一步。** 让用户二选一并按选择执行：
   - **A) 改 + 测试**——修改并验证，改动保留在工作区不提交。
   - **B) 改 + 测试 + 提交**——额外创建 Conventional Commit。
   push 永远不自动做；只有用户单独要求时才 push。

7. **应用修复。**
   - 更正 `/v1/...` 路径、HTTP 方法，以及操作文件和其资源/模块 `mod.rs` 里的 `文档：` / `Docs:` URL。
   - 对缺失命令，按 `AGENTS.md` 的注册步骤补齐（操作文件 → 资源枚举 → 模块枚举 → `cli.rs` → `commands/mod.rs`）；写操作用 `--data`，所有结果统一输出 JSON。
   - CLI 面向用户的文字（help、错误、`println!`）一律英文；代码注释可用中文。
   完成标准：每条已批准的发现都有对应改动。

8. **同步测试。** 更新受影响的 `tests/**` URL 断言；为每个新命令补 dry-run 用例（离线，走 `pc()` helper）。
   完成标准：新命令都有 help + dry-run 测试。

9. **验证。** 运行 `./scripts/test.sh`（fmt → clippy `-D warnings` → 测试）。若报格式问题先 `cargo fmt`。
   完成标准：脚本全绿、零 warning。

10. **积累经验——更新 `reference.md`。** 把本次新得到的教训沉淀进去：新确认的命名规律、某个**不**双段的路径、服务端本就不存在的操作、新的排除项、或被推翻的旧规则。一条事实只保留一个出处；删掉本次被证伪的内容。
    完成标准：本次学到的每个非显而易见的点都已记录。

11. **若选 B：提交。** 只暂存本次相关文件；用 Conventional Commits（路径/文档更正用 `fix:`，新命令用 `feat(scope):`），subject ≤ 50 字符，英文。fix 与 feat 同时存在时拆成多个提交。
