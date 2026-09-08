---
name: search-pingcode-api-docs
description: 检索 PingCode Open API 官方文档（developer.alpha.pingcode.live），确认 REST 端点路径、HTTP 方法、请求/响应字段、鉴权 scope、分页或版本行为。通过内置的零依赖 Python 脚本抓取文档 sitemap、按英文关键词为页面打分，并下载最匹配的页面。当代码注释中没有链接到相关文档页、链接的页面未覆盖所需细节，或不确定某端点是否存在时使用。
---

# 检索 PingCode API 文档

需要确认 PingCode Open API 的事实（端点路径、HTTP 方法、请求/响应字段、
鉴权 scope、查询参数）时，不要凭猜测——用内置脚本查。

脚本位于 `scripts/search_pingcode_docs.py`，**相对于本 skill 所在目录**
（即包含本 `SKILL.md` 的文件夹）。纯 Python 3 标准库实现——无需 pip
安装，除网络访问外无需任何配置。

## 何时使用

- 你正在处理的代码里没有写文档 URL（缺少注释，或端点归属不明确）。
- 注释中的文档页没有覆盖你需要的细节（折叠的嵌套数据结构、通用分页/约定）。
- 你不确定某个端点是否存在，或该调用哪个 REST 路径。
- 任何本地代码与注释都未覆盖、否则只能凭空编造的 API 细节。

## 如何运行

```bash
python3 <本-skill-目录>/scripts/search_pingcode_docs.py "<英文关键词>" [--max-pages 3] [--max-snippet 600] [--json]
```

示例：

```bash
python3 <本-skill-目录>/scripts/search_pingcode_docs.py "work item rest api"
python3 <本-skill-目录>/scripts/search_pingcode_docs.py "project list" --max-pages 5
python3 <本-skill-目录>/scripts/search_pingcode_docs.py "oauth scope permissions" --json
```

流程：抓取 `https://developer.alpha.pingcode.live/sitemap.xml` → 按关键词对
每个 URL 的 URL/标题/描述匹配情况打分 → 下载排名靠前的页面 → 从 `<main>`
中提取可读文本。无本地缓存，每次运行都是实时请求。退出码 2 表示没有结果。

## 规则

- **关键词必须是英文。** Sitemap 的 URL 路径是英文（如 `workitem`、
  `permissions`、`rest-api`）；中文关键词几乎永远匹配不到。多个词放在
  一个带引号的字符串里。返回的页面摘要本身可能是中文。
- **只采信 URL 路径以 `restapi/pingcode/` 开头的结果**
  （如 `https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects`）
  ——那些才是 PingCode Open API 的 REST 端点页面。忽略
  `restapi/nexus/...`（Nexus 扩展 API）和 `reference/resource/...`
  （数据模型/扩展参考）的命中；它们不属于 Open API。
- 以返回的**页面 URL 为准**，再阅读页面确认真实的 `METHOD /v1/...`
  路径——sitemap 里的页面名不是 REST 路径。也可以直接 webfetch 该页面
  URL 来核对字段。
- 如果什么都没搜到，如实报告——不要编造端点。
