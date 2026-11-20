<!-- assessed-at: 32d668641faea5a60036452b5d43ded302629ffa -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 8 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. CI is fully green on the pushed tip `7225fce`. Titusz's triage on 2026-11-20
promoted five defects to `normal` and decided that `alg_simhash` rejects an empty list (not yet
implemented). With the spec path gate's own leniency issue and the derived-facts issue, 8 `normal`
issues now block v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met (seed set met; 2 `normal` issues owed)

- `crates/iscc-lib/tests/data-edge.json` holds the seed set (16 vectors, 5 sections, 2 `except`
    entries); harnesses on all 12 surfaces; 5 tracked copies match `VENDORED_COPIES`
- `edge-vectors.md` now lets Go and Swift carry the file as test data, so the old packaging issue is
    resolved
- **`normal`:** Swift compares strings by canonical equivalence and reads a JSON bool as an Int; the
    Python template conflates `0` and `False`
- **`normal`:** the `alg_simhash` `[]` → `error` vector is not in the file yet (see Rust core)

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- **Missing (`normal`, decided 2026-11-20):** `alg_simhash` in `simhash.rs` still returns 32 zero
    bytes for an empty list instead of an error; the Go port has the same behaviour, and its
    generators rely on it
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- The `encode_seq` docstrings name the inputs iscc-lib rejects, gated by
    `tests/test_encode_seq_docs.py`

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC); spec structure tree now matches disk
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met

## C FFI

**Status**: met (v0.8.0 scope); 1 `normal` issue

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`; `c-ffi` job green
- **`normal`:** `src/lib.rs` is 2870 lines, past the crate's own ~2500-line split threshold

## Java Bindings

**Status**: met (v0.8.0 scope); 2 `normal` issues

- `encodeSeq` / `decodeSeq`; `IsccLibTest` runs both seq sections; `DataEdgeTest` green in CI; 0.7.0
    on Maven Central
- **`normal`:** the four streaming-hasher functions dereference a zero handle without a guard;
    `throw_and_default` throws `IllegalArgumentException` over a pending JVM exception

## Go Bindings

**Status**: met (v0.8.0 scope); `alg_simhash` change owed

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `data_edge_test.go` green in CI; spec structure tree now matches disk
- **`normal`:** `AlgSimhash([])` returns 32 zero bytes; the audio/mixed/meta generators call the
    public function and must move to an unchecked helper when it starts to reject `[]`

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; embedded NUL rejected; `PinnedRows` pins all jagged-array inputs;
    `DataEdgeTests.cs` green in CI; 0.7.0 on NuGet
- `docs/howto/dotnet.md` "Error handling" doesn't list `ArgumentNullException` (no issue filed)

## C++ Bindings

**Status**: met (v0.8.0 scope); 1 `normal` issue

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected; all 16 data-edge rows
    run from the generated header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan metadata
- **`normal`:** five `iscc.hpp` list wrappers pass NULL for an empty vector (part of the
    `alg_simhash` issue)

## Ruby Bindings

**Status**: met (v0.8.0 scope); 1 `normal` issue

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` green in CI; 0.7.0 on
    RubyGems
- **`normal`:** negative Bignums in `i32` list elements truncate instead of raising `RangeError`

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift `DataEdgeTests` and Kotlin `DataEdgeTest` green in CI; both shipped 0.7.0
- The Swift data-edge compare is loose (counted under data-edge above)

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; all how-tos cover the seq functions
- `scripts/check_spec_paths.py` (prek hook + 15 tests in `tests/test_check_spec_paths.py`) checks
    every spec tree entry and backticked path; the structure-tree issue is resolved
- **`normal`:** the gate is looser than its docstring (`*` crosses `/`, `NOT_PATHS` exempts tree
    entries, replay tests read the live index)
- **`normal` `[human]`:** no Tier 1 list parity check; hand-written counts and file lists remain in
    specs and per-crate `CLAUDE.md` files

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: PASSING** on `origin/develop` = `7225fce`
    (https://github.com/iscc/iscc-lib/actions/runs/44697229566): all 21 check-runs `success`
- One unpushed commit (`32d6686`, loop infra: `tools/cid.py`, `tests/test_cid.py`) is not yet
    covered by CI; no library code changed
- `mdformat` left the dev dependencies (the prek hook brings its own), which removed `soupsieve`
    from `uv.lock`; the Dependabot issue was dropped in triage. GitHub still lists open alerts
    (`soupsieve`, `setuptools` in `uv.lock`, `json` in the gem lockfile), presumably for `main`

## Open Issues

13 open: 0 critical, **8 normal**, 5 low. Normal: `alg_simhash` empty list + C++ NULL wrappers,
data-edge compare looseness, spec path gate leniency, derived-fact drift, iscc-ffi `lib.rs` size,
JNI null guard, JNI pending exception, Ruby Bignum truncation. issues.md is at 231 of 300 lines.

## Next Milestone

Clear the 8 open `normal` issues, which are now the only v0.8.0 release blockers. The `alg_simhash`
core-plus-Go change is a behaviour change that the issue says must land first and alone.
