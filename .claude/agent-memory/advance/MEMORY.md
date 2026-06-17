# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`
- Node.js: `crates/iscc-napi/src/lib.rs`
- WASM: `crates/iscc-wasm/src/lib.rs`
- C FFI: `crates/iscc-ffi/src/lib.rs`
- JNI: `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (Ruby wrapper + Result
    classes), lib/iscc_lib/version.rb, extconf.rb, Rakefile, Gemfile, iscc-lib.gemspec,
    test/test_smoke.rb. Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — src/lib.rs (proc macro interface for Swift/Kotlin). 32 Tier 1
    symbols, 11 result Records, IsccUniError enum, DataHasher/InstanceHasher Objects. Uses
    `uniffi::setup_scaffolding!()`, no UDL or build.rs. `publish = false`
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go). WASM bridge removed — pure Go only

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven POM is at `crates/iscc-jni/java/pom.xml` — run `mvn test` from `crates/iscc-jni/java/`
- CI workflow `.github/workflows/ci.yml` has 17 job entries (version-check, rust, python-test,
    python, nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, semver,
    coverage). `bench` = `cargo bench --no-run`. `swift` on `macos-14`; `kotlin` on `ubuntu` JDK 17
    \+ `cargo build -p iscc-uniffi` + `./gradlew test`
- `coverage` CI job (named `Coverage + CRAP`): standalone, no `needs:`, NO `continue-on-error`.
    toolchain+`llvm-tools-preview` → install `cargo-llvm-cov` + `cargo-binstall`
    (`taiki-e/install-action@v2`) → `cargo binstall -y --force cargo-crap@0.2.2` (`--force`
    LOAD-BEARING, iter 100: rust-cache restores `.crates.toml` metadata WITHOUT the cargo-crap
    binary → plain binstall skips → `cargo crap` dies "no such command" → CI RED) →
    `cargo llvm-cov -p   iscc-lib --lcov --output-path lcov.info` → upload-artifact (`name: lcov`) →
    report-only `cargo crap --format github` + `--format sarif --output crap.sarif` →
    `upload-sarif@v3`. Job-level `permissions: {contents: read, security-events: write}` (SARIF
    upload). Local mirror tasks `coverage`, `crap`, `crap:baseline` (all `depends=["coverage"]`).
    `lcov.info`+`crap.sarif` gitignored. Phase 3 (iter 97): enforcing `CRAP regression gate` (LAST
    step, after SARIF upload):
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression` — exits 1 if
    any function's CRAP rose beyond `--epsilon` (default 0.01)
- `.crap-baseline.json` (repo root, iter 97): COMMITTED CRAP baseline, NOT gitignored (only
    `lcov.info`+`crap.sarif` are). Envelope `{$schema, version:"0.2.2", entries:[...]}` — 97
    `iscc-lib` functions, 10 src files (excludes filter binding crates+benches). 18.8KB/782 lines.
    Generated: `cargo crap --lcov lcov.info --format json --output .crap-baseline.json`. Do NOT pass
    `--sort` (only on cargo-crap `main`, not 0.2.2). Refreshed in a reviewed commit, NOT CI
    auto-commit (would race CID loop pushes). GOTCHA: when checking gate exit codes, never pipe
    `cargo crap` into `tail`/`head` — `$?` reflects the pager, masking exit 1. Redirect to file then
    check `$?`
- `.cargo-crap.toml` (repo root, iter 96): keys `threshold=30.0`, `missing="pessimistic"`, `exclude`
    globs — excludes 7 binding crates + `packages/**` + `scripts/**` + `crates/iscc-lib/benches/**`.
    GOTCHA: built-in default excludes skip nested `tests/**` but NOT nested `benches/**` (matches
    repo-root only) → bench harness leaks at CRAP ~42 unless excluded. `--format github` silent
    below threshold
- `semver` CI job (iter 93): `obi1kenobi/cargo-semver-checks-action@v2` with `package: iscc-lib`,
    baseline = last crates.io release (auto-detected). `continue-on-error: true` — INFORMATIONAL
    pre-1.0 (post-0.4.0 `pub(crate)` narrowing of cdc/conformance/minhash/simhash/utils reports as
    breaking; expected). Drop `continue-on-error` at v1.0.0 to enforce. Local: `mise run semver`
    (`cargo semver-checks check-release -p iscc-lib`)
- Ruby CI job: libclang-dev required, ruby/setup-ruby@v1 `working-directory` is an action `with:`
    param (not step-level), bundler-cache auto-installs gems
- `rust` CI job feature matrix: clippy + test for `--no-default-features`, `--all-features`, and
    `--no-default-features --features text-processing` (issue #16)
- `version-check` job (checkout + setup-python only): `scripts/version_sync.py --check` (16 targets
    incl. Swift Constants, Package.swift releaseTag, Kotlin; exits 1 on mismatch)
- Go CI job has zero Rust dependencies — only checkout, setup-go, test, vet (4 steps)
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds. `maturin` is not on
    PATH — always invoke via `uv run maturin`. Builds a single `cp310-abi3` wheel (abi3-py310)
- PyO3 pin = single source: root `Cargo.toml` `[workspace.dependencies]`
    `pyo3 = { version, features   = ["abi3-py310"] }`. ONLY `crates/iscc-py` consumes it
    (`features = ["extension-module"]`); blast radius = `crates/iscc-py/src/lib.rs` only. Migration
    in progress 0.23→0.29 (RustSec advisories clear at 0.29), incremental one-minor-per-step.
    0.23→0.24 (iter 98) AND 0.24→0.25 (iter 99) both needed ZERO src changes — the
    `dict.into()`/`PyBytes::new(py,_).into()`/`.into_pyobject(py)?.into()`/raw `pyo3::ffi::*`
    +`Bound::from_owned_ptr` idioms in lib.rs all compile clean through 0.25 (predicted
    `IntoPyObject`/lifetime breaks did NOT materialize at 0.24 or 0.25 — may still hit at 0.26+).
    After bump: `cargo update -p   pyo3` → build/clippy(`-D warnings`)/fmt →
    `uv run maturin develop` → `uv run pytest` (286 tests). NEXT HOP: 0.25→0.26
- Release workflow (`release.yml`): 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget, maven-kotlin, swift). Pattern: input → build → **smoke test** → publish. NuGet uses
    `NUGET_API_KEY` (not OIDC); Ruby uses OIDC. npm `@iscc/lib` bundled single-package + release-job
    CI internals (`build-xcframework`, Kotlin Maven Central) → MEMORY-archive.md
- wasm-pack `--features` goes AFTER the path, NOT after `--`. Test-target filter (`-- --test unit`)
    fails — runner only accepts a positional FILTER; run full suite

## gen_sum_code_v0 — see MEMORY-archive.md for full details

- All 32 Tier 1 symbols implemented; all 7 bindings implement `gen_sum_code_v0`. `gen_sum_code_v0`
    is a thin file-I/O wrapper over `streaming::SumHasher`. `iscc_decode` returns
    `(u8,u8,u8,u8,Vec<u8>)`; `MainType` is `pub(crate)`

## Streaming

- `DataHasher`: persistent `buf: Vec<u8>` reused across `update()`. CDC → BLAKE3 chunk hash →
    MinHash. Tail: `copy_within` + `truncate`. ~1.1 GiB/s at 64 KiB. `InstanceHasher`: wraps BLAKE3
    → ISCC multihash (64-byte digest truncated)
- `SumHasher` (issue #37): inner `DataHasher` + `InstanceHasher`, `update` feeds both;
    `finalize(bits, wide, add_units)` composes via `gen_iscc_code_v0`. Full path
    `iscc_lib::streaming::SumHasher` (NOT crate-root). Bindings: Python `PySumHasher`, WASM
    `SumHasher` (`Option<inner>` finalize-once). Python GIL release (#39, archived)

## API Design

- Video API uses `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), other
    bindings pass `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields + validates with
    `TryFrom<u8>`. Delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix and dashes, returns exact digest bytes (not full tail)
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key
- 5 constants exported across bindings: META_TRIM_NAME/DESCRIPTION/META, IO_READ_SIZE,
    TEXT_NGRAM_SIZE (per-binding export patterns → MEMORY-archive.md)

## Documentation

- Tabbed syntax: `=== "Language"` 4-space indent, blank line before code block. Landing page tab
    order: Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin (11)

## Documentation Files

- Howto guides `docs/howto/{lang}.md`, API refs
    `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`, per-package READMEs + CLAUDE.md under
    `packages/{dotnet,cpp,swift,kotlin}/`. zensical.toml nav howto order: Rust, Python, Ruby,
    Node.js, WASM, Go, Java, C#/.NET, C/C++, Swift, Kotlin (NOTE: landing-page tab order differs —
    starts Python, Rust per the Documentation section above)
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md` (via `ORDERED_PAGES` +
    `discover_pages()`, excludes `docs/includes/`). Run after `zensical build` in docs CI

## Feature Flags

- `crates/iscc-lib/Cargo.toml` defines: `default = ["meta-code"]`, `text-processing` (unicode deps),
    `meta-code` (implies text-processing + JCS canonicalizer)
- `text-processing` gates: `text_clean`, `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`
- `meta-code` gates: META_TRIM constants, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests` in conformance, `sliding_window_bytes`
- `conformance` module is always available (not feature-gated). `conformance_selftest()` skips
    disabled code types (meta, text) via `#[cfg]` blocks — does not fail for missing features
- When gating `pub(crate)` functions, their tests must also be gated — dead-code lint fires in
    library builds even if test modules use them
- Integration tests in `crates/iscc-lib/tests/test_text_utils.rs` also need per-function gating
- `serde_json` stays as a regular (non-optional) dep because `conformance.rs` uses it for parsing
    `data.json`. Gating it requires restructuring conformance (future work)

## Ruby Bindings (Magnus) — see MEMORY-archive.md for full details

- Magnus 0.7.1 (not 0.8) — Ruby 3.1 compat. `function!` macro: no `&Ruby` param, use `Ruby::get()`
- rb_sys: `ExtensionTask.new("iscc-rb")` — task name = Cargo package name. `extconf.rb` at crate
    root
- 32/32 Tier 1 symbols exposed. 111 tests (61 unit + 50 conformance)
- Streaming: `RefCell<Option<inner>>` for one-shot finalize. `_` prefix for methods, NOT class names
- Linting: Standard Ruby + rubocop-minitest. Pre-commit hook needs portable PATH for `bundle`

## .NET / C++ Bindings — see MEMORY-archive.md for full details

- .NET: `packages/dotnet/` — P/Invoke over `iscc_ffi`, 32/32 Tier 1 symbols, `dotnet-version: 8.0`
- C++: `packages/cpp/` — header-only C++17, depends on `iscc-ffi`, CMake + ASAN tests

## UniFFI Bindings (Swift/Kotlin)

- `crates/iscc-uniffi/` — shared scaffolding crate, `uniffi = "0.31"` (workspace dep)
- Proc macro approach only: `#[uniffi::export]`, `#[derive(uniffi::Record)]`,
    `#[derive(uniffi::Object)]`, `#[uniffi::constructor]`. No UDL files, no build.rs
- `crate-type = ["cdylib", "staticlib", "lib"]` — cdylib for dynamic, staticlib for XCFramework
- Error: `#[derive(uniffi::Error)] enum IsccUniError` with `From<iscc_lib::IsccError>` impl
- Streaming: `Mutex<Option<Inner>>` (like Ruby's `RefCell<Option<Inner>>` but thread-safe)
- UniFFI doesn't support: `const` exports (use getter fns), `usize` (use u64), borrowed refs (owned)
- Result records need `Debug` derive for test `unwrap_err()`. Hashers need `Default` impl (clippy)
- 21 unit tests in-crate. Conformance testing happens in Swift/Kotlin test suites
- Binding generation: `uniffi-bindgen.rs` (3-line main), `[features] bindgen = ["uniffi/cli"]`,
    `[[bin]] required-features = ["bindgen"]`. Generate Swift via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate --library   target/debug/libiscc_uniffi.so --language swift --out-dir <dir>`
    → emits `iscc_uniffi.swift`, `iscc_uniffiFFI.h`, `iscc_uniffiFFI.modulemap` (rename to
    `module.modulemap` for SPM)

## Swift Package

- Two `Package.swift` coexist: root (SPM consumers, reads for dep resolution) +
    `packages/swift/Package.swift` (CI/local dev). Root uses Ferrostar toggle `useLocalFramework`
    - `releaseTag`/`releaseChecksum`, `binaryTarget` for distribution; omits testTarget
- `scripts/build_xcframework.sh`: 5 Rust targets → `lipo` → `xcodebuild -create-xcframework` →
    `ditto` zip → checksum. Output `target/ios/IsccLib.xcframework.zip` (`--release`/`--debug`)
- Version constant: `packages/swift/Sources/IsccLib/Constants.swift` (`isccLibVersion`). CI job
    (`swift:`, `macos-14`): `cargo build -p iscc-uniffi` → `swift build` → `swift test` with
    `-Xlinker -L`/`-rpath` → `target/debug`

## Kotlin Bindings (UniFFI/JVM)

- `packages/kotlin/` — Gradle JVM project, UniFFI-generated Kotlin via JNA (mature/complete). Key
    facts: generated `src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (do NOT hand-edit —
    regenerate via uniffi-bindgen), JNA native loading needs `jna.library.path` + `LD_LIBRARY_PATH`,
    `ConformanceTest.kt` = 9 methods/50 vectors. Full generate command, Gradle/JNA versions,
    gitignore quirks, Maven Central publishing → MEMORY-archive.md
