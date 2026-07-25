# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations. Detail lives in topic files: [ci-gates.md](ci-gates.md) (coverage/CRAP, cargo-deny
audit, semver, iai perf gates), [uniffi-swift-kotlin.md](uniffi-swift-kotlin.md) (UniFFI, Swift,
Kotlin), [deps-refresh.md](deps-refresh.md) (held-back majors, ruff hold-back, refresh slices),
[wasm-simd.md](wasm-simd.md) (BLAKE3 SIMD wiring + wasm-pack gotchas), [go-idv1.md](go-idv1.md) (Go
IDv1 + decode guards). Archived phases: [MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Unicode 16.0.0 freeze rule (iter 133): `text_clean`/`text_collapse` strip 16.0-unassigned code
    points BEFORE normalization via generated `crates/iscc-lib/src/utils/unicode16.rs` (731 ranges;
    regen with `uv run --script scripts/gen_unicode16_unassigned.py` — PEP 723, pins
    `unicodedata2==16.0.0`, ty-excluded in pyproject.toml). Keep filter first in both chains.
    Pending: binding boundary vectors (step b), full-code-space differential sweep (step c)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`. Node.js:
    `crates/iscc-napi/src/lib.rs`. WASM: `crates/iscc-wasm/src/lib.rs`. C FFI:
    `crates/iscc-ffi/src/lib.rs`. JNI: `crates/iscc-jni/src/lib.rs` +
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (wrapper + Result classes).
    Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go). WASM bridge removed — pure Go only.
    Experimental ISCC-IDv1, exact body-length decode guards, gofmt caveat → go-idv1.md

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn test -f crates/iscc-jni/java/pom.xml` (native lib prereq)
- CI workflow `.github/workflows/ci.yml` has 19 job entries (version-check, rust, python-test,
    python, nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver,
    coverage, audit). `bench` = `cargo bench --no-run`. `swift` on `macos-14`; `kotlin` on `ubuntu`
    JDK 17 + `cargo build -p iscc-uniffi` + `./gradlew test`
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json` +
    `--fail-regression   --fail-above`), `audit` (cargo-deny, `deny.toml`), `perf` (iai regression
    vs `.iai-baseline.json`). `semver` informational until v1.0.0. CRAP gate is CI-ONLY — refresh
    baseline (`mise run crap:baseline`) in the SAME step as any branch-adding change. Full mechanics
    \+ gotchas → ci-gates.md
- Key audit rule: fresh advisory with a patched release → `cargo update -p <crate>` lockfile bump,
    NEVER add to `deny.toml` `ignore` (iter 115: crossbeam-epoch 0.9.18→0.9.20)
- Dependency refresh (iters 124-131): Cargo.lock, uv.lock, ci.yml/docs.yml GHA actions, JVM
    manifests, Go module, Ruby Gemfile all refreshed; napi/dotnet verified current (wildcard
    floats). Held-back majors (criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32) carry `# held:`
    comments in root Cargo.toml; `ruff<0.16` hold in pyproject.toml (slice A cleared 78/104
    findings; 26 left, each needs a config/gate decision); rb_sys pinned `0.9.123` + minitest 5.x
    held in Gemfile. Reasons, junit-platform-launcher/Gradle gotcha + remaining slices (release.yml,
    ruff 0.16 B/C, majors) → deps-refresh.md
- GOTCHA: ci.yml concurrency has `cancel-in-progress: true` per ref — pushing a second develop
    commit cancels the in-flight CI run of the previous sha (its check-runs end "cancelled"). When a
    step needs a green CI on a specific sha, don't push again until it concludes
- System `python3` lacks PyYAML — use `uv run python` for YAML validation snippets
- GOTCHA: piping `cargo crap`/`cargo deny` to `tail` makes `$?` the pager's exit — use a file
- Ruby CI job: libclang-dev required; ruby/setup-ruby@v1 `working-directory` is an action `with:`
    param; bundler-cache auto-installs gems. `rust` job matrix: `--no-default-features`,
    `--all-features`, `--no-default-features --features text-processing`
- `version-check` job: `scripts/version_sync.py --check` (16 targets incl. Swift Constants,
    Package.swift releaseTag, Kotlin; exits 1 on mismatch). Go CI job has zero Rust deps
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds (`maturin` not on PATH
    — always `uv run maturin`). Builds a single `cp310-abi3` wheel (abi3-py310). GOTCHA: `uv sync`
    uninstalls the editable ext — re-run maturin develop after every sync
- PyO3 pin = single source: root `Cargo.toml` (`pyo3` "0.29", `abi3-py310`); ONLY `crates/iscc-py`
    consumes it. KEEP `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) — 0.28+ defaults
    `gil_used` to `false`, unsafe for raw `PyList_GetItem` ptrs. Recipe → MEMORY-archive.md
- GIL release complete (iters 111+116, #39/#41): 12 `py.detach` sites in
    `crates/iscc-py/src/lib.rs`. Video detach MUST open after frame-sig extraction; meta/audio/mixed
    stay attached by design. Tests: `tests/test_gil.py`
- Release workflow (`release.yml`): 9 boolean inputs → build → **smoke test** → publish (inputs,
    auth, CI internals → MEMORY-archive.md). `build-wheels` has 4 targets incl native-ARM aarch64
    (iter 123, #49); `test-wheels` matrixed, artifact name = `wheels-<os>-<target>`
- WASM SIMD (iters 117-118, #42): dual wiring required (`blake3/wasm32_simd` feature + RUSTFLAGS
    simd128 + wasm-opt `--enable-simd`); verification recipe + wasm-pack CLI gotchas → wasm-simd.md

## Benchmarks

- Two benches in `crates/iscc-lib/benches/`, both `harness = false`: `benchmarks.rs` (criterion 0.7,
    wall-clock + throughput; uses `std::hint::black_box` — `criterion::black_box` deprecated since
    0.6, fails clippy `-D warnings`) and `iai_benches.rs` (iai-callgrind 0.16.1,
    instruction-counts). iai runs need valgrind + runner — NOT preinstalled in fresh containers:
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`,
    then `mise run bench:iai:check`. Perf gate mechanics + bench-profile gotchas → ci-gates.md

## Streaming

- `DataHasher`: persistent `buf: Vec<u8>` reused across `update()`. CDC → BLAKE3 chunk hash →
    MinHash. Tail: `copy_within` + `truncate`. ~1.1 GiB/s at 64 KiB. `InstanceHasher`: wraps BLAKE3
    → ISCC multihash (64-byte digest truncated)
- `SumHasher` (issue #37): inner `DataHasher` + `InstanceHasher`, `update` feeds both;
    `finalize(bits, wide, add_units)` composes via `gen_iscc_code_v0`. Full path
    `iscc_lib::streaming::SumHasher` (NOT crate-root). Bindings: Python `PySumHasher`, WASM
    `SumHasher` (`Option<inner>` finalize-once). All 7 bindings implement `gen_sum_code_v0`. gen_sum
    details → MEMORY-archive.md

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
    `packages/{dotnet,cpp,swift,kotlin}/`. zensical.toml nav howto order differs from landing-page
    tab order (nav starts Rust, Python; landing starts Python, Rust)
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md` (via `ORDERED_PAGES` +
    `discover_pages()`, excludes `docs/includes/`). Run after `zensical build` in docs CI

## Feature Flags

- `crates/iscc-lib/Cargo.toml` defines: `default = ["meta-code"]`, `text-processing` (unicode deps),
    `meta-code` (implies text-processing + JCS canonicalizer)
- `text-processing` gates: `text_clean`, `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`.
    `meta-code` gates: META_TRIM constants, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests` in conformance, `sliding_window_bytes`
- `conformance` module is always available (not feature-gated). `conformance_selftest()` skips
    disabled code types (meta, text) via `#[cfg]` blocks — does not fail for missing features
- When gating `pub(crate)` functions, gate their tests too — dead-code lint fires in library builds
    even if test modules use them; `tests/test_text_utils.rs` also needs per-function gating
- `serde_json` stays as a regular (non-optional) dep because `conformance.rs` uses it for parsing
    `data.json`. Gating it requires restructuring conformance (future work)

## Ruby Bindings (Magnus) — see MEMORY-archive.md for full details

- Magnus 0.7.1 (not 0.8) — Ruby 3.1 compat. `function!` macro: no `&Ruby` param, use `Ruby::get()`
- rb_sys: `ExtensionTask.new("iscc-rb")` — task name = Cargo package name. `extconf.rb` at crate
    root. 32/32 Tier 1 symbols exposed. 111 tests (61 unit + 50 conformance)
- Streaming: `RefCell<Option<inner>>` for one-shot finalize. `_` prefix for methods, NOT class names
- Linting: Standard Ruby + rubocop-minitest. Pre-commit hook needs portable PATH for `bundle`

## Other Bindings — pointers

- .NET: `packages/dotnet/` — P/Invoke over `iscc_ffi`, 32/32 Tier 1 symbols, `dotnet-version: 8.0`.
    C++: `packages/cpp/` — header-only C++17, depends on `iscc-ffi`, CMake + ASAN tests. Full detail
    → MEMORY-archive.md
- UniFFI scaffolding, Swift package (two Package.swift, XCFramework script), Kotlin/JVM (Gradle +
    JNA) → uniffi-swift-kotlin.md
