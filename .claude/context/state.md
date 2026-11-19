<!-- assessed-at: cc91ef5cd83c526939ea6469516eca8b66746aa6 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — data-edge.json seed set landed on Rust + Python; repair and binding fan-out owed

ISCC-SEQ is bound and documented on all 11 surfaces. The shared edge-case file `data-edge.json` now
exists with 15 seed vectors and Rust and Python harnesses, but the last review returned NEEDS_WORK:
the Rust harness fails the enforcing CI CRAP gate. That commit is at HEAD, unpushed, so CI is green
only on the older pushed tip `6c619db`. 6 `normal` issues are open.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met

- `crates/iscc-lib/tests/data-edge.json` is tracked with 15 vectors in 5 sections (iscc_decode 4,
    encode_seq 8, gen_text_code_v0 1, gen_mixed_code_v0 1, gen_iscc_code_v0 1). It is excluded from
    the crate package; `test_vendored_fixtures.py` knows it, with an empty `VENDORED_COPIES` list
- Rust harness `tests/test_data_edge.rs` (7 tests, incl. 4 negative) and Python harness
    `tests/test_data_edge.py` (checks iscc-core 1.5.0) exist. `STRICT_BASE32_DIVERGENCES` is gone
    from `tests/test_seq.py`
- **Blocking (review NEEDS_WORK):** in `test_data_edge.rs`, `call` scores CRAP 210 and
    `check_vector` 156, over the threshold of 30, so the enforcing CI gate would fail on push. Two
    Python harness flaws are also owed: `pytest.raises` wraps the result conversion, and the
    reference-side rejection tuple misses `IndexError`
- **Missing:** vendored copies and harnesses on the other 10 bindings (Node, WASM, C, Java, Go,
    .NET, C++, Ruby, Swift, Kotlin)

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES` live in `codec.rs`.
    `conformance_selftest` runs the seq vectors. Data-edge harness present (see above)
- iai Perf, Coverage + CRAP and semver-checks pass on the pushed tip (semver is informational). The
    unpushed data-edge harness would fail CRAP
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI. Data-edge harness with the iscc-core cross-check present
- **Missing (`normal` issue):** the `encode_seq` docstrings don't name the non-ASCII case-fold
    divergence. The two harness repair items above

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations; conformance and seq tests run the seq vectors;
    plain-text `description` in `package.json`; `@iscc/lib` 0.7.0 on npm (OIDC)
- Missing: the data-edge harness. The napi `Buffer` reference leak is a `low` issue

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance and unit tests cover seq; SIMD release build met. `Cargo.toml`
    inherits `license` and has a `description` (the published 0.7.0 package still lacks them)
- **Missing:** the data-edge harness. **`normal` issue:** `specs/wasm-bindings.md` names a
    `crates/iscc-wasm/package.json` that does not exist

## C FFI

**Status**: partially met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** the C data-edge harness (generated table allowed). **`normal` issue:**
    `docs/c-ffi-api.md` type names differ from `iscc.h`. The `src/lib.rs` split is `low`

## Java Bindings

**Status**: partially met

- `encodeSeq` / `decodeSeq` in `IsccLib.java`; `IsccLibTest.java` runs both seq sections; 0.7.0 on
    Maven Central
- Missing: the data-edge harness. The JNI pending-exception mapping is a `low` issue

## Go Bindings

**Status**: partially met

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32, 264
    `func Test`. `seq_test.go` comment now points at the data-edge encode_seq rows
- Missing: the data-edge harness (`seqStrictDivergences` is still a hand-written copy)

## C# / .NET Bindings

**Status**: partially met

- `EncodeSeq` / `DecodeSeq` wrap the generated P/Invoke code; embedded NUL rejected via
    `ToNativeUtf8`; 0.7.0 on NuGet
- **Missing:** the data-edge harness. **`normal` issue:** the string-array pin loop is copied three
    times and the gen copies pass NULL for `[]`

## C++ Bindings

**Status**: partially met

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected via
    `detail::checked_c_str`; 0.7.0 vcpkg/Conan metadata
- Missing: the data-edge harness (generated table allowed)

## Ruby Bindings

**Status**: partially met

- `encode_seq` / `decode_seq` registered and run by `test_conformance.rb`; array args go through
    `array_to_vec`; 0.7.0 on RubyGems
- Missing: the data-edge harness. The negative-Bignum `i32` truncation is a `low` issue

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
- **`normal` issue `[human]`:** hand-written derived facts (Tier 1 lists, counts) drift

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: PASSING** on the pushed tip `6c619db` (= `origin/develop`): 21/21 check-runs
    succeeded, check-suite `success`
- **Not covered by CI:** 10 unpushed commits at HEAD. Besides loop tooling (`tools/cid.py`,
    `tests/test_cid.py`, `CLAUDE.md`) they carry the data-edge file and harnesses, which the review
    found would fail the CI CRAP gate
- **Missing (`normal` issue):** Dependabot alerts on `soupsieve`, `setuptools` and the `json` gem

## Open Issues

18 open: 0 critical, **6 normal**, 12 low. Normal: c-ffi-api type names, Python `encode_seq`
docstrings, Dependabot alerts, .NET pin-loop duplication, the wasm-bindings spec drift, and
derived-fact drift.

## Next Milestone

Make the data-edge harnesses at HEAD pass the CI CRAP gate and fix the two Python harness flaws, so
`develop` can be pushed green. Then bring `data-edge.json` to the other 10 bindings, and clear the 6
open `normal` issues.
