<!-- assessed-at: 5f55123c746776e262eaa9b31303c9b16b264f25 -->

# Project State

## Status: IN_PROGRESS

## Phase: Bug fixes applied; normal-priority gaps remain

The `alg_cdc_chunks` hang-on-zero bug is fixed in the Rust core: the public API now returns
`IsccResult` and validates `avg_chunk_size < 2`. All binding crates are updated accordingly. All 12
CI jobs pass (run 22662032256). Two `normal`-priority gaps were filed: Go's `AlgCdcChunks` lacks the
same validation, and the release workflow has no smoke tests before publishing.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `alg_cdc_chunks` public API now returns `IsccResult<Vec<&[u8]>>` — validates `avg_chunk_size < 2`
    with `IsccError::InvalidInput` ✅
- `alg_cdc_chunks_unchecked` added as `pub(crate)` for internal callers (`gen_data_code_v0`,
    `DataHasher::update`) where size is the compile-time constant `DATA_AVG_CHUNK_SIZE` ✅
- `docs/howto/rust.md` updated to reflect `IsccResult<Vec<&[u8]>>` return type ✅
- `data.json` at iscc-core v1.3.0 (50 total vectors) ✅
- Rust conformance assertion: `assert_eq!(tested, 20, ...)` ✅
- 316 tests pass with default features (up from 314 — new validation tests added) ✅
- Feature matrix CI (5 steps) passed in latest green run ✅

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `alg_cdc_chunks` updated to propagate `IsccResult` from Rust core via `PyResult` ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- `alg_cdc_chunks` updated to propagate `IsccResult` error from Rust core ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]` ✅
- `alg_cdc_chunks` maps `IsccResult` to `JsError` ✅
- `wasm-opt` upgraded from `-O` to `-O3` for max runtime performance ✅
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20` ✅
- `WASM (wasm-pack test)` = SUCCESS in CI run 22662032256 ✅

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass (per last green CI run) ✅
- `iscc_alg_cdc_chunks` propagates `IsccResult` error via null return ✅
- `cbindgen` header freshness check in CI passed ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- `AlgCdcChunks` JNI now validates `avgChunkSize < 2` with `IllegalArgumentException` (was `< 0`) ✅
- 65 Maven tests pass (per last green CI run) ✅

## Go Bindings

**Status**: partially met

- All 32 Tier 1 symbols via pure Go ✅
- 155 Go tests pass; `go vet` clean ✅
- **Gap**: `packages/go/cdc.go:AlgCdcChunks` has no `avgChunkSize` validation — when 0 is passed,
    `algCdcParams` computes degenerate parameters and the chunking loop hangs (matches bug the Rust
    core just fixed). Issue filed as `normal` priority.

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
- `crates/iscc-rb/LICENSE` (Apache 2.0) added ✅
- `alg_cdc_chunks` propagates `IsccResult` via `map_err(to_magnus_err)` ✅
- **All 32 of 32 Tier 1 symbols** exposed:
    - Gen functions: `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
        `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
        `gen_iscc_code_v0`, `gen_sum_code_v0` (10) ✅
    - Text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse` (4) ✅
    - Codec/diagnostic: `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`,
        `json_to_data_url`, `conformance_selftest` (6) ✅
    - Algorithm primitives: `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
        `soft_hash_video_v0` (5) ✅
    - Constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
        `TEXT_NGRAM_SIZE` (5) ✅
    - Streaming types: `DataHasher`, `InstanceHasher` (2) ✅
- 111 Minitest tests (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- Dedicated `ruby` CI job — runs standardrb, clippy, compile, and test ✅
- `docs/howto/ruby.md` (422 lines) ✅; `docs/ruby-api.md` (781 lines — all 32 symbols) ✅
- `zensical.toml` Reference section: "Ruby API" nav entry ✅
- Root `README.md` Ruby section (install tab + quickstart) ✅
- Standard Ruby linting fully configured ✅
- RubyGems.org account created, `GEM_HOST_API_KEY` secret configured in GitHub ✅
- Gem name `iscc-lib` not yet reserved (RubyGems has no reservation mechanism — name is claimed on
    first `gem push`)

## C# / .NET Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/dotnet/` does not exist; no `csbindgen` integration; no CI job

## C++ Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/cpp/` does not exist; no `iscc.hpp`; no vcpkg/Conan manifests

## Swift Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (not
    started)

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- Ruby install instructions and quickstart present ✅
- **Gap**: C#, C++, Swift, Kotlin sections not present (target requires all 4; all `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages including `crates/iscc-rb/README.md` (93 lines)
    ✅
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet (all `low` priority)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 8 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, ruby.md
    ✅
- `docs/howto/rust.md` updated: `alg_cdc_chunks` return type corrected to `IsccResult<Vec<&[u8]>>` ✅
- `docs/ruby-api.md` API reference page (781 lines) ✅
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22662032256: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22662032256
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench, Ruby ✅
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- `release.yml` fixed: `cargo test --workspace --exclude iscc-rb` (prevents Ruby compilation
    conflict in non-Ruby test context) ✅
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1` ✅
- **Gap**: Target now requires release workflow smoke-tests (install built artifact, run conformance
    tests) before each publish step — none implemented yet (`normal` priority)
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)
- RubyGems account setup complete: account created, `GEM_HOST_API_KEY` secret added to GitHub ✅
- Gem name `iscc-lib` claimed on first `gem push` (no pre-reservation on RubyGems)

## Next Milestone

Two `normal`-priority gaps are now actionable by the CID loop:

1. **Fix Go `AlgCdcChunks` validation** (`packages/go/cdc.go`): add `avgChunkSize < 2` guard
    returning an `error` — mirrors the Rust core fix. Includes a test for the error case and update
    to `docs/howto/go.md`.

2. **Add release smoke tests** (`release.yml`): add `test-<artifact>` jobs between each `build-*`
    and `publish-*` step (6 pipelines: PyPI, npm-lib, npm-wasm, RubyGems, Maven, FFI). Each job
    installs the built artifact and runs the binding's conformance suite before gating publish.

CI is green — no CI fix required first.
