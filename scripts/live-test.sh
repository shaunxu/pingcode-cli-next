#!/usr/bin/env bash
# Live integration tests: end-to-end journeys against the real PingCode Open API.
# WARNING: creates real data (projects/workitems/spaces/departments/groups) in the
# target tenant — always run with credentials of a dedicated test tenant.
#
# Credentials are read from the repo-root .env (gitignored) or the real environment
# (real env vars take precedence). Just run:  ./scripts/live-test.sh
set -euo pipefail
cd "$(dirname "$0")/.."

# 从仓库根 .env 加载凭据（不覆盖已存在的真实环境变量）。
if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source ./.env
  set +a
fi

# live-test.sh 本身就是显式入口：自动武装门控，无需手动设置 PC_LIVE_TESTS=1。
# （门控仍用于拦截裸跑 cargo test / ./scripts/test.sh 误打真实 API。）
export PC_LIVE_TESTS=1

if [[ -z "${PC_CLIENT_ID:-}" || -z "${PC_CLIENT_SECRET:-}" ]] && [[ -z "${PC_TOKEN:-}" ]]; then
  echo "error: provide PC_CLIENT_ID+PC_CLIENT_SECRET (client credentials) or PC_TOKEN," >&2
  echo "       either in .env (repo root, gitignored) or as environment variables." >&2
  echo "example .env:" >&2
  echo "  PC_CLIENT_ID=xxxx" >&2
  echo "  PC_CLIENT_SECRET=yyyy" >&2
  exit 1
fi

echo "==> cargo test --test live (hitting: ${PC_OPEN_API_BASE_URL:-https://api.pingcode.com})"
cargo test --test live -- --nocapture
