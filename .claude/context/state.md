<!-- assessed-at: f1304248acb603e627d24e76d75c84c78b917bc4 -->

# Project State

## Status: IN_PROGRESS

## Phase: Ruby bindings — release infra complete; linting, docs remaining

The Ruby Magnus bridge exposes all 32 Tier 1 symbols with full conformance tests (111 tests, 295
assertions, 50 vectors). This iteration added the RubyGems publish step to `release.yml` (6-platform
`build-gem` job + `publish-rubygems` job with idempotency check), completing release infrastructure.
All 12 CI jobs remain green (run 22650345378). Remaining Ruby work: Standard Ruby linting,
`docs/howto/ruby.md`, full `crates/iscc-rb/README.md`, and Ruby section in root `README.md`.

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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22650345378 ✅

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

## Ruby Bindings

**Status**: partially met

- `crates/iscc-rb/` scaffold with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
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
- **Conformance tests**: `test/test_conformance.rb` — 50 dynamically generated test methods covering
    all 9 gen\_\*\_v0 functions against official data.json vectors ✅
- 111 Minitest tests total (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- **Dedicated `ruby` CI job** — runs clippy, compile, and test on ubuntu-latest / Ruby 3.1 ✅
- `crates/iscc-rb/lib/iscc_lib/version.rb` exists; synced by `version_sync.py` ✅
- `version_sync.py --check` reports `OK: crates/iscc-rb/lib/iscc_lib/version.rb = 0.1.0` ✅
- `crates/iscc-rb/README.md` exists (stub, 31 lines) ✅
- **RubyGems publish step** added to `release.yml`: `build-gem` (5 platforms, Ruby 3.1/3.2/3.3 via
    `oxidize-rb/actions/cross-gem@v1`) + `publish-rubygems` (idempotency check, `GEM_HOST_API_KEY`)
    ✅
- **Missing**: Standard Ruby linting not configured (no `standard` gem in Gemfile, no
    `.standard.yml`, not wired into pre-commit hooks or CI)
- **Missing**: `docs/howto/ruby.md` guide does not exist; no `docs/ruby-api.md`
- **Missing**: No Ruby section in root `README.md`
- **Missing**: `crates/iscc-rb/README.md` is a stub — full installation/usage instructions not
    complete

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
- **Gap**: Ruby install instructions and quickstart not present
- **Gap**: C#, C++, Swift, Kotlin sections not present (target requires all 4; all `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages (including `crates/iscc-rb/README.md`) ✅
- **Gap**: `crates/iscc-rb/README.md` is a stub (31 lines); full installation/usage instructions not
    yet complete
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet (all `low` priority)

## Documentation

**Status**: partially met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 7 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- **Gap**: No `docs/howto/ruby.md` guide; no `docs/ruby-api.md`; no Ruby tabs in multi-language
    examples
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22650345378: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22650345378
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench, Ruby ✅
- `release.yml` now has **6 registry** `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven,
    FFI, **RubyGems** ✅
- `build-gem` job: 5 platforms (x86_64-linux, aarch64-linux, x86_64-darwin, arm64-darwin,
    x64-mingw-ucrt) via `oxidize-rb/actions/cross-gem@v1` ✅
- `publish-rubygems` job: idempotency check, source gem fallback, `GEM_HOST_API_KEY` secret ✅
- Rust CI job still uses `--exclude iscc-rb` — Ruby clippy covered by dedicated ruby job ✅
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)
- **Note**: RubyGems account setup, gem name reservation, and `GEM_HOST_API_KEY` secret still
    require human action before first release

## Next Milestone

Release infrastructure for Ruby is now complete. The next work package should focus on Ruby
documentation:

1. **How-to guide** — write `docs/howto/ruby.md` (installation via Bundler, gem usage examples,
    streaming API, conformance selftest; follow the pattern of existing `go.md` / `java.md`)
2. **Expand README** — `crates/iscc-rb/README.md` stub (31 lines) → full guide with install,
    quickstart, API overview, contributing notes
3. **Root README Ruby section** — add Ruby install badge + quickstart snippet alongside the other 6
    languages
4. **Standard Ruby linting** (lower priority) — add `standard` gem + `rubocop-minitest` to Gemfile;
    create `.standard.yml`; wire `standardrb` check into the `ruby` CI job
