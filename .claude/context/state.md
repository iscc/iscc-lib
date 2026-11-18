<!-- assessed-at: 0c2269de666285b7749f4c0df43e9d6f04366f96 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core; CI red; 0 of 11 binding surfaces

The Rust core now has `encode_seq` / `decode_seq` / `read_unit` / `SEQ_MAINTYPES`, with 44 seq tests
and the seq conformance sections running. **CI on the pushed `develop` tip `9124c98` is not green.**
The enforcing cargo-deny gate fails on a new advisory, and the Swift job was cancelled before it
started. No binding exposes the seq codec yet. HEAD `0c2269d` is one unpushed human commit that
changes only `.claude/` files (the base32-strict decision), so the CI run on `9124c98` covers HEAD's
code.

## Rust Core Crate

**Status**: partially met

- Tier 1 = 35 at crate root (33 + `encode_seq` / `decode_seq` re-exported from `codec`). Tier 2
    `read_unit` + `SEQ_MAINTYPES` sit in `codec.rs`, on private raw-`u32` header helpers. 400
    `#[test]` in `crates/iscc-lib` (44 in `tests/test_seq.rs`, which includes the IEP-0020 Table 2
    vectors). `conformance_selftest` runs the 4 + 4 seq vectors
- The `iscc_decompose` doc now points to `decode_seq` (iscc-core 1.5.0 docstring)
- **Missing (criterion added by `0c2269d`):** tests asserting that decoding rejects
    `ISCC:GAAAAAAAAB`, `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` and `ISCC:GAAAAAAAAA======`. There are 0
    hits in `crates/`
- iai Perf and Coverage + CRAP both passed on `9124c98` against the regenerated CRAP baseline (116
    entries). semver-checks is informational. The v1.0.0 cut and the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 33 Tier 1 symbols; abi3-py310 wheels for Linux x86_64/aarch64, macOS universal2 and Windows x64;
    12 `py.detach` sites; 0.7.0 on PyPI
- **Missing:** `encode_seq` / `decode_seq`, the seq conformance vectors, and the differential pytest
    against `iscc_core` 1.5.0. The spec now requires that test to assert the strict-base32
    divergence (padding and spare bits) explicitly

## Node.js Bindings

**Status**: partially met

- 33 Tier 1 symbols with TypeScript declarations; `@iscc/lib` 0.7.0 published via OIDC
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors

## WASM Bindings

**Status**: partially met

- 33 Tier 1 symbols; SIMD release build (`+simd128`, `wasm-opt --enable-simd`) met
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors
- Open `normal` issue: `crates/iscc-wasm/Cargo.toml` has no `license` or `description`

## C FFI

**Status**: partially met

- 49 `#[unsafe(no_mangle)]` exports; committed `iscc.h` with a freshness check; C test program;
    `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** `iscc_encode_seq` / `iscc_decode_seq` and the seq vectors in `tests/test_iscc.c`
- Open `normal` issue: `docs/c-ffi-api.md` uses unprefixed type names

## Go Bindings

**Status**: partially met

- Pure Go, 33 Tier 1 symbols, 179 `func Test`, `CGO_ENABLED=0`; IDv1 at the reference API shape
- **Missing (two prerequisite `normal` issues):** no Go `isccClean` helper, and `decodeBase32`
    accepts spare-bit aliases (e.g. `ISCC:GAAAAAAAAB`) that Rust rejects. No Go test pins strict
    base32 decoding
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader (`parseConformanceData` loads only
    `gen_*` sections), and seq test cases

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0 (Java and Kotlin on Maven Central,
    RubyGems, NuGet, vcpkg/Conan metadata, SPM XCFramework)
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness (Swift and Kotlin hard-code per-section counts)
- Ruby has two open `normal` issues: `json` 2.21.1 in `Gemfile.lock` (Dependabot #1) and the
    `File.read` call in `test_conformance.rb` that passes no encoding

## Documentation

**Status**: met (for 0.7.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, a
    Development section; deploys from `main`. The root README and 12 per-crate/package READMEs are
    present
- Pending for v0.8.0: seq API entries, and the 33 → 35 count in the crate `CLAUDE.md`, `notes/00`,
    `notes/04`, `docs/architecture.md` and the binding docs

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind (`crates/iscc-lib/benches/`); pytest-benchmark comparing
    iscc-core and iscc-lib in `benchmarks/python/`; speedups published in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs. `release.yml` is `workflow_dispatch`-only with 8 registry toggles and
    creates the `vX.Y.Z` + `packages/go/vX.Y.Z` tags. 21 version-sync targets at 0.7.0
- **Latest CI: FAILING** on `9124c98` (conclusion `cancelled`):
    https://github.com/iscc/iscc-lib/actions/runs/44210618484. 19 jobs passed, including CRAP, Perf,
    and all language suites except Swift
- **Failed — `Audit (cargo-deny)` (enforcing):** `RUSTSEC-2026-0374`, `basic-toml` unmaintained. Per
    `Cargo.lock` it arrives via `askama_derive` ← `askama` ← `uniffi_bindgen`. This is a new
    advisory, not a code change. Not tracked in issues.md
- **Cancelled — `Swift (swift build, swift test)`:** it never got a runner. The job annotation reads
    "The configuration 'macos-14-arm64-us-default' is not supported". `ci.yml:251` uses
    `runs-on: macos-14`, and `release.yml` uses `macos-14` at 11 sites, so the release path is
    exposed too. Not tracked in issues.md
- **Untracked:** Dependabot alerts #2–#6 on `uv.lock`: `soupsieve` 2.9.1 (2 high, 2 medium) and
    `setuptools` 83.0.0 (low). Dev-group only, and there is no `issues.md` entry

## Open Issues

13 open: 0 critical, **6 normal**, 7 low. All 6 `normal` are actionable and unresolved: c-ffi-api
type names, Go `iscc_clean`, Go `decodeBase32` spare-bit aliases, wasm npm metadata, Ruby `json`
alert, Ruby locale. The `cid:run` fallback-model issue was deleted in review.

## Next Milestone

Restore green CI on `develop` (cargo-deny advisory, Swift runner). Then continue v0.8.0 ISCC-SEQ:
`encode_seq` / `decode_seq` on all 11 surfaces with every harness running the seq vectors, the
strict-base32 rejection tests, and clearing the actionable `normal` issues that block the release.
