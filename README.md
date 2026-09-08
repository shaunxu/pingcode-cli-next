# pc — PingCode CLI

PingCode Open API 的命令行客户端。

## 安装

发布的二进制托管在 [GitHub Releases](https://github.com/shaunxu/pingcode-cli-next/releases)，支持 Linux (x86_64)、macOS (Apple Silicon) 和 Windows (x86_64)。

**Shell（Linux / macOS）一键安装：**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shaunxu/pingcode-cli-next/releases/latest/download/pc-installer.sh | sh
```

**PowerShell（Windows）一键安装：**

```powershell
irm https://github.com/shaunxu/pingcode-cli-next/releases/latest/download/pc-installer.ps1 | iex
```

**Homebrew：**

```bash
brew tap shaunxu/tap
brew install pc
```

也可以直接从 Releases 页面下载对应平台的压缩包（`.tar.xz` / `.zip`，附带 `.sha256` 校验文件）解压使用。

## 配置

通过命令行参数或环境变量提供 PingCode 访问凭据。默认使用 OAuth2 客户端凭据模式（Client Credentials），即用 Client ID / Client Secret 自动换取企业令牌；也可直接提供访问令牌跳过换取：

| 参数              | 环境变量               | 说明                                             |
| ----------------- | ---------------------- | ------------------------------------------------ |
| `--base-url`      | `PC_OPEN_API_BASE_URL` | Open API 基础地址，默认 `https://api.pingcode.com` |
| `--client-id`     | `PC_CLIENT_ID`         | 应用 Client ID（客户端凭据模式）                 |
| `--client-secret` | `PC_CLIENT_SECRET`     | 应用 Client Secret（客户端凭据模式）             |
| `--token`         | `PC_TOKEN`             | 直接指定访问令牌，跳过客户端凭据换取             |
| `-v/--verbose`    | -                      | 把每个 HTTP 请求/响应（UTC 时间戳、方法、URL、Headers、Body、状态码、耗时）打印到 stderr 用于排查问题；敏感信息自动脱敏 |
| `--dry-run`       | -                      | 只打印将要发出的 HTTP 请求，不实际发送；跳过鉴权，可离线运行 |

凭据可通过 shell 环境变量持久化（例如写入 `~/.zshrc` / `~/.bashrc`）：

```bash
export PC_CLIENT_ID=<你的 Client ID>
export PC_CLIENT_SECRET=<你的 Client Secret>
```

所有命令的返回结果统一以 pretty-print JSON 输出到 stdout。所有全局参数既可放在子命令前，也可放在子命令后（如 `pc pjm workitem create --dry-run`）。

## 命令模式

绝大多数命令遵循三级结构：**模块（module）→ 资源（resource）→ 操作（operation）**。

```
pc <module> <resource> <operation> [选项]
```

例如创建工作项（模块 `pjm`、资源 `workitem`、操作 `create`）：

```bash
# 内联 JSON 请求体
pc pjm workitem create --data '{"project_id":"<项目ID>","type_id":"<类型ID>","title":"工作项标题"}'

# 从文件读取请求体
pc pjm workitem create --data @payload.json

# 从标准输入读取请求体
cat payload.json | pc pjm workitem create --data @-

# 离线预览请求：不换令牌、不发网络，无需凭据
pc --dry-run pjm workitem create --data '{"project_id":"p1","type_id":"t1","title":"x"}'

# 排查问题：打印完整 HTTP 请求/响应（日志在 stderr，结果 JSON 在 stdout，互不干扰）
pc -v pjm workitem list --project-id <项目ID>
```

`-v/--verbose` 输出到 stderr，格式如下（Headers 与 Body 均为 pretty JSON）：

```
[2026-09-08T07:56:27.277Z] REQUEST POST https://api.pingcode.com/v1/pjm/workitems
Headers
{
  "authorization": "Bearer ***",
  "content-type": "application/json",
  "user-agent": "pc/1.0.1"
}
Body
{
  ...请求体...
}
[2026-09-08T07:56:27.410Z] RESPONSE 200 https://api.pingcode.com/v1/pjm/workitems (133ms)
Headers
{ ...响应头... }
Body
{ ...响应体... }
```

日志中的敏感信息会自动脱敏：Authorization 头显示为 `Bearer ***`，令牌换取请求 URL 中的 `client_secret` 与响应中的 `access_token` 掩码为 `***`，multipart 上传只列字段名/文件名/字节数而不打印文件内容。

少数命令不遵循三级模式（如 `state`），作为自由命令直接挂在顶层：

```bash
pc --help
pc state                      # 以 JSON 展示认证状态、企业与用户信息
PC_CLIENT_ID=<ID> PC_CLIENT_SECRET=<SECRET> pc state
pc --token <TOKEN> state
```

## 开发

从源码构建、测试与发布流程见 [CONTRIBUTING.md](CONTRIBUTING.md)。
