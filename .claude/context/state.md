<!-- assessed-at: d6686f992a3a0255d151fe8a65c8d47f18d47876 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ on Rust core + 7 of 11 surfaces (Python, Go, Node.js, WASM, C FFI, .NET, C++); CI green

The pushed `develop` tip `dee8ba3` passes CI on all 21 check-runs. HEAD `d6686f9` adds only a
`cid(log)` commit, so that run covers HEAD's code. Iteration 207 bound `encode_seq` / `decode_seq`
in the C++ header-only wrapper, bringing ISCC-SEQ to 7 of 11 surfaces. 10 `normal` issues are open;
the embedded-NUL issue now covers both .NET and C++.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 426 `#[test]`, with 44 in `tests/test_seq.rs`. `conformance_selftest` runs
    the seq vectors
- `text_trim` / `text_remove_newlines` use the Python `str.strip()` set (`codec::is_py_strip_char`,
    `pub(crate)`). Strict base32 with ASCII-only upper-casing is met; the `encode_seq` case-fold
    divergence is recorded in `rust-core.md` and decisions.md
- iai Perf, Coverage + CRAP and semver-checks passed on `dee8ba3` (semver is informational)
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

- 35 Tier 1 symbols with TypeScript declarations: `encode_seq(units: Array<string>): Buffer` and
    `decode_seq(data: Buffer): Array<string>` in `index.d.ts`. `conformance.test.mjs` runs the seq
    vectors; `__tests__/seq.test.mjs` adds round-trip and negative tests. `@iscc/lib` 0.7.0 is
    published via OIDC
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
    `iscc_decode_seq` (returns `char**`). Committed `iscc.h` declares both; Rust `mod tests` walks
    the data.json seq vectors; `tests/test_iscc.c` adds a round trip and a zero-count case
- Existing: header freshness check, `examples/iscc_sum.c` + CMake, `docs/howto/c-cpp.md`, 0.7.0
    tarballs on GitHub Releases
- **Missing (`normal` issues):** `docs/c-ffi-api.md` uses unprefixed type names and has no seq
    entries; `src/lib.rs` is 2870 lines, past the crate's ~2500 split threshold

## Go Bindings

**Status**: partially met

- Pure Go, 35 Tier 1 symbols: `EncodeSeq` and `DecodeSeq` port the Rust code, with a private
    `readUnit`. `CGO_ENABLED=0`; IDv1 at the reference API shape; strict base32; `isccClean` at all
    5 string-input sites
- `TextTrim` / `TextRemoveNewlines` use `isPyStripChar`; `TextClean` keeps `strings.TrimSpace` by
    design. 264 `func Test`; `ConformanceSelftest` runs the seq vectors
- Stale doc: `packages/go/CLAUDE.md` still says "all 9 gen functions" and "all 46 conformance"

## C# / .NET Bindings

**Status**: partially met

- `IsccLib.EncodeSeq(string[])` → `byte[]` and `IsccLib.DecodeSeq(ReadOnlySpan<byte>)` → `string[]`
    wrap the generated P/Invoke declarations. `ConformanceTests.cs` runs both data.json seq sections
    plus a byte round trip; `SeqTests.cs` adds 10 tests. `Iscc.Lib` 0.7.0 is on NuGet
- **Missing (`normal` issues):** an embedded NUL silently truncates every string input; the
    string-array pin loop is copied three times and the gen copies pass NULL for `[]`
- Stale doc: `packages/dotnet/CLAUDE.md` still says "all 9 gen functions (50 vectors total)" and
    omits `SeqTests.cs`

## C++ Bindings

**Status**: partially met

- `iscc.hpp` has `iscc::encode_seq(const std::vector<std::string>&)` → `std::vector<uint8_t>` (L366)
    and `iscc::decode_seq(const std::vector<uint8_t>&)` → `std::vector<std::string>` (L384),
    RAII-owned, with an embedded-NUL guard on `encode_seq` only. 0.7.0 ships via vcpkg/Conan
    metadata
- `tests/test_iscc.cpp` hard-codes vectors for every function (no data.json loader); seq coverage is
    `encode_seq`/`decode_seq` test_0001, the test_0004 byte round trip, an empty-list and a NUL case
- **Missing (`normal` issue):** the other 15 `c_str()` call sites still truncate at an embedded NUL
- Stale docs: `packages/cpp/CLAUDE.md` ("35+ test cases", no string-array type-map row),
    `packages/cpp/README.md`, `docs/howto/c-cpp.md`

## Java, Ruby, Swift, Kotlin Bindings

**Status**: partially met

- All four expose the 33-symbol surface and shipped 0.7.0: Java and Kotlin on Maven Central, Ruby on
    RubyGems, Swift as an SPM XCFramework
- **Missing on all four:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness. Swift and Kotlin hard-code per-section counts
- Ruby has two open `normal` issues: `json` in `Gemfile.lock` (Dependabot) and the `File.read` call
    in `test_conformance.rb` that passes no encoding

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
- **Latest CI: PASSING** on `dee8ba3`: https://github.com/iscc/iscc-lib/actions/runs/44434612334.
    All 21 check-runs succeeded. No unpushed code changes at HEAD
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve` has two high alerts, `setuptools`),
    tracked as a `normal` issue

## Open Issues

17 open: 0 critical, **10 normal**, 7 low. The 10 `normal` issues: c-ffi-api type names, wasm npm
metadata, the Ruby `json` alert, the Ruby locale fix, the Python `encode_seq` case-fold docstrings,
the `uv.lock` alerts, the napi `Buffer` reference leak, the `iscc-ffi/src/lib.rs` split, .NET + C++
embedded-NUL truncation (fix approach awaits a human decision, not yet escalated), and .NET pin-loop
duplication.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 4 surfaces (Java, Ruby,
Swift, Kotlin), with every conformance harness running the seq vectors. Also clear the 10 open
`normal` issues that block the release.
