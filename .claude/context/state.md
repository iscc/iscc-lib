# Project State

## Status: IN_PROGRESS

## Phase: Tooling bootstrap complete — Rust workspace not yet started

The CID infrastructure, dev tooling, and project scaffolding are fully in place (14 commits).
`mise.toml` defines all task runner targets and `pyproject.toml` defines Python dev dependencies,
but no Rust workspace, no source code, and no tests exist yet.

## What Exists

- **14 git commits** — CID infrastructure, devcontainer, pre-commit hooks, mise tasks, gitattributes
- **Architecture docs**: `notes/` (00-09) covering all design decisions and tooling stack
- **`mise.toml`**: task runner with `test`, `lint`, `format`, `check`, and all `cid:*` commands
- **`pyproject.toml`**: Python dev deps (prek, ruff, pytest, taplo, yamlfix, ty, mdformat, etc.)
- **`uv.lock`**: Python lockfile committed
- **Pre-commit hooks installed**: `.pre-commit-config.yaml` + prek hooks active (pre-commit +
    pre-push)
- **Dev container**: `.devcontainer/` with Dockerfile, devcontainer.json, run-agent.sh
- **CID agents**: `.claude/agents/` with 4 agent definitions; `tools/cid.py` orchestrator
- **Target/learnings/state context files**: all present and initialized

## What's Missing

- `Cargo.toml` workspace root (virtual workspace with `[workspace]`)
- `crates/iscc-lib/` — pure Rust core with all 9 `gen_*_v0` entrypoints
- `crates/iscc-py/` — PyO3/maturin Python bindings crate
- Conformance test vectors vendored from `iscc-core/data.json`
- Any Rust source code or tests
- CI/CD workflows (`.github/workflows/`)
- `crates/iscc-node/`, `crates/iscc-wasm/`, `crates/iscc-cffi/` (Node.js, WASM, C FFI)

## Verification

No tests exist. `cargo test --workspace` cannot run (no `Cargo.toml`). Nothing to verify.

## Next Milestone

Bootstrap the Rust workspace: create `Cargo.toml` (virtual workspace), `crates/iscc-lib/` skeleton
(`Cargo.toml` + `src/lib.rs` + module stubs for all 9 `gen_*_v0` functions), and vendor the
conformance test vectors so that `cargo test -p iscc-lib` compiles and runs.
