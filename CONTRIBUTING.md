vulntrix Contribution Guide
===========================

Thanks for helping improve vulntrix — a fast, safe OSV vulnerability scanner written in Rust.

1) Fork & Clone
---------------
    git clone https://github.com/Evozeus/vulntrix.git
    cd vulntrix
    git checkout -b feat/short-description

2) Dev Basics
-------------
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
    ./scripts/smoke-test.sh

    Notes:
    - Use Rust stable.
    - Keep commits small and focused.
    - Add tests if you touch core logic.

3) Commit Messages (Conventional Commits)
-----------------------------------------
    feat: add ndjson output option
    fix: handle empty severity gracefully
    docs: update README usage section
    chore: ci and tooling updates
    refactor: simplify OSV JSON mapping

4) Open a Pull Request
----------------------
    - Target branch: main
    - All CI checks must pass
    - Fill out the PR template
    - Keep discussion concise and respectful

5) Code Style
-------------
    - Always run cargo fmt before committing
    - Zero clippy warnings (treat as errors)
    - Prefer readability and explicit types when helpful

6) Tests
--------
    - Unit tests for core logic
    - Integration tests for API and ecosystem handling
    - Run ./scripts/smoke-test.sh before pushing

7) License
----------
By contributing, you agree your changes are licensed under the MIT License.

8) Thanks
---------
Huge thanks for helping shape vulntrix into a fast, elegant, open-source scanner.
