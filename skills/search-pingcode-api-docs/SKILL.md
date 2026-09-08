---
name: search-pingcode-api-docs
description: Search the official PingCode Open API documentation (developer.alpha.pingcode.live) to confirm REST endpoint paths, HTTP methods, request/response fields, auth scopes, pagination or version behavior. Runs a bundled zero-dependency Python script that fetches the docs sitemap, scores pages by English keywords, and downloads the best-matching pages. Use when code comments do not already link to the relevant doc page, the linked page does not cover the detail needed, or you are unsure an endpoint exists.
---

# Search PingCode API Docs

Need a PingCode Open API fact (endpoint path, HTTP method, request/response
fields, auth scope, query params)? Do not guess it — look it up with the
bundled script.

The script lives at `scripts/search_pingcode_docs.py` **relative to this
skill's directory** (the folder containing this `SKILL.md`). It is pure
Python 3 standard library — no pip install, no configuration beyond network
access.

## When to use

- No doc URL is written in the code you are working with (missing comment,
  or the endpoint's ownership is unclear).
- The comment's doc page does not cover the detail you need (folded nested
  data structures, common pagination/conventions).
- You are not sure whether an endpoint exists, or which REST path to call.
- Any API detail not covered by local code or comments that you would
  otherwise have to invent.

## How to run

```bash
python3 <this-skill-dir>/scripts/search_pingcode_docs.py "<english keywords>" [--max-pages 3] [--max-snippet 600] [--json]
```

Examples:

```bash
python3 <this-skill-dir>/scripts/search_pingcode_docs.py "work item rest api"
python3 <this-skill-dir>/scripts/search_pingcode_docs.py "project list" --max-pages 5
python3 <this-skill-dir>/scripts/search_pingcode_docs.py "oauth scope permissions" --json
```

Flow: fetch `https://developer.alpha.pingcode.live/sitemap.xml` → score every
URL by keyword matches against URL/title/description → download the top pages
→ extract readable text from `<main>`. No local cache; every run is live.
Exit code 2 means no results.

## Rules

- **Keywords must be English.** Sitemap URL paths are English (e.g.
  `workitem`, `permissions`, `rest-api`); Chinese keywords almost never
  match. Multiple words go in one quoted string. The returned page snippets
  themselves may be Chinese.
- **Only trust results whose URL path starts with `restapi/pingcode/`**
  (e.g. `https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjects`)
  — those are the PingCode Open API REST endpoint pages. Ignore
  `restapi/nexus/...` (Nexus extension APIs) and `reference/resource/...`
  (data model / extension reference) hits; they are not the Open API.
- Treat the returned **page URL as authoritative**, then read the page for
  the real `METHOD /v1/...` path — page names in the sitemap are not REST
  paths. You can also webfetch the page URL directly to verify fields.
- If the search finds nothing, report that honestly — do not invent
  endpoints.
