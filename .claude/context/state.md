<!-- assessed-at: a6a942c -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings + conformance update

Version 0.1.0 was released. Since then, the target was extended with a full Ruby bindings
requirement (Magnus-based, `iscc-rb` crate). Three open issues block DONE: a critical conformance
gap (iscc-core v1.3.0 adds 4 new test vectors and a `META_TRIM_META` size limit), the Ruby bindings
implementation (not yet started), and a low-priority README logo request.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `gen_meta_code_v0`, `json_to_data_url`, `META_TRIM_*` constants: `#[cfg(feature = "meta-code")]`
- `gen_text_code_v0`, `text_clean`, `text_collapse`: `#[cfg(feature = "text-processing")]`
- All other symbols always available
- `Cargo.toml` features: `default = ["meta-code"]`, `meta-code = ["text-processing", ...]`,
    `text-processing = [dep:unicode-*]` ✅
- `conformance_selftest()` always callable; meta/text sections gated internally ✅
- 314 tests with default features; 250 (no-default-features); 284 (text-processing only) ✅
- CI feature matrix: 5 steps (2 clippy + 3 test combos) all passing ✅
- **Gap**: Does not yet implement iscc-core v1.3.0 changes: 4 new conformance vectors
    (`test_0017`–`test_0020`), `META_TRIM_META` 128 000-byte limit, `_metadata` key in data.json
    vector loader tolerance, codec validation tightening (issue: `critical`)

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅
- Version bumped to 0.1.0 ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅
- Version bumped to 0.1.0 ✅

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- 79 wasm-bindgen tests pass; `cargo clippy -p iscc-wasm -- -D warnings` clean ✅

## C FFI

**Status**: met

- `iscc_gen_sum_code_v0(path, bits, wide, add_units: bool)` — 4-parameter signature ✅
- `iscc_IsccSumCodeResult.units: char **` — NULL-terminated array or `NULL` ✅
- Memory-safety fix applied: boxed slices replace `shrink_to_fit` (commit c5857d9) ✅
- 85 Rust tests + 65 C tests pass; `cargo clippy -p iscc-ffi -- -D warnings` clean ✅
- `iscc_sum.c` example compiles; `docs/howto/c-cpp.md` linked in nav ✅
- `cbindgen` header freshness checked in CI ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- `genSumCodeV0(String path, int bits, boolean wide, boolean addUnits)` — 4-parameter ✅
- 65 Maven tests pass; `cargo clippy -p iscc-jni -- -D warnings` clean ✅
- Version bumped to 0.1.0 ✅

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go ✅; 154 Go tests pass; `go vet` clean ✅
- `GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool) (*SumCodeResult, error)` ✅
- Pure Go (no cgo); CGO_ENABLED=0 works ✅

## Ruby Bindings

**Status**: not started

- `crates/iscc-rb/` does not exist; no Magnus bridge, no Ruby gemspec, no Minitest suite
- `specs/ruby-bindings.md` spec written (280 lines) ✅
- `ruby ruby-dev` added to devcontainer Dockerfile ✅
- No Ruby CI job in `.github/workflows/ci.yml`
- No RubyGems step in `.github/workflows/release.yml`
- Not in `scripts/version_sync.py` sync targets
- DevContainer has Ruby installed but crate not scaffolded

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- **Gap**: Ruby install instructions and quickstart not present (target requires Ruby/gem section) ✅

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 7 existing crates/packages; experimental notices removed ✅
- **Gap**: `crates/iscc-rb/README.md` not present (crate not started)

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- All 5 language howto guides updated to 4-parameter `gen_sum_code_v0` signature ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- `uv run zensical build` exits 0 ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; target requires Ruby in all
    multi-language code example tabs

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- pytest-benchmark comparison files; speedup factors in `docs/benchmarks.md` ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **All 11 CI jobs SUCCESS** on HEAD `a6a942c` — **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22626200126
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10, Python 3.14, Python (gate),
    Node.js, WASM, C FFI, Java, Go, Bench — all SUCCESS ✅
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- Tag push `v*.*.*` triggers all publish jobs ✅
- v0.1.0 released to all registries ✅
- **Gap**: No `ruby` CI job; no `rubygems` publish step in `release.yml`; Ruby not in
    `version_sync.py` (9 targets, should be 10 with `iscc-rb/lib/iscc_lib/version.rb`)

## Next Milestone

Three open issues block DONE. Priority order:

1. **Critical — iscc-core v1.3.0 conformance update**: Update `reference/iscc-core` shallow clone to
    v1.3.0; vendor new `data.json`; add `_metadata` key tolerance to the vector loader; implement
    `META_TRIM_META` 128 000-byte payload limit in `gen_meta_code_v0`; pass 4 new conformance
    vectors (`test_0017` JCS float-as-integer, `test_0018` JCS large float, `test_0019` description
    trim, `test_0020` description trim i18n).

2. **Normal — Ruby bindings** (`crates/iscc-rb/`): Scaffold the Magnus crate, implement all 32 Tier
    1 symbols, add Minitest conformance suite, add Ruby CI job, add RubyGems release step, update
    version_sync, add docs/howto/ruby.md, update README and Per-Crate READMEs.

3. **Low — Language logos**: Visual logo additions to README and docs (deferred until Ruby is done).
