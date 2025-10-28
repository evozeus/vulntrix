# 🧠 vulntrix

**Fast OSV vulnerability lookups across ecosystems.**  
Built in Rust with Clap for modern CLI ergonomics and rich output options (table, JSON, NDJSON).

---

## ✨ Features

- 🔍 Queries the OSV.dev database for open-source packages
- 🧩 Ecosystems: `crates-io`, `npm`, `pypi`, `maven`, `rubygems`, `packagist`, `nuget`, `go`
- 🧱 Output formats: human-readable table (default), `json`, or `ndjson`
- 🧪 One-shot smoke test script to sanity-check builds

---

## 🚀 Quick Start

### Build from source
~~~bash
git clone https://github.com/Evozeus/vulntrix.git
cd vulntrix
cargo build --release
~~~

### Usage
~~~bash
# Help (shows formats and subcommands)
vulntrix --help

# Scan a package (table output)
vulntrix scan openssl --ecosystem crates-io

# Version-aware filter
vulntrix scan requests --ecosystem pypi --version 2.31.0

# JSON output
vulntrix -f json scan lodash --ecosystem npm

# NDJSON output (one JSON object per line)
vulntrix -f ndjson scan lodash --ecosystem npm
~~~

Example NDJSON line:
~~~json
{"package":"lodash","ecosystem":"npm","vulns":[{"id":"GHSA-29mw-wpgm-hmr9","summary":"ReDoS in lodash","severity":"CVSS_V3"}]}
~~~

---

## 🧪 Smoke Test

Run all supported output modes and validate NDJSON with `jq`:
~~~bash
./smoke-test.sh
~~~

Expected tail:
~~~
✅ NDJSON lines validated with jq
✅ Done — all current features exercised
~~~

---

## 🔧 CLI Overview

- `-f, --format <FORMAT>`: `table` (default), `json`, `ndjson`
- `scan`: query a single package across a specified ecosystem
- `bulk`: placeholder (explicitly non-functional for now)

Supported ecosystems (OSV-backed):
`crates-io`, `npm`, `pypi`, `maven`, `rubygems`, `packagist`, `nuget`, `go`

---

## 🗂 Project Layout

~~~
vulntrix/
├── src/
│   ├── main.rs   # CLI + output logic
│   └── osv.rs    # OSV client + types
├── smoke-test.sh # Pre-ship sanity suite
├── Cargo.toml
└── README.md
~~~

---

## 🛣️ Roadmap

- `--min-score` / `--min-severity` filters
- Real bulk-file mode (`<ecosystem> <package> [version]` per line)
- Local response caching
- TUI mode for interactive triage

---

## 🧰 Build Targets

- Linux (x86_64, aarch64)
- macOS (Apple Silicon + Intel)
- Windows (x86_64)

Cross-compile example:
~~~bash
cargo build --release --target x86_64-unknown-linux-musl
~~~

---

## ⚖️ License

MIT License © 2025 [Evozeus](https://github.com/Evozeus)  
OSV data © Google and respective ecosystem maintainers.
