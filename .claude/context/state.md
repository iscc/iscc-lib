<!-- assessed-at: 03e57a9276c214483eae1d0ef3c4217e93d90c34 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core + Python (1 of 11 surfaces); CI green

CI on the pushed `develop` tip `05dd0c8` is green on all 21 check-runs, Swift included, so the two
runner-capacity cancels are behind us. HEAD `03e57a9` adds two commits that touch only `.claude/`,
so that run covers HEAD's code. Rust and Go codec decoding now upper-cases ASCII only, and Titusz
confirmed that `encode_seq` stays strict where iscc-core accepts the aliases. ISCC-SEQ is still
bound on Python only. 7 `normal` issues are open.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 421 `#[test]`: 44 in `tests/test_seq.rs`, 17 in `tests/codec_clean.rs` and
    13 in `tests/test_strict_base32.rs`. `conformance_selftest` runs the seq vectors
- Strict base32 is met. Padding, spare bits and interior newlines are rejected, and all 3 decoding
    sites use `to_ascii_uppercase` (`codec.rs` `decode_base32` and `encode_seq`, `lib.rs`
    `iscc_normalize`). Non-ASCII case-fold aliases are rejected, and `rust-core.md` "Base32 input is
    strict" now records the confirmed `encode_seq` divergence from iscc-core
- **Missing (`normal` issue):** `text_trim` and `text_remove_newlines` keep U+001C–U+001F, which
    iscc-core strips
- iai Perf, Coverage + CRAP and semver-checks passed on `05dd0c8` (semver is informational). The
    v1.0.0 cut and the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols, including `encode_seq` (returns `bytes`) and `decode_seq` (returns
    `list[str]`), with `.pyi` stubs and `__all__`. abi3-py310 wheels for 4 platforms; 12 `py.detach`
    sites; 0.7.0 on PyPI
- `tests/test_seq.py` has 29 test functions. `STRICT_BASE32_DIVERGENCES` pins padding, spare bits
    and the two case-fold rows (`long_s_case_fold`, `sharp_s_case_fold`): iscc-core accepts,
    iscc-lib rejects
- **Missing (`normal` issue):** the `encode_seq` docstrings (`_lowlevel.pyi:56`, the `__init__.py`
    wrapper) don't mention the case-fold divergence
- Stale doc: `crates/iscc-py/CLAUDE.md` still says "27 #[pyfunction]s"

## Node.js Bindings

**Status**: partially met

- 33 Tier 1 symbols with TypeScript declarations; `@iscc/lib` 0.7.0 published via OIDC
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors

## WASM Bindings

**Status**: partially met

- 33 Tier 1 symbols; the SIMD release build (`+simd128`, `wasm-opt --enable-simd`) is met
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors
- Open `normal` issue: `crates/iscc-wasm/Cargo.toml` has no `license` and no `description`

## C FFI

**Status**: partially met

- 49 `#[unsafe(no_mangle)]` exports; committed `iscc.h` with a freshness check; C test program;
    `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** `iscc_encode_seq` / `iscc_decode_seq` and the seq vectors in `tests/test_iscc.c`
- Open `normal` issue: `docs/c-ffi-api.md` uses unprefixed type names

## Go Bindings

**Status**: partially met

- Pure Go, 33 Tier 1 symbols, 197 `func Test`, `CGO_ENABLED=0`; IDv1 at the reference API shape;
    strict canonical base32 decoding; `isccClean` at all four string-input sites
- ASCII-only upper-casing is met: the `asciiUpper` helper (`codec.go:563`) replaces
    `strings.ToUpper` in `decodeBase32` and `checkPrefix`, with alias/twin tests in `codec_test.go`
- `asciiUpper` always copies, even for input that is already upper case (review measured 26 → 34
    allocations per `IsccDecode`). Not filed; review suggests adding a fast path in the Go seq step
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader, and seq test cases including the
    `STRICT_BASE32_DIVERGENCES` inputs. The prerequisites (cleaning, ASCII upper-casing) are done

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0: Java and Kotlin on Maven Central, Ruby on
    RubyGems, .NET on NuGet, C++ through vcpkg/Conan metadata, Swift as an SPM XCFramework
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness. Swift and Kotlin hard-code per-section counts. The .NET work depends on the C FFI step
- Ruby has two open `normal` issues: `json` in `Gemfile.lock` (Dependabot) and the `File.read` call
    in `test_conformance.rb` that passes no encoding
- Swift passes again on `05dd0c8`

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
- Review 198 reported 10 of 16 iai benches running 3.5–7.7% below the committed baseline, which
    loosens the effective tolerance on those benches. A baseline refresh needs authorisation

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; Swift is the only job on macOS (`macos-15`). `release.yml` is
    `workflow_dispatch`-only with 8 registry toggles. 21 version-sync targets are at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: PASSING** on `05dd0c8`: https://github.com/iscc/iscc-lib/actions/runs/44357670484.
    All 21 check-runs succeeded, Swift included. No unpushed code changes at HEAD
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve`, `setuptools`), tracked as a `normal`
    issue

## Open Issues

14 open: 0 critical, **7 normal**, 7 low. The 7 `normal` issues: c-ffi-api type names, wasm npm
metadata, the Ruby `json` alert, the Ruby locale fix, the Python `encode_seq` case-fold docstrings,
the `uv.lock` alerts, and text-utils U+001C–U+001F trimming. All 7 can be worked now. The base32
case-fold alias issue is closed.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 10 surfaces, with every
conformance harness running the seq vectors. Also clear the 7 open `normal` issues that block the
release.
