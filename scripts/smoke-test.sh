#!/usr/bin/env bash
set -euo pipefail

say() { printf "\n\033[1;36m%s\033[0m\n" "$*"; }

say "🔧 1) Format & lint"
cargo fmt
cargo clippy -- -D warnings

say "🌍 2) Table output (no version)"
cargo run -- scan openssl --ecosystem crates-io

say "🔢 3) Version-aware filter (table)"
cargo run -- scan requests --ecosystem pypi --version 2.31.0

say "💾 4) JSON output"
cargo run -- -f json scan lodash --ecosystem npm

say "📄 5) NDJSON output + validation"
NDJSON_OUT="$(cargo run -- -f ndjson scan lodash --ecosystem npm)"
echo "$NDJSON_OUT"
# Validate each line parses as JSON:
if echo "$NDJSON_OUT" | jq -c . >/dev/null 2>&1; then
  echo "✅ NDJSON lines validated with jq"
else
  echo "❌ NDJSON validation failed"
  exit 1
fi

say "✅ Done — all current features exercised"
