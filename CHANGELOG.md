# Changelog
All notable changes to this project will be documented in this file.

The format is based on **[Keep a Changelog](https://keepachangelog.com/en/1.1.0/)**  
and this project adheres to **[Semantic Versioning](https://semver.org/spec/v2.0.0.html)**.

## [Unreleased]

## [0.1.0] - 2025-10-28
### Added
- Fast OSV lookups with a single binary: `vulntrix`.
- Multi-ecosystem support: **crates.io**, **PyPI**, **npm** (`--ecosystem`).
- Version-aware filtering (`--version`) to show only advisories affecting a specific version.
- Output formats:
  - **table** (default, human-friendly)
  - **json** (single JSON object)
  - **ndjson** (newline-delimited JSON)
- Consistent CLI UX with `clap`: `--format`, `--verbose`, `--help`.
- Robust error handling + deterministic output; warnings denied via `#![deny(warnings)]`.
- Smoke test script (`smoke-test.sh`) to exercise all major features quickly.
- GitHub Actions CI (Rust build, fmt, clippy).

### Changed
- Normalized CVSS display: `CVSS_V3`, `CVSS_V4`, or `-` (unknown).

### Security
- Network requests are timeout-bound within the client module and avoid unsafe code.

---

## Links
- **Repo:** https://github.com/Evozeus/vulntrix
- **Issues:** https://github.com/Evozeus/vulntrix/issues

[Unreleased]: https://github.com/Evozeus/vulntrix/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Evozeus/vulntrix/releases/tag/v0.1.0
