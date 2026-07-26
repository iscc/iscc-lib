# Advance Agent Memory

Detail lives in topic files: [ci-gates.md](ci-gates.md),
[uniffi-swift-kotlin.md](uniffi-swift-kotlin.md), [deps-refresh.md](deps-refresh.md),
[wasm-simd.md](wasm-simd.md), [go-idv1.md](go-idv1.md). Archived phases:
[MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

- [issues.md ledger rule](feedback-issues-ledger.md) — never edit issues.md; ledger → handoff Notes

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Unicode 16.0.0 freeze rule (iter 133): `text_clean`/`text_collapse` strip 16.0-unassigned code
    points BEFORE normalization via generated `crates/iscc-lib/src/utils/unicode16.rs` (731 ranges;
    regen: `uv run --script scripts/gen_unicode16_unassigned.py` — PEP 723, pins
    `unicodedata2==16.0.0`). Keep filter first in both chains. Boundary fixture (iter 141):
    `crates/iscc-lib/tests/unicode_boundary.json` (ASCII-only, data.json-shaped, 4 code points × 2
    sections) + `tests/test_unicode_boundary.rs` — propagation source for bindings. Pending: binding
    propagation (blocked on parked ordering ruling + Go 15.0-tables decision), full-code-space
    differential sweep
- Bindings: Python `crates/iscc-py/python/iscc_lib/__init__.py`; Node `crates/iscc-napi/src/lib.rs`;
    WASM `crates/iscc-wasm/src/lib.rs`; C FFI `crates/iscc-ffi/src/lib.rs`; JNI
    `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (wrapper + Result classes).
    Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go). WASM bridge removed — pure Go only.
    Experimental ISCC-IDv1, exact body-length decode guards, gofmt caveat → go-idv1.md

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn test -f crates/iscc-jni/java/pom.xml` (native lib prereq)
- CI `.github/workflows/ci.yml`: 19 jobs (version-check, rust, python-test, python, nodejs, wasm,
    c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver, coverage, audit).
    `bench` = `cargo bench --no-run`; `swift` on macos-14; `kotlin` ubuntu JDK 17 + gradlew test
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json`), `audit` (cargo-deny,
    `deny.toml`), `perf` (iai vs `.iai-baseline.json`); `semver` informational until v1.0.0. CRAP
    gate is CI-ONLY — refresh baseline (`mise run crap:baseline`) in the SAME step as any
    branch-adding change. Full mechanics + gotchas → ci-gates.md
- Audit rule: advisory with patched release → `cargo update -p <crate>`, never `deny.toml` `ignore`
- Dependency refresh (iters 124-140): lockfiles/manifests current; held majors (criterion 0.8, jni
    0.22, magnus 0.8, uniffi 0.32) carry `# held:` in root Cargo.toml; ruff 0.16 (`ruff format`
    covers Markdown fences); prek ruff hooks carry `pyi` in `types_or` (CI parity — prek types
    `.pyi` as `pyi`, NOT `python`); NEVER blanket `--fix` (deletes 13 `# noqa: S603/S607`); rb_sys
    pinned `0.9.123`. Detail + prek staged-probe gotcha → deps-refresh.md
- GOTCHA: ci.yml concurrency has `cancel-in-progress: true` per ref — pushing a second develop
    commit cancels the in-flight CI run of the previous sha (its check-runs end "cancelled"). When a
    step needs a green CI on a specific sha, don't push again until it concludes
- GOTCHAs: system `python3` lacks PyYAML (use `uv run python`); piping `cargo crap`/`cargo deny` to
    `tail` makes `$?` the pager's exit — redirect to a file
- Ruby CI job: libclang-dev required; ruby/setup-ruby@v1 `working-directory` is a `with:` param.
    `rust` job matrix: `--no-default-features` / `--all-features` / `--features text-processing`
- `version-check` job: `scripts/version_sync.py --check` (16 targets incl. Swift Constants,
    Package.swift releaseTag, Kotlin; exits 1 on mismatch). Go CI job has zero Rust deps
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds (`maturin` not on PATH
    — always `uv run maturin`). Builds a single `cp310-abi3` wheel (abi3-py310). GOTCHA: `uv sync`
    uninstalls the editable ext — re-run maturin develop after every sync
- PyO3 pin = single source: root `Cargo.toml` (`pyo3` "0.29", `abi3-py310`); ONLY `crates/iscc-py`
    consumes it. KEEP `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) — 0.28+ defaults
    `gil_used` to `false`, unsafe for raw `PyList_GetItem` ptrs. Recipe → MEMORY-archive.md
- GIL release (iters 111+116, #39/#41): 12 `py.detach` sites in `crates/iscc-py/src/lib.rs`. Video
    detach MUST open after frame-sig extraction; meta/audio/mixed stay attached. `tests/test_gil.py`
- release.yml static gate (iter 142): `scripts/check_release_workflow.py` (guard shape, artifact
    wiring via matrix-include expansion + symmetric glob match, `needs:` graph) — prek hook
    `check-release-workflow` + `tests/test_check_release_workflow.py` in CI. Any release.yml edit
    must keep it green. `pyyaml` is an explicit dev dep
- Release workflow (`release.yml`): 9 boolean inputs → build → **smoke test** → publish (inputs,
    auth, CI internals → MEMORY-archive.md). `build-wheels` 4 targets incl native-ARM aarch64;
    `test-wheels` matrixed, artifact name = `wheels-<os>-<target>`. All 28 non-`prepare-release`
    jobs carry `!cancelled() && !failure()` `if:` guards (iter 139); 97 `uses:` refs at current
    majors (iter 140: checkout@v7, upload/download-artifact v7/v8 pair, gh-release@v3 — statically
    verified only, `workflow_dispatch`). Lint edits via
    `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7` (cached, offline-safe)
- WASM SIMD (#42): dual wiring (`blake3/wasm32_simd` + RUSTFLAGS simd128 + wasm-opt
    `--enable-simd`); recipe + wasm-pack gotchas → wasm-simd.md

## Benchmarks

- Two benches in `crates/iscc-lib/benches/`, both `harness = false`: `benchmarks.rs` (criterion 0.7,
    wall-clock; uses `std::hint::black_box` — `criterion::black_box` deprecated, fails clippy) and
    `iai_benches.rs` (iai-callgrind 0.16.1). iai needs valgrind + runner (NOT preinstalled):
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`,
    then `mise run bench:iai:check`. Perf gate mechanics + bench-profile gotchas → ci-gates.md

## Streaming

- `DataHasher`: persistent `buf` reused across `update()`; CDC → BLAKE3 chunk hash → MinHash; ~1.1
    GiB/s at 64 KiB. `InstanceHasher`: BLAKE3 → ISCC multihash (64-byte digest truncated)
- `SumHasher` (issue #37): inner `DataHasher` + `InstanceHasher`; `finalize(bits, wide, add_units)`
    composes via `gen_iscc_code_v0`. Full path `iscc_lib::streaming::SumHasher` (NOT crate-root).
    All 7 bindings implement `gen_sum_code_v0`. gen_sum details → MEMORY-archive.md

## API Design

- Video API uses `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), other
    bindings pass `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields + validates with
    `TryFrom<u8>`. Delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix and dashes, returns exact digest bytes (not full tail) as
    `(u8,u8,u8,u8,Vec<u8>)`; `MainType` is `pub(crate)`
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key. 5 cross-binding constants → MEMORY-archive.md

## Documentation Files

- Tabbed syntax: `=== "Language"` 4-space indent, blank line before code block. Landing-page tab
    order: Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin (11); zensical.toml
    nav howto order differs (nav starts Rust, Python; landing starts Python, Rust)
- Howto guides `docs/howto/{lang}.md`; API refs
    `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`; per-package READMEs + CLAUDE.md under
    `packages/{dotnet,cpp,swift,kotlin}/`; `docs/unicode.md` (iter 143) = user-facing Unicode 16.0.0
    freeze-rule page (nav Explanation + `ORDERED_PAGES` + llms.txt; keep in sync with
    `unicode_boundary.json` if vectors ever change)
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md` (excludes
    `docs/includes/`). Run after `zensical build` in docs CI

## Feature Flags

- `crates/iscc-lib/Cargo.toml`: `default = ["meta-code"]`; `text-processing` gates `text_clean`,
    `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`; `meta-code` (implies text-processing
    \+ JCS) gates META_TRIM constants, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests`, `sliding_window_bytes`
- `conformance` module always available; `conformance_selftest()` skips disabled code types via
    `#[cfg]`. `serde_json` stays non-optional (conformance.rs parses data.json)
- When gating `pub(crate)` functions, gate their tests too — dead-code lint fires in library builds
    even if test modules use them; `tests/test_text_utils.rs` also needs per-function gating

## Ruby Bindings (Magnus) — full details → MEMORY-archive.md

- Magnus 0.7.1 (not 0.8, Ruby 3.1 compat); `function!` has no `&Ruby` param — use `Ruby::get()`.
    `ExtensionTask.new("iscc-rb")` = Cargo package name; `extconf.rb` at crate root. Streaming:
    `RefCell<Option<inner>>` one-shot finalize; `_` prefix for methods NOT class names. Standard
    Ruby + rubocop-minitest; pre-commit hook needs portable PATH for `bundle`

## Other Bindings — pointers

- .NET: `packages/dotnet/` — P/Invoke over `iscc_ffi`, 32/32 Tier 1 symbols, `dotnet-version: 8.0`.
    C++: `packages/cpp/` — header-only C++17, depends on `iscc-ffi`, CMake + ASAN tests. Full detail
    → MEMORY-archive.md
- UniFFI scaffolding, Swift package (two Package.swift, XCFramework script), Kotlin/JVM (Gradle +
    JNA) → uniffi-swift-kotlin.md
