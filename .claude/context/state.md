<!-- assessed-at: 1e56791 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings + fix WASM CI regression

CID iteration 1 vendored the iscc-core v1.3.0 conformance vectors (4 new Meta-Code tests:
test_0017–test_0020) into both Rust and Go, and fixed the Go conformance loader to skip the new
`_metadata` top-level key. The critical conformance issue is resolved in Rust and Go. However, the
advance agent introduced a CI regression: the WASM conformance test still asserts `tested == 16`
while data.json now supplies 20 Meta-Code vectors — `WASM (wasm-pack test)` fails in both CI runs
triggered after the vendoring commit. Two open issues remain (Ruby bindings, language logos).

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `data.json` updated to iscc-core v1.3.0 (50 total vectors; 4 new: test_0017–test_0020) ✅
- Rust conformance assertion updated: `assert_eq!(tested, 20, ...)` in `lib.rs` ✅
- 314 tests pass with default features (per review agent verification) ✅
- `_metadata` key in data.json: ignored silently by `serde_json` (unknown fields skipped) ✅
- Feature matrix CI (5 steps) passed in the prior green run ✅

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅
- Python bindings use data.json by section name — 4 new vectors exercised without code changes ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅
- Node.js tests use data.json by section name — 4 new vectors exercised without code changes ✅

## WASM Bindings

**Status**: partially met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]` ✅
- **CI REGRESSION**: `test_gen_meta_code_v0_conformance` fails in
    `crates/iscc-wasm/tests/conformance.rs`
- Root cause: line 66 still asserts `assert_eq!(tested, 16, "expected 16 conformance tests to run")`
    but data.json now has 20 Meta-Code vectors — assertion must be updated to `20`
- CI run 22627307003: `WASM (wasm-pack test)` = FAILURE; all other 10 jobs = SUCCESS ❌
- The advance agent forgot to update this assertion when it updated the equivalent in `lib.rs`

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass (per last green CI run) ✅
- C FFI tests use data.json by section name — new vectors exercised without code changes ✅
- `cbindgen` header freshness check in CI passed ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- 65 Maven tests pass (per last green CI run) ✅
- Java tests use data.json by section name — new vectors exercised without code changes ✅

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go ✅
- `parseConformanceData()` helper added to skip `_metadata` key in data.json ✅
- `packages/go/testdata/data.json` updated to iscc-core v1.3.0 (byte-identical to Rust copy) ✅
- All 9 per-function Go test files updated to use `parseConformanceData()` ✅
- 155 Go tests pass; `go vet` clean ✅; CGO_ENABLED=0 confirmed ✅

## Ruby Bindings

**Status**: not started

- `crates/iscc-rb/` does not exist; no Magnus bridge, no Ruby gemspec, no Minitest suite
- `specs/ruby-bindings.md` spec written (280 lines) ✅
- `ruby ruby-dev` added to devcontainer Dockerfile ✅
- No Ruby CI job in `.github/workflows/ci.yml`
- No RubyGems step in `.github/workflows/release.yml`
- Not in `scripts/version_sync.py` sync targets

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- **Gap**: Ruby install instructions and quickstart not present (target requires Ruby/gem section)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 7 existing crates/packages; experimental notices removed ✅
- **Gap**: `crates/iscc-rb/README.md` not present (crate not started)

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- All 5 language howto guides current ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; no Ruby tabs in multi-language
    examples

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **FAILING** — latest CI run 22627307003: `WASM (wasm-pack test)` = FAILURE ❌
- URL: https://github.com/iscc/iscc-lib/actions/runs/22627307003
- Failed job: `WASM (wasm-pack test)` — `test_gen_meta_code_v0_conformance` panics with
    `assertion left == right failed: expected 16 conformance tests to run` (left=20, right=16)
- All other 10 jobs in the latest run succeed ✅
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- **Gap**: No `ruby` CI job; no `rubygems` publish step; Ruby not in `version_sync.py`

## Next Milestone

**Immediate (fixes CI first)**: Update `crates/iscc-wasm/tests/conformance.rs` line 66 to change
`assert_eq!(tested, 16, ...)` → `assert_eq!(tested, 20, ...)`. This is a one-line fix that unblocks
the WASM CI job. Verify that no other binding test files (Python, Node.js, Java, C FFI) have similar
hardcoded count assertions that also need updating (grep shows none — confirmed already).

**Then**: Begin Ruby bindings (`crates/iscc-rb/`): scaffold the Magnus crate per the spec at
`.claude/context/specs/ruby-bindings.md`, implement all 32 Tier 1 symbols, add Minitest conformance
suite, add Ruby CI job, add RubyGems release step, update version_sync, add docs/howto/ruby.md,
update README and Per-Crate READMEs.

**Low priority**: Language logos for README and docs (deferred until Ruby is done).
