<!-- assessed-at: 8367d61a9c148d8dbdc542e0cff434ba5a4a95b4 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core + Python; CI green; 1 of 11 binding surfaces

CI is green on the pushed `develop` tip `72054e8`, which includes the Python seq binding. HEAD
`8367d61` is one unpushed loop-log commit touching only `.claude/` files, so that green run covers
HEAD's code. ISCC-SEQ is bound on Python only; 10 surfaces remain. 8 `normal` issues are open (one
new: core `iscc_clean` whitespace parity) and all can be worked now.

## Rust Core Crate

**Status**: partially met

- Tier 1 = 35 at the crate root (`encode_seq` / `decode_seq` re-exported from `codec`). Tier 2
    `read_unit` + `SEQ_MAINTYPES` live in `codec.rs`. 400 `#[test]` in `crates/iscc-lib`, 44 of them
    in `tests/test_seq.rs`. `conformance_selftest` runs the seq vectors
- **Missing:** Rust tests asserting that decoding rejects `ISCC:GAAAAAAAAB` and
    `ISCC:GAAAAAAAAA======` (strict base32, decisions.md 2026-11-18); zero hits in `crates/iscc-lib`
- **Missing (new `normal` issue):** `codec::iscc_clean` trims with `str::trim`, so it keeps
    U+001C–U+001F, which iscc-core's `str.strip()` removes. This affects every cleaned entry point
- iai Perf and Coverage + CRAP pass on `72054e8`. semver-checks is informational. The v1.0.0 cut and
    the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols: `encode_seq` (returns `bytes`) and `decode_seq` (returns `list[str]`) are in
    `src/lib.rs`, the `__init__.py` wrappers, `__all__` and `_lowlevel.pyi`. abi3-py310 wheels for 4
    platforms; 12 `py.detach` sites; 0.7.0 on PyPI
- `tests/test_seq.py` (23 test functions): fixture conformance, round trip, bad inputs, an
    `iscc_core` differential asserting the strict-base32 divergence, and a shared-rejection
    differential
- **Missing:** the spec's "both sides reject the same inputs" criterion is blocked by the
    `iscc_clean` U+001C–U+001F issue. `crates/iscc-py/CLAUDE.md` still says "27 #[pyfunction]s"

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
- **Missing (prerequisite `normal` issues):** Go has no `isccClean` helper (it must strip the same
    set as the fixed core, including U+001C–U+001F), and `decodeBase32` accepts spare-bit aliases
    (e.g. `ISCC:GAAAAAAAAB`) that Rust rejects
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader (`parseConformanceData` loads only
    the `gen_*` sections), and seq test cases

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0 (Java and Kotlin on Maven Central,
    RubyGems, NuGet, vcpkg/Conan metadata, SPM XCFramework)
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness (Swift and Kotlin hard-code per-section counts). The .NET path depends on the C FFI step
- Ruby has two open `normal` issues: `json` in `Gemfile.lock` (Dependabot) and the `File.read` call
    in `test_conformance.rb` that passes no encoding

## Documentation

**Status**: met (for 0.7.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, a
    Development section; deploys from `main`. The root README and 12 per-crate/package READMEs are
    present
- Pending for v0.8.0: seq API entries, and the 33 → 35 count in the crate `CLAUDE.md` files,
    `notes/00`, `notes/04`, `docs/architecture.md` and the binding docs

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind (`crates/iscc-lib/benches/`); pytest-benchmark comparing
    iscc-core and iscc-lib in `benchmarks/python/`; speedups published in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; macOS jobs on `macos-15`. `release.yml` is `workflow_dispatch`-only with 8
    registry toggles; its 10 macOS sites use `macos-15`, checked statically only until the next
    release. 21 version-sync targets at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: PASSING** on `72054e8`: https://github.com/iscc/iscc-lib/actions/runs/44295959238.
    All 21 check-runs succeeded
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve`, `setuptools`), tracked as a `normal`
    issue

## Open Issues

15 open: 0 critical, **8 normal**, 7 low. All 8 `normal` issues can be worked now: c-ffi-api type
names, Go `iscc_clean`, Go `decodeBase32` spare-bit aliases, core `iscc_clean` U+001C–U+001F
trimming, wasm npm metadata, the Ruby `json` alert, the Ruby locale fix, and the `uv.lock` alerts.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 10 surfaces with every
harness running the seq vectors, plus cleaning and strict-base32 parity in the core and Go. Also
clear the 8 open `normal` issues that block the release.
