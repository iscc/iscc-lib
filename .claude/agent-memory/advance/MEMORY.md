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

- `cargo build -p iscc-jni` before `mvn test -f crates/iscc-jni/java/pom.xml` (native lib prereq)
- CI workflow `.github/workflows/ci.yml` has 18 job entries (version-check, rust, python-test,
    python, nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver,
    coverage). `bench` = `cargo bench --no-run`. `swift` on `macos-14`; `kotlin` on `ubuntu` JDK 17
    \+ `cargo build -p iscc-uniffi` + `./gradlew test`
- `coverage` CI job (`Coverage + CRAP`): standalone, NO `continue-on-error`. toolchain +
    `llvm-tools-preview` → `cargo binstall -y --force {cargo-llvm-cov,cargo-crap@0.2.2}` (`--force`
    LOAD-BEARING, rust-cache gotcha) → `cargo llvm-cov -p iscc-lib --lcov` → upload `lcov` →
    report-only `cargo crap --format github`+`sarif` (`upload-sarif@v3`, job perms
    `security-events: write`) → enforcing LAST step runs `cargo crap` with --baseline +
    --fail-regression vs `.crap-baseline.json` (exit 1 if a fn's CRAP rose > `--epsilon` 0.01).
    Local: `mise run coverage|crap|crap:baseline`
- `.crap-baseline.json` (repo root, COMMITTED, NOT gitignored): `{$schema, version, entries}`, 97
    fns/10 src files. Regen `cargo crap ... --format json --output .crap-baseline.json` (NO `--sort`
    in 0.2.2). GOTCHA: never pipe `cargo crap` into `tail`/`head` to check exit — `$?` = pager,
    masks exit 1; redirect to a file first. `.cargo-crap.toml`: `threshold=30.0`,
    `missing="pessimistic"`, `exclude` globs MUST list `crates/iscc-lib/benches/**` (built-in
    excludes are repo-root only → harness leaks at CRAP ~42)
- `semver` CI job: `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, baseline = last
    crates.io release. `continue-on-error: true` — INFORMATIONAL pre-1.0 (post-0.4.0 `pub(crate)`
    narrowing reports as breaking; expected). Drop `continue-on-error` at v1.0.0. Local:
    `mise run semver`
- Ruby CI job: libclang-dev required, ruby/setup-ruby@v1 `working-directory` is an action `with:`
    param (not step-level), bundler-cache auto-installs gems. `rust` job feature matrix: clippy+test
    for `--no-default-features`, `--all-features`,
    `--no-default-features --features text-processing`
- `version-check` job: `scripts/version_sync.py --check` (16 targets incl. Swift Constants,
    Package.swift releaseTag, Kotlin; exits 1 on mismatch). Go CI job has zero Rust deps
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds (`maturin` not on PATH
    — always `uv run maturin`). Builds a single `cp310-abi3` wheel (abi3-py310)
- PyO3 pin = single source: root `Cargo.toml` (`pyo3` "0.29", `abi3-py310`); ONLY `crates/iscc-py`
    consumes it (0.23→0.29 done, iter 105, #1 closed). KEEP explicit
    `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) — 0.28+ defaults `gil_used`⇒`false`,
    unsafe for raw `PyList_GetItem` ptrs. Recipe → MEMORY-archive.md. `cargo audit` NOT in CI
- Release workflow (`release.yml`): 9 boolean inputs, pattern input → build → **smoke test** →
    publish. Full input list + per-registry auth + release-job CI internals → MEMORY-archive.md
- wasm-pack `--features` goes AFTER the path, NOT after `--`; test runner accepts only a positional
    FILTER (`-- --test unit` fails), so run the full suite

## Benchmarks

- Two benches in `crates/iscc-lib/benches/`, both `harness = false`: `benchmarks.rs` (criterion,
    wall-clock + throughput, incl `gen_sum_code_v0` via tempfile) and `iai_benches.rs`
    (iai-callgrind `0.16`→0.16.1, instruction-counts, v1.0.0 perf gate #3). Share
    `deterministic_bytes`/`synthetic_text` builders
- iai harness COMPILES without valgrind/runner; running needs them (devcontainer HAS valgrind 3.19 +
    runner 0.16.1; local `mise run bench:iai`). `Perf (iai-callgrind)` CI job steps: apt valgrind →
    binstall runner (rust-cache `--force` gotcha) → run benches → zero-collection guard → enforcing
    `Check perf regression` step → upload `target/iai/` as `iai-baseline` (`if: always()`)
- PERF REGRESSION GATE (iter 109 slice 2b + iter 110 hardening DONE, issue #3 + [review] issue):
    `scripts/iai_regression.py` (stdlib only, no uv in CI) + committed `.iai-baseline.json` (repo
    root, NOT gitignored, 16 Ir entries). On-disk leaf dir `<bench_fn>.<bench_id>` = JSON key; parse
    first int of the `summary: <Ir> ...` line. Gates Ir ONLY.
    `check_regressions(run, baseline,   allow_missing=False)` returns False (exit 1) if ANY of: a
    shared bench >`baseline*1.10`; a shared bench current Ir == 0 (`zero_benches`,
    partial-strip/harness false-green; independent of `--allow-missing`); a baselined bench missing
    from the run (`only_baseline`) UNLESS `--allow-missing`. only-in-run STILL warns only
    (deliberate — new benches don't fail until a `--update` refresh). `--update --from-dir DIR`
    rebuilds. Glob skips `.out.old`. COMMITTED baseline MUST be CI-sourced (download green Perf
    `iai-baseline` artifact, then `--update`) to match CI rustc. Tasks `bench:iai:baseline` +
    `bench:iai:check`. Tests `tests/test_iai_regression.py` (synthetic temp `.out` dirs, never a
    live run; load script by path like `test_cid.py`). Mirrors `.crap-baseline`
- Two committed bench-config facts (iter 108; full write-up in learnings.md): root `Cargo.toml`
    `[profile.bench] strip = false, debug = true` (else stripped binary → `summary: 0` false green,
    caught by the CI guard); `IAI_CALLGRIND_ALLOW_ASLR=true` (mise + ci.yml) skips iai's
    `setarch -R` that the devcontainer kernel blocks (ASLR = cache noise, not `Ir`)
- Harness-authoring details (`#[library_benchmark]`/`library_benchmark_group!`/`main!` API,
    `black_box`, proc-macro docstring-rejection, borrow-returning GOTCHAs) → MEMORY-archive.md.
    Harness is complete + correct; do not edit it for the perf-gate work

## Streaming

- `DataHasher`: persistent `buf: Vec<u8>` reused across `update()`. CDC → BLAKE3 chunk hash →
    MinHash. Tail: `copy_within` + `truncate`. ~1.1 GiB/s at 64 KiB. `InstanceHasher`: wraps BLAKE3
    → ISCC multihash (64-byte digest truncated)
- `SumHasher` (issue #37): inner `DataHasher` + `InstanceHasher`, `update` feeds both;
    `finalize(bits, wide, add_units)` composes via `gen_iscc_code_v0`. Full path
    `iscc_lib::streaming::SumHasher` (NOT crate-root). Bindings: Python `PySumHasher`, WASM
    `SumHasher` (`Option<inner>` finalize-once). All 7 bindings implement `gen_sum_code_v0` (thin
    `streaming::SumHasher` wrapper). Python GIL release (#39, archived). gen_sum details →
    MEMORY-archive.md

## API Design

- Video API uses `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), other
    bindings pass `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields + validates with
    `TryFrom<u8>`. Delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix and dashes, returns exact digest bytes (not full tail) as
    `(u8,u8,u8,u8,Vec<u8>)`; `MainType` is `pub(crate)`
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key
- 5 constants exported across bindings: META_TRIM_NAME/DESCRIPTION/META, IO_READ_SIZE,
    TEXT_NGRAM_SIZE (per-binding export patterns → MEMORY-archive.md)

## Documentation Files

- Tabbed syntax: `=== "Language"` 4-space indent, blank line before code block. Landing-page tab
    order: Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin (11)
- Howto guides `docs/howto/{lang}.md`, API refs
    `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`, per-package READMEs + CLAUDE.md under
    `packages/{dotnet,cpp,swift,kotlin}/`. zensical.toml nav howto order: Rust, Python, Ruby,
    Node.js, WASM, Go, Java, C#/.NET, C/C++, Swift, Kotlin (differs from landing-page tab order
    above, which starts Python, Rust)
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
