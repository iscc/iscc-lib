<!-- assessed-at: c5ab678642466fcde8ddc122992103623269a93e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core; CI green; 0 of 11 binding surfaces

CI is green again on the pushed `develop` tip `f335226`. The cargo-deny advisory now has a justified
ignore, and the macOS jobs run on `macos-15`. The Rust core has the ISCC-SEQ codec, but no binding
exposes it yet. 7 `normal` issues are open and all can be worked now. HEAD `c5ab678` is one unpushed
loop-log commit that changes only `.claude/` files, so the green run covers HEAD's code.

## Rust Core Crate

**Status**: partially met

- Tier 1 = 35 at the crate root (`encode_seq` / `decode_seq` re-exported from `codec`). Tier 2
    `read_unit` + `SEQ_MAINTYPES` live in `codec.rs`. 400 `#[test]` in `crates/iscc-lib`, 44 of them
    in `tests/test_seq.rs`. `conformance_selftest` runs the seq vectors
- **Missing:** tests asserting that decoding rejects `ISCC:GAAAAAAAAB`,
    `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` and `ISCC:GAAAAAAAAA======` (strict base32, decisions.md
    2026-11-18)
- iai Perf and Coverage + CRAP pass on `f335226`. semver-checks is informational. The v1.0.0 cut and
    the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 33 Tier 1 symbols; abi3-py310 wheels for Linux x86_64/aarch64, macOS universal2 and Windows x64;
    12 `py.detach` sites; 0.7.0 on PyPI
- **Missing:** `encode_seq` / `decode_seq`, the seq conformance vectors, and a differential pytest
    against `iscc_core` 1.5.0 that asserts the strict-base32 divergence (padding and spare bits)

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
- **Missing (two prerequisite `normal` issues):** Go has no `isccClean` helper, and `decodeBase32`
    accepts spare-bit aliases (e.g. `ISCC:GAAAAAAAAB`) that Rust rejects
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader (`parseConformanceData` loads only
    the `gen_*` sections), and seq test cases

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0 (Java and Kotlin on Maven Central,
    RubyGems, NuGet, vcpkg/Conan metadata, SPM XCFramework)
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness (Swift and Kotlin hard-code per-section counts)
- Ruby has two open `normal` issues: `json` in `Gemfile.lock` (Dependabot #1) and the `File.read`
    call in `test_conformance.rb` that passes no encoding

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

- `ci.yml` has 20 jobs; Swift runs on `macos-15`. `release.yml` is `workflow_dispatch`-only with 8
    registry toggles, and all 10 of its macOS sites use `macos-15`. That half is checked statically
    only until the next release. There are 21 version-sync targets at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` (`basic-toml` via uniffi_bindgen's askama proc-macro),
    with a documented reason. No upstream version drops it
- **Latest CI: PASSING** on `f335226`: https://github.com/iscc/iscc-lib/actions/runs/44283260882.
    All 21 check-runs succeeded
- **Missing:** the `uv.lock` Dependabot bump (alerts #2–#6: `soupsieve`, 2 high and 2 medium;
    `setuptools`, low), now tracked as a `normal` issue

## Open Issues

14 open: 0 critical, **7 normal**, 7 low. All 7 `normal` issues can be worked now: c-ffi-api type
names, Go `iscc_clean`, Go `decodeBase32` spare-bit aliases, wasm npm metadata, the Ruby `json`
alert, the Ruby locale fix, and the `uv.lock` alerts.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on all 11 surfaces, with every harness running
the seq vectors, plus the strict-base32 rejection tests. Also clear the 7 open `normal` issues that
block the release.
