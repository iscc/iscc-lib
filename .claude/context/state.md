<!-- assessed-at: 064a4328011eee201ee3423725824a331057e8d8 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings complete; only low-priority work remains

All high-priority binding work is complete. Ruby bindings have been fully met since iteration 14.
Iterations 15–16 made no source code changes; only CID loop administrative context files were
updated. All 12 CI jobs pass in the latest run (22657251138). The only remaining gaps are
low-priority targets (C#, C++, Swift, Kotlin bindings) which the CID loop skips by policy.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `data.json` updated to iscc-core v1.3.0 (50 total vectors; 4 new: test_0017–test_0020) ✅
- Rust conformance assertion updated: `assert_eq!(tested, 20, ...)` in `lib.rs` ✅
- 314 tests pass with default features ✅
- Feature matrix CI (5 steps) passed in the latest green run ✅

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

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]` ✅
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20` ✅
- `WASM (wasm-pack test)` = SUCCESS in CI run 22657251138 ✅

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
- `parseConformanceData()` helper skips `_metadata` key in data.json ✅
- `packages/go/testdata/data.json` updated to iscc-core v1.3.0 (50 vectors) ✅
- 155 Go tests pass; `go vet` clean ✅; CGO_ENABLED=0 confirmed ✅
- `docs/howto/go.md` code examples corrected (GenSumCodeV0 4-arg signature) ✅
- README.md Go quickstart corrected (direct `iscc.GenMetaCodeV0(...)` call, no WASM runtime) ✅

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
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
- Pure Ruby wrapper: 10 result classes (`*CodeResult < Result < Hash`), keyword args, method
    chaining for streaming types ✅
- **Conformance tests**: 50 dynamically generated test methods covering all 9 gen\_\*\_v0 functions
    ✅
- 111 Minitest tests total (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- **Dedicated `ruby` CI job** — runs standardrb, clippy, compile, and test on ubuntu-latest / Ruby
    3.1 ✅
- `crates/iscc-rb/lib/iscc_lib/version.rb` exists; synced by `version_sync.py` ✅
- `crates/iscc-rb/README.md` (93 lines — installation, quickstart, API overview) ✅
- **RubyGems publish step** in `release.yml`: `build-gem` (5 platforms, Ruby 3.1/3.2/3.3 via
    `oxidize-rb/actions/cross-gem@v1`) + `publish-rubygems` (idempotency check, `GEM_HOST_API_KEY`)
    ✅
- `docs/howto/ruby.md` (422 lines) ✅; **`docs/ruby-api.md`** (781 lines — all 32 symbols with
    signatures, parameter tables, return types, code examples) ✅
- `zensical.toml` Reference section: "Ruby API" nav entry added ✅
- Root `README.md` Ruby section (install tab + quickstart) ✅
- **Standard Ruby linting** fully configured ✅ (`.standard.yml`, Gemfile, CI step, pre-commit/push
    hooks)
- **Human-action note**: RubyGems.org account creation, gem name reservation, and `GEM_HOST_API_KEY`
    secret setup still require manual steps before first release — all automation is in place

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
- Ruby install instructions and quickstart now present ✅
- Go quickstart fixed (direct function call, no stale WASM runtime pattern) ✅
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
- `docs/howto/go.md` updated with correct `GenSumCodeV0` 4-arg signature ✅
- `docs/ruby-api.md` API reference page (781 lines); nav entry in zensical.toml ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22657251138: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22657251138
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench, Ruby (standardrb + clippy + compile + test) ✅
- `release.yml` has **6 registry** `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- `build-gem` job: 5 platforms (x86_64-linux, aarch64-linux, x86_64-darwin, arm64-darwin,
    x64-mingw-ucrt) via `oxidize-rb/actions/cross-gem@v1` ✅
- `publish-rubygems` job: idempotency check, source gem fallback, `GEM_HOST_API_KEY` secret ✅
- Rust CI job uses `--exclude iscc-rb` — Ruby clippy covered by dedicated ruby job ✅
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)
- **Note**: RubyGems account setup, gem name reservation, and `GEM_HOST_API_KEY` secret still
    require human action before first release

## Next Milestone

All `critical` and `normal` priority work is complete. The CID loop is in a **maintenance / idle
state** — no automatable gaps remain. Possible next actions (all require human decision):

1. **Merge `develop` → `main`** and cut an initial release: run `mise run pr:main`, then trigger
    `release.yml` after human account setup on each registry (RubyGems, etc.)
2. **Tackle low-priority bindings** (C#, C++, Swift, Kotlin) — requires human approval to elevate
    priority in issues.md
3. **Validate `alg_cdc_chunks` edge case** (avg_chunk_size=0 hang) — review upstream `iscc-core`
    behavior first; low priority
4. **Add language logos** to README and docs — cosmetic; low priority
