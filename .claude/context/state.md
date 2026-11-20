<!-- assessed-at: 1ffa2d91caa93fd7fe98583db0a9481810233ef7 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 6 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 228 tightened `scripts/check_spec_paths.py` and closed that issue; CI is
fully green on the pushed tip `5498b6c`. Six `normal` issues still block v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met (seed set met; 1 `normal` issue owed)

- `crates/iscc-lib/tests/data-edge.json` has 6 sections: `iscc_decode`, `encode_seq`,
    `gen_text_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `alg_simhash`; harnesses on all 12
    surfaces; the 5 tracked copies are byte-identical and match `VENDORED_COPIES`
- **`normal`:** Swift compares strings by canonical equivalence and reads a JSON bool as an Int; the
    Python template conflates `0` and `False`

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- `alg_simhash` returns `InvalidInput` for `[]`; `rust-core.md` Tier 1 table matches the code
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met

## C FFI

**Status**: met (v0.8.0 scope); 1 `normal` issue

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row (incl. `alg_simhash`) except the 2 marked `c: skip`
- **`normal`:** `src/lib.rs` is past the crate's own ~2500-line split threshold

## Java Bindings

**Status**: met (v0.8.0 scope); 2 `normal` issues

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections, green in CI; 0.7.0 on Maven Central
- **`normal`:** the four streaming-hasher functions dereference a zero handle without a guard;
    `throw_and_default` throws `IllegalArgumentException` over a pending JVM exception

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `AlgSimhash([])` errors; `data_edge_test.go` runs the `alg_simhash` section, green in CI

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; embedded NUL rejected; `PinnedRows` pins all jagged-array inputs;
    `DataEdgeTests.cs` runs all 6 sections, green in CI; 0.7.0 on NuGet
- `docs/howto/dotnet.md` "Error handling" doesn't list `ArgumentNullException` (no issue filed)

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected; all data-edge rows run
    from the generated header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan
- Every vector input goes through `detail::safe_data`, held by `tests/test_cpp_safe_data.py`

## Ruby Bindings

**Status**: met (v0.8.0 scope); 1 `normal` issue

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections,
    green in CI; 0.7.0 on RubyGems
- **`normal`:** negative Bignums in `i32` list elements truncate instead of raising `RangeError`

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift `DataEdgeTests` and Kotlin `DataEdgeTest` run all 6 sections, green in CI; both
    shipped 0.7.0
- The Swift data-edge compare is loose (counted under data-edge above)

## Documentation

**Status**: met (v0.8.0 scope); 1 `normal` issue

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; all how-tos cover the seq functions
- `scripts/check_spec_paths.py` now matches globs with its own `glob_regex` (`*` stays in one
    segment), applies `NOT_PATHS` to backticked spans only, and replays on a frozen path set (18
    tests); the leniency issue is closed
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
- **Latest CI: PASSING** on `origin/develop` = `5498b6c`
    (https://github.com/iscc/iscc-lib/actions/runs/44736457762): all 21 check-runs `success`
- Only the iteration log commit `1ffa2d9` is unpushed; green CI covers all code at HEAD
- GitHub Dependabot alerts track `main` and stay open until the next release (triaged, no issue)

## Open Issues

11 open: 0 critical, **6 normal**, 5 low. Normal: data-edge compare looseness, derived-fact drift,
iscc-ffi `lib.rs` size, JNI null guard, JNI pending exception, Ruby Bignum truncation. issues.md is
at 183 of 300 lines.

## Next Milestone

Clear the 6 open `normal` issues, which are the only remaining v0.8.0 release blockers.
