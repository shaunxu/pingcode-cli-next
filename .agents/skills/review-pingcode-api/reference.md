# 参考：PingCode Open API 审计要点

审计 `src/commands` 并交叉核对 `tests/offline/`、`tests/live/`（文档 ↔ 命令 ↔ 离线测试 ↔ live 旅程）时沉淀下来的结论。每次执行结束都要更新本文件（SKILL.md 第 10 步）。
页面名指 `https://developer.alpha.pingcode.live/restapi/pingcode/<pageName>`。

**黄金法则：** sitemap 页面名不等于 REST 路径。务必从抓取到的页面确认真实的
`METHOD /v1/...`。页面名里的叠词（doubled word）通常对应路径里的重复段。

## 命名规律

- **按项目/产品/测试库查询的配置列表，路径资源段是双段。**
  配置中心（企业级）列表是单段；按作用域查询的列表会重复资源词：
  - pjm：配置中心 `/v1/pjm/workitem_priorities` vs 按项目
    `/v1/pjm/workitem/workitem_priorities`（`workitem_states`、`workitem_types`、
    `workitem_properties` 同理）。页面名 `getPjmWorkitemWorkitemPrioritiesByProjectId` 等。
  - ship idea：`/v1/ship/idea/idea_priorities`、`idea_states`、`idea_properties`、
    `idea_plans`（都带 `?product_id=`）。页面名 `getShipIdeaIdea*ByProductId`。
  - ship ticket：`/v1/ship/ticket/ticket_priorities`、`ticket_states`、`ticket_types`、
    `ticket_properties`、`ticket_solutions`。页面名 `getShipTicketTicket*ByProductId`。
  - testhub testcase：`/v1/testhub/testcase/testcase_types`、`testcase_states`、
    `testcase_properties`（带 `?library_id=`）。页面名 `getTesthubTestcaseTestcase*ByLibraryId`。
- **不要一刀切地双段。** 这些按作用域查询的列表是单段、且是正确的：
  pjm `workitem/tags`；ship `ticket/channels`、`ticket/tags`；ship `idea/suites`；
  testhub `testrun/statuses`（页面名 `getTesthubTestrunStatusesByLibraryId`）；
  testhub 嵌套的 `libraries/{library_id}/suites` 与 `libraries/{library_id}/testplan_types`。
- **嵌套式与扁平式是两套独立且都有效的资源。** 例如需求排期同时存在：
  扁平 `/v1/ship/idea/idea_plans?product_id=`（scope `pcp:read:ship:idea`）和
  嵌套 `/v1/ship/products/{product_id}/idea_plans[/{plan_id}]`（scope
  `pcp:read:ship:product`）。嵌套路径的末段是 `idea_plans`，不是 `plans`。
- **组织"团队"资源是 `user_groups`。** `/v1/directory/user_groups`，路径参数为
  `user_group_id`；成员在 `/v1/directory/user_groups/{user_group_id}/members[/{member_id}]`。
  **不存在** `/v1/directory/groups`（`getDirectoryGroups*` 页面 404，真实页面名是
  `getDirectoryUserGroups*`）。没有删除团队、修改成员的接口。

## 按 principal 区分的通用资源

评论（comments）、附件（attachments）、关注人（participants）、活动记录（activities）、
关联（relations）、评审（reviews）是跨模块资源，以 `principal_type` + `principal_id`
定位（workitem、workitem_review、testrun、testcase、testcase_review、idea、
idea_review、ticket、page；附件还允许 workitem_deliverable）。

- 对这些资源的**单条 GET/DELETE** 必须带 `?principal_type=&principal_id=`；
  只带 id 的页面会 404。真实页面名以 `...ByPrincipalTypeAndPrincipalId` 结尾
  （如 `getCommentsByCommentIdByPrincipalTypeAndPrincipalId`）。CLI 通常已正确传了查询参数，
  目前发现的问题只是 doc comment 里的 URL 写错。
- 这些通用类型按模块各自定义（如 `PrincipalType` 在 `comments/mod.rs`、
  `attachments/mod.rs`）；复用已有的定义，不要新建。

## 服务端确实存在的端点（缺失时补）

- `comments`：list + **create**（`POST /v1/comments`，请求体需
  principal_type/principal_id/content，可选 reply_comment_id）+ **get** + **delete**。
  没有 update（评论不可改）。
- `entity_properties`（`/v1/entity_properties`）：create/list/get/delete，以
  `entity_type`（workitem|idea|ticket|testcase）+ `entity_id` 定位。create 请求体需
  `property_key`、`value`（JSON ≤2KB）、`overwrite`（1|0）。没有 update。
- `getMyself`（`GET /v1/myself`）是真实端点，但已被 `state` 命令使用，不做独立命令。

## 服务端不存在的操作（不要臆造）

以下资源没有 delete 端点：project、product、sprint、ticket、idea、testhub library、
testplan、testrun、directory user、ship customer、workitem-state，以及各 `*Property`
定义（方案里嵌套的属性关联链接除外）。评论与扩展属性没有 update。当某个 CRUD 动作看似缺失时，
先抓页面确认它确实不存在，再下结论。

## 排除范围

- **DevOps**：所有页面名含 `Devops` 的（代码托管、构建、部署、环境、PR、提交）。用户明确不实现 DevOps。
- **通用说明页**：`overview`、`uri`、`request`、`data-format`、`limit`。
- **OAuth/内部流程**（由 client 处理，不做成命令）：`getAuthTokenByGrantType`、
  `getAuthTokenByGrantType_2/_3`、`getAuthorizeByResponseType`。
- 只统计 `restapi/pingcode/` 下的页面；忽略 `restapi/nexus/...` 与 `reference/...`。

## 测试注意事项

- **离线测试**在 `tests/offline/`（入口 `tests/offline.rs`，按资源分目录组织）：
  `tests/offline/common/mod.rs` 的 `pc()` 会清掉凭据环境变量并在临时目录运行，全部无网络。
  dry-run 预览断言在 **stderr**（`[dry-run] METHOD URL`），断言里是完整 URL
  （如 `https://api.pingcode.com/v1/...`）。路径/方法变更时，操作文件与对应的
  `tests/offline/**/*.rs` URL 字符串都要改；新命令要补 help + dry-run 用例。
- **Live 旅程**在 `tests/live/`（入口 `tests/live.rs`，harness 在 `common.rs`，
  旅程在 `journeys/`）：对真实 API 发请求，默认跳过，仅 `PC_LIVE_TESTS=1` + 有效凭据时运行
  （`./scripts/live-test.sh`），`./scripts/test.sh` 不触发它。旅程通过 CLI 参数调用命令
  （`run_ok(["pjm", "workitem", ...])`），**不含 URL 字符串**，路径修正不会导致编译失败——
  需人工核对受影响旅程的步骤断言，必要时按 AGENTS.md「Live 测试」约定补步骤或旅程，
  改动后提示用户在专用测试租户跑 live 验证。
