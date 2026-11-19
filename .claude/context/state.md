<!-- assessed-at: 8908dd3eefb9dd63259ef67b6f36b5bd9b24a2d0 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ done everywhere; shared edge-case vectors not started

ISCC-SEQ is bound and documented on all 11 surfaces. On 2026-11-19 the human added a second v0.8.0
release criterion, the shared edge-case vector file `data-edge.json`. That work has not started: the
file does not exist and no harness runs it. 6 `normal` issues are open. CI is green on the pushed
tip `6c619db`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: not started

- `crates/iscc-lib/tests/data-edge.json` does not exist, and no vendored copy is tracked
    (`git ls-files` finds no match). No harness on any surface loads it
- `tests/test_seq.py` still keeps the hand-written `STRICT_BASE32_DIVERGENCES` list, which the spec
    says the file should replace
- Owed by the spec (`specs/edge-vectors.md`): the seed set (iscc_decode padding and spare bits,
    encode_seq case-fold alias / `[]` / NUL-junk, gen_text NUL, mixed and iscc-code `[]`), a schema
    test, `test_vendored_fixtures.py` coverage, runs on Rust and all 11 bindings, and the Python
    check against iscc-core 1.5.0

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES` live in `codec.rs`.
    426 `#[test]`, 44 of them in `tests/test_seq.rs`. `conformance_selftest` runs the seq vectors
- iai Perf, Coverage + CRAP and semver-checks pass (semver is informational)
- **Missing for v0.8.0:** the Rust `data-edge.json` harness (see above)
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI
- **Missing (`normal` issue):** the `encode_seq` docstrings don't name the non-ASCII case-fold
    divergence. **Missing for v0.8.0:** the data-edge harness and the iscc-core cross-check

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations; the conformance and seq tests run the seq vectors.
    `package.json` now has a plain-text `description`; `@iscc/lib` 0.7.0 is on npm (OIDC)
- Missing: the data-edge harness. The napi `Buffer` reference leak is now a `low` issue

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; the conformance and unit tests cover seq; SIMD release build is met.
    `Cargo.toml` now inherits `license` (Apache-2.0) and has a `description`, so wasm-pack writes
    both into `pkg/package.json`. The published 0.7.0 package still lacks them
- **Missing:** the data-edge harness. **`normal` issue:** `specs/wasm-bindings.md` still lists a
    `crates/iscc-wasm/package.json` (L28, L165) that does not exist

## C FFI

**Status**: partially met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** the C data-edge harness (generated table allowed). **`normal` issue:**
    `docs/c-ffi-api.md` type names differ from `iscc.h`. The `src/lib.rs` split (2870 lines) is now
    `low`

## Java Bindings

**Status**: partially met

- `encodeSeq` / `decodeSeq` in `IsccLib.java`; `IsccLibTest.java` runs both seq sections; 0.7.0 on
    Maven Central
- Missing: the data-edge harness. The JNI pending-exception mapping is now a `low` issue

## Go Bindings

**Status**: partially met

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32, 264
    `func Test`
- Missing: the data-edge harness

## C# / .NET Bindings

**Status**: partially met

- `EncodeSeq` / `DecodeSeq` wrap the generated P/Invoke code; embedded NUL rejected via
    `ToNativeUtf8` (9 `NulInputTests`); 0.7.0 on NuGet
- **Missing:** the data-edge harness. **`normal` issue:** the string-array pin loop is copied three
    times and the gen copies pass NULL for `[]`

## C++ Bindings

**Status**: partially met

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected via
    `detail::checked_c_str`; 0.7.0 vcpkg/Conan metadata. `tests/test_iscc.cpp` hard-codes the seq
    vectors
- Missing: the data-edge harness (generated table allowed)

## Ruby Bindings

**Status**: partially met

- `encode_seq` / `decode_seq` registered and run by `test_conformance.rb`; array args go through
    `array_to_vec` (16 tests in `test_array_args.rb`); 0.7.0 on RubyGems
- Missing: the data-edge harness. The negative-Bignum `i32` truncation is now a `low` issue

## Swift and Kotlin Bindings

**Status**: partially met

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq` (28 `#[test]`). Swift and Kotlin
    conformance suites have 19 tests each, including all 8 seq vectors; both shipped 0.7.0
- Missing: the data-edge harness on both

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs. All how-tos cover `encode_seq` /
    `decode_seq`
- **`normal` issue `[human]`:** hand-written derived facts (Tier 1 lists, counts) drift; needs a
    Tier 1 parity check and the removal of unchecked counts from specs and per-crate `CLAUDE.md`

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: PASSING** on the pushed tip `6c619db`: 21/21 check-runs succeeded, GitHub Actions
    check-suite `success`
- **Not covered by CI:** 4 unpushed commits at HEAD change `tools/cid.py`, `tests/test_cid.py` and
    `CLAUDE.md` (loop tooling only, no library code)
- **Missing (`normal` issue):** Dependabot alerts on `soupsieve`, `setuptools` and the `json` gem

## Open Issues

18 open: 0 critical, **6 normal**, 12 low. Normal: c-ffi-api type names, Python `encode_seq`
docstrings, Dependabot alerts, .NET pin-loop duplication, the wasm-bindings spec drift, and
derived-fact drift. The napi Buffer leak, the JNI exception mapping, the Ruby Bignum truncation and
the iscc-ffi split were lowered to `low` by the human.

## Next Milestone

Meet the new v0.8.0 criterion: create `data-edge.json` with the seed set and have the Rust core and
all 11 bindings run it. Also clear the 6 open `normal` issues, keeping CI green on `develop`.
