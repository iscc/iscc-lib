<!-- assessed-at: b00d83d654d3062a72f431397e63ee4562f6575d -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ on Rust core + 9 of 11 surfaces (all but Swift and Kotlin); CI green

The pushed `develop` tip `a15164b` passes CI on all 21 check-runs. HEAD `b00d83d` adds only a
`cid(log)` commit, so that run covers HEAD's code. Iteration 209 bound `encode_seq` / `decode_seq`
in the Ruby gem, bringing ISCC-SEQ to 9 of 11 surfaces; only the shared UniFFI crate (Swift +
Kotlin) remains. 11 `normal` issues are open, including a new Ruby one: Magnus `Vec<T>` parameters
can segfault.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 426 `#[test]`, with 44 in `tests/test_seq.rs`. `conformance_selftest` runs
    the seq vectors
- `text_trim` / `text_remove_newlines` use the Python `str.strip()` set. Strict base32 with
    ASCII-only upper-casing is met; the `encode_seq` case-fold divergence is recorded in
    `rust-core.md` and decisions.md
- iai Perf, Coverage + CRAP and semver-checks passed on `a15164b` (semver is informational)
- **Missing (human-gated, not v0.8.0 work):** the crate is not yet `>= 1.0.0`, and the MSRV
    criterion is unverified. Nothing in the Rust core is owed for v0.8.0

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols, including `encode_seq` (returns `bytes`) and `decode_seq` (returns
    `list[str]`), with `.pyi` stubs and `__all__`. abi3-py310 wheels for 4 platforms; 12 `py.detach`
    sites; 0.7.0 on PyPI
- **Missing (`normal` issue):** the `encode_seq` docstrings don't mention the case-fold divergence
- Stale doc: `crates/iscc-py/CLAUDE.md` still says "27 #[pyfunction]s"

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations (`encode_seq` → `Buffer`, `decode_seq` →
    `Array<string>`). `conformance.test.mjs` runs the seq vectors; `__tests__/seq.test.mjs` adds
    round-trip and negative tests. `@iscc/lib` 0.7.0 is published via OIDC
- **Missing (`normal` issue):** every napi `Buffer` parameter keeps a reference to a rejected
    non-Buffer object (an upstream napi-rs 3.11 bug, crate-wide)
- Stale doc: `crates/iscc-napi/CLAUDE.md` still says 33 symbols

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols: `encode_seq` (`units: JsValue`, any JS iterable) and `decode_seq` (`&[u8]`).
    `tests/conformance.rs` runs the seq vectors and `tests/unit.rs` adds seq tests. The SIMD release
    build (`+simd128`, `wasm-opt --enable-simd`) is met
- **Missing (`normal` issue):** `crates/iscc-wasm/Cargo.toml` has no `license` and no `description`
- Stale doc: `crates/iscc-wasm/CLAUDE.md` still says 33 symbols

## C FFI

**Status**: partially met

- 51 `#[unsafe(no_mangle)]` exports, including `iscc_encode_seq` (returns `IsccByteBuffer`) and
    `iscc_decode_seq` (returns `char**`). Committed `iscc.h` declares both; Rust tests walk the
    data.json seq vectors; `tests/test_iscc.c` adds a round trip and a zero-count case
- Existing: header freshness check, `examples/iscc_sum.c` + CMake, `docs/howto/c-cpp.md`, 0.7.0
    tarballs on GitHub Releases
- **Missing (`normal` issues):** `docs/c-ffi-api.md` uses unprefixed type names and has no seq
    entries; `src/lib.rs` is 2870 lines, past the crate's ~2500 split threshold

## Java Bindings

**Status**: partially met

- `IsccLib.java` declares `byte[] encodeSeq(String[])` and `String[] decodeSeq(byte[])`; the JNI
    crate exports both. `IsccLibTest.java` runs both data.json seq sections, a round trip and
    negatives. `io.iscc:iscc-lib` 0.7.0 is on Maven Central
- **Missing (`normal` issue):** JNI exports turn a pending JVM exception (e.g. OOM) into
    `IllegalArgumentException` (crate-wide `throw_and_default` shape)
- No CI job runs the JNI suite under `-Xcheck:jni`. `encodeSeqRejectsPadding` uses a literal the
    padding-tolerant reference also rejects, so it cannot catch a padding regression
- Stale docs: `crates/iscc-jni/README.md`, `crates/iscc-jni/CLAUDE.md`, `docs/howto/java.md`,
    `docs/java-api.md` have no seq entries

## Go Bindings

**Status**: partially met

- Pure Go, 35 Tier 1 symbols: `EncodeSeq` and `DecodeSeq` port the Rust code, with a private
    `readUnit`. `CGO_ENABLED=0`; IDv1 at the reference API shape; strict base32; `isccClean` at all
    5 string-input sites
- `TextClean` keeps `strings.TrimSpace` by design. 264 `func Test`; `ConformanceSelftest` runs the
    seq vectors
- Stale doc: `packages/go/CLAUDE.md` still says "all 9 gen functions" and "all 46 conformance"

## C# / .NET Bindings

**Status**: partially met

- `IsccLib.EncodeSeq(string[])` → `byte[]` and `IsccLib.DecodeSeq(ReadOnlySpan<byte>)` → `string[]`
    wrap the generated P/Invoke declarations. `ConformanceTests.cs` runs both seq sections;
    `SeqTests.cs` adds 10 tests. `Iscc.Lib` 0.7.0 is on NuGet
- **Missing (`normal` issues):** an embedded NUL silently truncates every string input; the
    string-array pin loop is copied three times and the gen copies pass NULL for `[]`
- Stale doc: `packages/dotnet/CLAUDE.md` still says "all 9 gen functions (50 vectors total)"

## C++ Bindings

**Status**: partially met

- `iscc.hpp` has `iscc::encode_seq` → `std::vector<uint8_t>` and `iscc::decode_seq` →
    `std::vector<std::string>`, RAII-owned, with an embedded-NUL guard on `encode_seq` only. 0.7.0
    ships via vcpkg/Conan metadata. `tests/test_iscc.cpp` hard-codes the seq vectors it runs
- **Missing (`normal` issue):** the other 15 `c_str()` call sites still truncate at an embedded NUL
- Stale docs: `packages/cpp/CLAUDE.md`, `packages/cpp/README.md`, `docs/howto/c-cpp.md`

## Ruby Bindings

**Status**: partially met

- `crates/iscc-rb/src/lib.rs` registers `encode_seq` (Array → binary String) and `decode_seq`
    (String → Array of `ISCC:` units) at L573-574, giving 35 Tier 1 symbols. `test_conformance.rb`
    reads `data.json` as explicit UTF-8 and runs both seq sections; `test_seq.rb` adds 13 tests
    (padding negative + control, `"\0junk"`). 0.7.0 is on RubyGems
- **Missing (`normal` issues):** Magnus 0.8.2 `Vec<T>` parameters segfault when an element's
    `to_str` mutates the array (every `Vec<_>` param); the `json` alert in `Gemfile.lock`
- Stale docs: `crates/iscc-rb/README.md`, `docs/howto/ruby.md`, `crates/iscc-rb/CLAUDE.md` have no
    seq entries; the CLAUDE.md Key Files table lacks `test_iscc_id.rb` and `test_seq.rb`

## Swift and Kotlin Bindings

**Status**: partially met

- Both expose the 33-symbol surface through `crates/iscc-uniffi` and shipped 0.7.0: Kotlin on Maven
    Central, Swift as an SPM XCFramework
- **Missing on both:** `encode_seq` / `decode_seq` (no hit in `crates/iscc-uniffi` or the package
    sources; only the vendored `data.json` matches) and the seq vectors in each conformance harness.
    Swift and Kotlin hard-code per-section counts
- UniFFI is the last surface whose embedded-NUL handling has not been probed

## Documentation

**Status**: met (for 0.7.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, and a
    Development section; the site deploys from `main`. The root README and 12 per-crate/package
    READMEs are present
- Pending for v0.8.0: seq API entries, and updating the 33 → 35 count in the crate `CLAUDE.md`
    files, `notes/00`, `notes/04`, `docs/architecture.md` and the binding docs

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
- **Latest CI: PASSING** on `a15164b`: https://github.com/iscc/iscc-lib/actions/runs/44452418939.
    All 21 check-runs succeeded. No unpushed code changes at HEAD
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve` has two high alerts, `setuptools`),
    tracked as a `normal` issue

## Open Issues

18 open: 0 critical, **11 normal**, 7 low. The 11 `normal` issues: c-ffi-api type names, wasm npm
metadata, the Ruby `json` alert, the Python `encode_seq` case-fold docstrings, the `uv.lock` alerts,
the napi `Buffer` reference leak, the `iscc-ffi/src/lib.rs` split, .NET + C++ embedded-NUL
truncation (fix approach awaits a human decision), .NET pin-loop duplication, the JNI
pending-exception mapping, and the Ruby Magnus `Vec<T>` segfault. The Ruby locale issue is resolved.

## Next Milestone

Finish v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the last two surfaces (Swift and Kotlin via
UniFFI), with every conformance harness running the seq vectors. Then the 33 → 35 documentation
sweep, and clear the 11 open `normal` issues that block the release.
