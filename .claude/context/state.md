<!-- assessed-at: bf86d128b7a55014c5e30ac99d8649f0b5da969e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ bound and documented on all 11 surfaces; 10 normal issues remain

HEAD `bf86d12` is the pushed `develop` tip and passes CI on all 21 check-runs. Iteration 212 fixed
the Ruby Magnus `Vec<T>` segfault at all 7 array-argument sites and bumped `json` in the Ruby lock;
review filed one new `normal` issue (Ruby negative-Bignum truncation). ISCC-SEQ is complete on every
surface, so the only thing between the tree and v0.8.0 is the 10 open `normal` issues.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 426 `#[test]`, with 44 in `tests/test_seq.rs`. `conformance_selftest` runs
    the seq vectors
- Strict base32 with ASCII-only upper-casing is met. The `encode_seq` case-fold divergence is
    recorded in `rust-core.md` and decisions.md
- iai Perf, Coverage + CRAP and semver-checks passed on `bf86d12` (semver is informational)
- **Missing (human-gated, not v0.8.0 work):** the crate is not yet `>= 1.0.0`, and the MSRV
    criterion is unverified. Nothing in the Rust core is owed for v0.8.0

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols, including `encode_seq` (returns `bytes`) and `decode_seq` (returns
    `list[str]`), with `.pyi` stubs and `__all__`. abi3-py310 wheels for 4 platforms; 12 `py.detach`
    sites; 0.7.0 on PyPI. `docs/howto/python.md` has a seq section
- **Missing (`normal` issue):** the `encode_seq` docstrings don't mention the case-fold divergence

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations (`encode_seq` → `Buffer`, `decode_seq` →
    `Array<string>`). `conformance.test.mjs` runs the seq vectors; `__tests__/seq.test.mjs` adds
    round-trip and negative tests. `@iscc/lib` 0.7.0 is published via OIDC
- **Missing (`normal` issue):** every napi `Buffer` parameter keeps a reference to a rejected
    non-Buffer object (an upstream napi-rs 3.11 bug, crate-wide)

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols: `encode_seq` (`units: JsValue`, any JS iterable) and `decode_seq` (`&[u8]`).
    `tests/conformance.rs` runs the seq vectors and `tests/unit.rs` adds seq tests. The SIMD release
    build (`+simd128`, `wasm-opt --enable-simd`) is met
- **Missing (`normal` issue):** `crates/iscc-wasm/Cargo.toml` has no `license` and no `description`

## C FFI

**Status**: partially met

- 51 `#[unsafe(no_mangle)]` exports, including `iscc_encode_seq` (returns `IsccByteBuffer`) and
    `iscc_decode_seq` (returns `char**`). Committed `iscc.h` declares both; `tests/test_iscc.c` adds
    a round trip and a zero-count case. `docs/c-ffi-api.md` has seq entries
- Existing: header freshness check, `examples/iscc_sum.c` + CMake, `docs/howto/c-cpp.md`, 0.7.0
    tarballs on GitHub Releases
- **Missing (`normal` issues):** `docs/c-ffi-api.md` uses unprefixed type names while
    `docs/howto/c-cpp.md` uses the prefixed `iscc_IsccByteBuffer`, so the two pages disagree;
    `src/lib.rs` is 2870 lines, past the crate's ~2500 split threshold

## Java Bindings

**Status**: partially met

- `IsccLib.java` declares `byte[] encodeSeq(String[])` and `String[] decodeSeq(byte[])`; the JNI
    crate exports both. `IsccLibTest.java` runs both data.json seq sections. `docs/java-api.md` and
    `docs/howto/java.md` document both. `io.iscc:iscc-lib` 0.7.0 is on Maven Central
- **Missing (`normal` issue):** JNI exports turn a pending JVM exception (e.g. OOM) into
    `IllegalArgumentException` (crate-wide `throw_and_default` shape)
- No CI job runs the JNI suite under `-Xcheck:jni`. `encodeSeqRejectsPadding` uses a literal the
    padding-tolerant reference also rejects, so it cannot catch a padding regression

## Go Bindings

**Status**: met for v0.8.0 (v1.0.0 items human-gated)

- Pure Go, 35 Tier 1 symbols: `EncodeSeq` and `DecodeSeq` port the Rust code, with a private
    `readUnit`. `CGO_ENABLED=0`; IDv1 at the reference API shape; strict base32; `isccClean` at all
    5 string-input sites
- `TextClean` keeps `strings.TrimSpace` by design. 264 `func Test`; `ConformanceSelftest` runs the
    seq vectors. `packages/go/CLAUDE.md` and `docs/howto/go.md` are current

## C# / .NET Bindings

**Status**: partially met

- `IsccLib.EncodeSeq(string[])` → `byte[]` and `IsccLib.DecodeSeq(ReadOnlySpan<byte>)` → `string[]`
    wrap the generated P/Invoke declarations. `ConformanceTests.cs` runs both seq sections;
    `SeqTests.cs` adds 10 tests. `Iscc.Lib` 0.7.0 is on NuGet
- **Missing (`normal` issues):** an embedded NUL silently truncates every string input; the
    string-array pin loop is copied three times and the gen copies pass NULL for `[]`
- The NUL fix approach is now decided (decisions.md 2026-11-19): reject an embedded NUL at the
    wrapper boundary in .NET and C++; the C ABI stays NUL-terminated

## C++ Bindings

**Status**: partially met

- `iscc.hpp` has `iscc::encode_seq` → `std::vector<uint8_t>` and `iscc::decode_seq` →
    `std::vector<std::string>`, RAII-owned, with an embedded-NUL guard on `encode_seq` only. 0.7.0
    ships via vcpkg/Conan metadata. `tests/test_iscc.cpp` hard-codes the seq vectors it runs
- **Missing (`normal` issue, shared with .NET):** the other 15 `c_str()` call sites still truncate
    at an embedded NUL; the reject-at-boundary approach is decided

## Ruby Bindings

**Status**: partially met

- `encode_seq` / `decode_seq` registered; `test_conformance.rb` runs both seq sections. 0.7.0 is on
    RubyGems. All 7 array-argument sites go through `array_to_vec` (`RArray::into_iter` over a
    hidden copy, `TryConvertOwned` bound); no `Vec<_>` parameter remains
- `test_array_args.rb` (16 tests) pins that a mutating `to_str`/`to_int` no longer segfaults.
    `Gemfile.lock` has `json (2.21.2)`; Dependabot alert #1 stays open until develop merges into
    `main` (the default branch). `docs/ruby-api.md` Error Handling now names
    `TypeError`/`RangeError`
- **Missing (`normal` issue):** negative Bignum elements in the `i32` lists (`gen_audio_code_v0`,
    video frames) truncate instead of raising `RangeError` (Magnus `RBignum::to_i32` skips the lower
    bound), contradicting the docs' `RangeError` promise

## Swift and Kotlin Bindings

**Status**: met for v0.8.0

- `crates/iscc-uniffi/src/lib.rs` exports `encode_seq(Vec<String>)` → `Vec<u8>` and
    `decode_seq(Vec<u8>)` → `Vec<String>`; 28 `#[test]` in the crate; module doc says 35. Generated
    Swift `encodeSeq(units:)` / `decodeSeq(data:)` and Kotlin `encodeSeq` / `decodeSeq`
- Swift `ConformanceTests.swift` and Kotlin `ConformanceTest.kt` each have 19 tests, including all 8
    data.json seq vectors. Both shipped 0.7.0; how-to pages, `CLAUDE.md` files and READMEs are
    current

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, and a
    Development section; the site deploys from `main`. The root README and 12 per-crate/package
    READMEs are present
- No stale Tier 1 count of 33 remains outside `.claude/`. All 11 how-to pages and the Rust, Java,
    Ruby and C FFI API pages cover `encode_seq` / `decode_seq`
- Open doc questions are tracked by the C-page type-name issue and the Ruby Bignum issue (above)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind (`crates/iscc-lib/benches/`); pytest-benchmark comparing
    iscc-core and iscc-lib in `benchmarks/python/`; speedups published in `docs/benchmarks.md`
- Most iai benches run a few percent below the committed baseline, which loosens the effective
    tolerance. A baseline refresh needs authorisation

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; Swift is the only job on macOS (`macos-15`). `release.yml` is
    `workflow_dispatch`-only with 8 registry toggles. 21 version-sync targets are at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: PASSING** on `bf86d12` (= HEAD, pushed):
    https://github.com/iscc/iscc-lib/actions/runs/44492092328. All 21 check-runs succeeded
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve` has two high alerts, `setuptools`),
    tracked as a `normal` issue

## Open Issues

17 open: 0 critical, **10 normal**, 7 low. The 10 `normal` issues: c-ffi-api type names, wasm npm
metadata, the Python `encode_seq` case-fold docstrings, the `uv.lock` alerts, the napi `Buffer`
reference leak, the `iscc-ffi/src/lib.rs` split, .NET + C++ embedded-NUL truncation (approach now
decided), .NET pin-loop duplication, the JNI pending-exception mapping, and the Ruby negative-Bignum
truncation. The Ruby segfault and `json` issues are closed.

## Next Milestone

Clear the 10 open `normal` issues that block the v0.8.0 release (or get them marked as blocked on
the human or upstream), keeping CI green on `develop`.
