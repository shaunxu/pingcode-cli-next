---
name: review-pingcode-api
description: 对照官方 PingCode REST API 文档审计 pc CLI 的 `src/commands` 并修正偏差。当用户想"对照 REST 文档检查命令"、审计/检查 src/commands 有没有遗漏或错误的端点、核对 PingCode Open API 的路径/HTTP 方法/文档 URL，或说"review/audit PingCode API"时使用。先只读审计、输出分类清单并等待人工批准，批准前不改任何文件。
---

# Review PingCode API

让 CLI 命令与官方 PingCode Open API 保持一致。工作分**两个阶段，阶段之间硬性停顿**——阶段 1 绝不修改文件。

每次编辑都要遵守的约定见 [`reference.md`](reference.md)：阶段 1 前读一遍，阶段 2 动手前再读一遍。仓库编码规范（CLI 输出用英文、doc comment 必须写文档地址、`output::print_json`、命令注册步骤、测试目录结构）见 `AGENTS.md`。

## 阶段 1 — 审计（只读）

1. **盘点 CLI 与测试。** 从 `src/commands/**/*.rs` 中，按操作文件提取：doc comment 里的页面名（`文档：.../pingcode/<pageName>`）、HTTP 方法、以及实际调用的 `/v1/...` 路径字面量。同时盘点两侧测试：
   - `tests/offline/`：每个资源文件覆盖了哪些操作（help / dry-run），dry-run 断言里的 HTTP 方法与完整 URL 字符串；
   - `tests/live/journeys/`：每条旅程用 `run_ok`/`run_try`/`run_fail` 等调用了哪些子命令（参数序列）。
   完成标准：每个操作文件、每份离线测试、每条 live 旅程都已登记，无遗漏。

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
   - ⑤ **测试与命令/文档不一致**——审计四方差异（文档 ↔ 命令 ↔ 离线测试 ↔ live 旅程）：
     离线断言的方法/完整 URL 与命令实际调用不符（通常 `cargo test` 已会失败，仍要登记）；
     命令缺少离线 help/dry-run 覆盖；live 旅程引用了不存在或已改名的子命令/参数
     （旅程是字符串参数、编译期不报错，运行才暴露），或受影响端点在旅程中缺少对应步骤。
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

8. **同步测试——修复清单里所有 ⑤ 类项。**
   - **离线（`tests/offline/`，入口 `tests/offline.rs`）**：把与命令实际调用不符的断言改对（dry-run 预览在 stderr，断言里是完整 URL，形如 `https://api.pingcode.com/v1/...`）；为缺少覆盖的命令（含 ③ 新增命令）补 help + dry-run 用例，走 `tests/offline/common/mod.rs` 的 `pc()` helper，按资源归入 `tests/offline/<module>/<resource>.rs`。
   - **Live（`tests/live/`，入口 `tests/live.rs`）**：live 旅程通过 CLI 参数调用命令（如 `run_ok(["pjm", "workitem", ...])`），**不含 URL 字符串**——路径/方法修正不会让它编译失败，但会让原本失败的旅程步骤转为成功。更正引用了不存在/已改名子命令或参数的旅程调用；核对受影响端点所属旅程（`tests/live/journeys/` 按业务链路组织，只读端点看 `read_smoke.rs`）的断言是否需要调整；新命令若自然属于某条 CRUD 旅程，按 AGENTS.md「Live 测试」约定追加步骤（创建→列表/详情交叉验证→清理）。Live 默认跳过、本地无凭据无法验证，改动须在汇报中注明请用户跑 `./scripts/live-test.sh`。
   完成标准：⑤ 类项全部处理；命令都有离线 help + dry-run 测试；受影响的 live 旅程已核对或明确说明无需改动。

9. **验证。** 运行 `./scripts/test.sh`（fmt → clippy `-D warnings` → 测试）。若报格式问题先 `cargo fmt`。该脚本不触发 live 旅程（门控默认跳过）；本次若改动了 `tests/live/`，提醒用户在专用测试租户手动跑 `./scripts/live-test.sh`。
   完成标准：脚本全绿、零 warning；live 改动已提示用户验证。

10. **积累经验——更新 `reference.md`。** 把本次新得到的教训沉淀进去：新确认的命名规律、某个**不**双段的路径、服务端本就不存在的操作、新的排除项、或被推翻的旧规则。一条事实只保留一个出处；删掉本次被证伪的内容。
    完成标准：本次学到的每个非显而易见的点都已记录。

11. **若选 B：提交。** 只暂存本次相关文件；用 Conventional Commits（路径/文档更正用 `fix:`，新命令用 `feat(scope):`），subject ≤ 50 字符，英文。fix 与 feat 同时存在时拆成多个提交。
