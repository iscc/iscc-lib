# Project State

## Status: IN_PROGRESS

## Phase: Pre-implementation (planning/design)

iscc-lib is entirely in the planning/design phase. Comprehensive architecture documents exist in
`notes/`, CID agent infrastructure is in place, and a dev container is configured — but no source
code, no Cargo workspace, no tests, and no git commits exist yet.

## What Exists

- **Architecture docs**: 10 documents in `notes/` (00-09) covering workspace structure, language
  bindings, async/streaming, API design, tooling, CI/CD, security, references, and benchmarks
- **Target definition**: `.claude/context/target.md` defines Milestone 1 goals
- **Dev container**: `.devcontainer/` with Dockerfile and run-agent.sh
- **Project guidelines**: `CLAUDE.md` with build commands, tooling stack, design constraints
- **CID infrastructure**: `.claude/agents/` with 4 agent definitions, `tools/cid.py` orchestrator
- **Cauldron notes**: AI feedback docs and iterdev concept in `cauldron/`
- **Prior art reference**: `bio-codes/iscc-sum` (CDC, BLAKE3, streaming pattern documented)

## What's Missing

- `Cargo.toml` workspace root (virtual workspace)
- `mise.toml` for tool versions and task runner
- `crates/iscc/` — Rust core library with all `gen_*_v0` entrypoints
- `crates/iscc-py/` — PyO3/maturin Python bindings
- `pyproject.toml` for Python project
- Conformance test vectors (from iscc-core/data.json, vendored)
- Any Rust source code or tests
- CI/CD workflows
- Any git commits

## Verification

No tests exist. No code exists. Nothing to run.

## Next Milestone

Bootstrap the Rust workspace: create `Cargo.toml` (virtual workspace), `mise.toml`, the `crates/iscc/`
skeleton with `lib.rs` and module stubs, and vendor the conformance test vectors — so that
`cargo test -p iscc` can at least compile and run a placeholder test.
