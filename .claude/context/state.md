# Project State

## Status: IN_PROGRESS

## Phase: Workspace bootstrapped — stub functions only, no implementations

The Rust workspace is set up with a virtual workspace root and one crate (`iscc-lib`). All 9
`gen_*_v0` function signatures are defined with correct parameter types, but every function returns
`Err(NotImplemented)`. No conformance vectors are vendored yet. No binding crates exist.

## What Exists

- **20 git commits** — tooling bootstrap + workspace skeleton
- **Root `Cargo.toml`**: virtual workspace, `workspace.dependencies`, release profile configured
- **`crates/iscc-lib/`**: pure Rust crate with `thiserror` dependency
- **All 9 `gen_*_v0` function stubs** in `src/lib.rs` with correct signatures
- **9 stub tests** that verify each function returns `NotImplemented` — all pass
- **`cargo clippy`** clean (no warnings)
- **`cargo fmt`** clean
- **Architecture docs**: `notes/` (00-09) covering all design decisions
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents

## What's Missing

- **Conformance test vectors** — `iscc-core/data.json` not yet vendored
- **All 9 function implementations** — every `gen_*_v0` is a stub returning `NotImplemented`
- **Internal modules** — no `codec`, `cdc`, `minhash`, `simhash`, `blake3`, `utils` modules
- **`crates/iscc-py/`** — PyO3/maturin Python bindings
- **`crates/iscc-node/`** — napi-rs Node.js bindings
- **`crates/iscc-wasm/`** — wasm-bindgen WASM bindings
- **`crates/iscc-cffi/`** — C FFI with cbindgen
- **CI/CD workflows** — no `.github/workflows/`
- **Benchmarks** — no criterion or pytest-benchmark
- **Documentation site** — no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **9 tests pass** (all stub tests, no conformance tests)
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- No `unsafe` code present
- No conformance vector testing yet — zero real correctness verification

## Next Milestone

Vendor conformance test vectors (`iscc-core/data.json`) and implement the first `gen_*_v0` function
end-to-end (likely `gen_instance_code_v0` or `gen_data_code_v0`, leveraging existing Rust from
`bio-codes/iscc-sum`), with conformance tests verifying output matches `iscc-core`.
