# 安全策略

## 支持的版本

`pc` 仍处于 0.x 阶段，仅最新的 minor 版本接受安全修复。建议始终使用 [GitHub Releases](https://github.com/shaunxu/pingcode-cli-next/releases) 中的最新版本。

## 上报安全漏洞

**请不要为安全漏洞创建公开的 issue。** 请通过以下任一方式私下上报：

- 使用 GitHub 的[私有安全公告（Security Advisory）](https://github.com/shaunxu/pingcode-cli-next/security/advisories/new)；
- 或直接联系维护者 [@shaunxu](https://github.com/shaunxu)。

上报时请尽量包含：

- 问题描述与潜在影响；
- 复现步骤（PoC 更佳）；
- 受影响的版本、操作系统与安装方式；
- 你建议的修复方案（如有）。

## 响应流程

- 我们会在收到上报后尽快确认（通常 3 个工作日内）；
- 确认漏洞后会评估影响范围、准备修复，并在新版本发布后通过 [Security Advisories](https://github.com/shaunxu/pingcode-cli-next/security/advisories) 公开披露；
- 报告者会在公告中致谢（除非要求匿名）。

## 凭据与令牌安全

`pc` 的凭据（Client ID / Client Secret / 访问令牌）只在本地使用，不会被收集或上传到第三方。请注意：

- 不要把凭据提交到仓库或粘贴到公开 issue / PR 中；
- 提交问题前可用 `--dry-run` 离线预览请求，输出中不含令牌；
- 若凭据意外泄露，请立即在 PingCode 侧吊销/轮换。
