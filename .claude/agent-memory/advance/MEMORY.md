# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations. Detail lives in topic files: [ci-gates.md](ci-gates.md) (coverage/CRAP, cargo-deny
audit, semver, iai perf gates), [uniffi-swift-kotlin.md](uniffi-swift-kotlin.md) (UniFFI, Swift,
Kotlin). Archived phases: [MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`. Node.js:
    `crates/iscc-napi/src/lib.rs`. WASM: `crates/iscc-wasm/src/lib.rs`. C FFI:
    `crates/iscc-ffi/src/lib.rs`. JNI: `crates/iscc-jni/src/lib.rs` +
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (wrapper + Result classes).
    Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go). WASM bridge removed — pure Go only

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn test -f crates/iscc-jni/java/pom.xml` (native lib prereq)
- CI workflow `.github/workflows/ci.yml` has 19 job entries (version-check, rust, python-test,
    python, nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver,
    coverage, audit). `bench` = `cargo bench --no-run`. `swift` on `macos-14`; `kotlin` on `ubuntu`
    JDK 17 + `cargo build -p iscc-uniffi` + `./gradlew test`
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json` +
    `--fail-regression   --fail-above`), `audit` (cargo-deny, `deny.toml`), `perf` (iai regression
    vs `.iai-baseline.json`). `semver` informational until v1.0.0. Full mechanics + gotchas →
    ci-gates.md
- Key audit rule: fresh advisory with a patched release → `cargo update -p <crate>` lockfile bump,
    NEVER add to `deny.toml` `ignore` (iter 115: crossbeam-epoch 0.9.18→0.9.20)
- GOTCHA: never pipe `cargo crap`/`cargo deny` into `tail`/`head` to check exit — `$?` = pager,
    masks exit 1; redirect to a file first
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
    unsafe for raw `PyList_GetItem` ptrs. Recipe → MEMORY-archive.md
- GIL release complete (iters 111+116, #39/#41): 12 `py.detach` sites in `crates/iscc-py/src/lib.rs`
    (data/instance/image/sum, 3 hasher `update()`s, text, video+flat, soft_hash_video+flat). Video
    detach MUST open after `extract_frame_sigs`/`flat_bytes_to_frames`; closures capture only
    owned-Rust-Vec borrows. meta/audio/mixed stay attached by design. Tests: `tests/test_gil.py`
- Release workflow (`release.yml`): 9 boolean inputs, pattern input → build → **smoke test** →
    publish. Full input list + per-registry auth + release-job CI internals → MEMORY-archive.md
- wasm-pack `--features` goes AFTER the path, NOT after `--`; test runner accepts only a positional
    FILTER (`-- --test unit` fails), so run the full suite
- WASM SIMD (iters 117-118, #42): BLAKE3's wasm32 backend needs BOTH the `blake3/wasm32_simd` Cargo
    feature (direct dep in iscc-wasm Cargo.toml, feature-unification only, no `use blake3` —
    RUSTFLAGS alone leaves `Platform::Portable`) AND `RUSTFLAGS: -C target-feature=+simd128` (CI
    `wasm` + release `build-wasm` steps) + `--enable-simd` in the wasm-opt array. `v128` opcode
    counting is a false-positive signal (LLVM auto-vectorizes the portable path); the honest wiring
    proof is `cargo tree --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` showing
    `wasm32_simd`. wasm-pack `pkg/` is self-gitignored (`pkg/.gitignore` = `*`)

## Benchmarks

- Two benches in `crates/iscc-lib/benches/`, both `harness = false`: `benchmarks.rs` (criterion,
    wall-clock + throughput, incl `gen_sum_code_v0` via tempfile) and `iai_benches.rs`
    (iai-callgrind 0.16.1, instruction-counts). Share `deterministic_bytes`/`synthetic_text`
    builders. iai compiles without valgrind; running needs valgrind + runner (devcontainer has both;
    `mise run bench:iai`). Perf gate mechanics + bench-profile gotchas → ci-gates.md

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
    `packages/{dotnet,cpp,swift,kotlin}/`. zensical.toml nav howto order: Rust, Python, Ruby,
    Node.js, WASM, Go, Java, C#/.NET, C/C++, Swift, Kotlin (differs from landing-page tab order
    above, which starts Python, Rust)
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
- When gating `pub(crate)` functions, their tests must also be gated — dead-code lint fires in
    library builds even if test modules use them. Integration tests in
    `crates/iscc-lib/tests/test_text_utils.rs` also need per-function gating
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
