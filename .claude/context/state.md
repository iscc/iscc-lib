<!-- assessed-at: 0ca1978155436ea580cf162d3ccf329f6f812e2e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core + Python; CI green; 1 of 11 binding surfaces

CI is green on the pushed `develop` tip `3e1f8e5`, which includes the core `iscc_clean` strip-set
fix. HEAD `0ca1978` is one unpushed loop-log commit that touches only `.claude/` files, so the green
run covers HEAD's code. ISCC-SEQ is bound on Python only, leaving 10 surfaces. 8 `normal` issues are
open and all of them can be worked now.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 408 `#[test]`: 44 in `tests/test_seq.rs` and 17 in `tests/codec_clean.rs`.
    `conformance_selftest` runs the seq vectors
- `codec::iscc_clean` now trims the Python `str.strip()` set through `is_py_strip_char`, which adds
    U+001C–U+001F. All four codec trim sites use it, and `codec.rs` has no `.trim()` calls left
- **Missing:** Rust tests asserting that decoding rejects `ISCC:GAAAAAAAAB` and
    `ISCC:GAAAAAAAAA======` (strict base32, decisions.md 2026-11-18). There are zero hits in
    `crates/iscc-lib`
- **Missing (`normal` issue):** `text_trim` and `text_remove_newlines` still keep U+001C–U+001F,
    which iscc-core strips
- iai Perf and Coverage + CRAP pass on `3e1f8e5`. semver-checks is informational. The v1.0.0 cut and
    the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols: `encode_seq` (returns `bytes`) and `decode_seq` (returns `list[str]`) in
    `src/lib.rs`, the `__init__.py` wrappers, `__all__` and `_lowlevel.pyi`. abi3-py310 wheels for 4
    platforms; 12 `py.detach` sites; 0.7.0 on PyPI
- `tests/test_seq.py` has 29 test functions: fixture conformance, round trip, bad inputs, the
    `iscc_core` strict-base32 differential, and whitespace-rejection differentials in both
    directions, with the strip set derived live. The seq "both sides reject the same inputs"
    criterion is no longer blocked
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

- Pure Go, 33 Tier 1 symbols, 179 `func Test`, `CGO_ENABLED=0`; IDv1 at the reference API shape
- **Missing (prerequisite `normal` issues):** an `isccClean` helper that strips the Python strip
    set, U+001C–U+001F included. `decodeBase32` also accepts spare-bit aliases (e.g.
    `ISCC:GAAAAAAAAB`) that Rust rejects
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader (`parseConformanceData` loads only
    the `gen_*` sections), and seq test cases

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0: Java and Kotlin on Maven Central, Ruby on
    RubyGems, .NET on NuGet, C++ through vcpkg/Conan metadata, Swift as an SPM XCFramework
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness. Swift and Kotlin hard-code per-section counts. The .NET work depends on the C FFI step
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
- 10 of 16 iai benches sit 3.5–7.7% below the committed baseline, so the effective regression
    tolerance on those benches is looser than 10%. Refreshing the baseline needs authorisation

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; macOS jobs run on `macos-15`. `release.yml` is `workflow_dispatch`-only with
    8 registry toggles. 21 version-sync targets are at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: PASSING** on `3e1f8e5`: https://github.com/iscc/iscc-lib/actions/runs/44310966154.
    All 21 check-runs succeeded
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve`, `setuptools`), tracked as a `normal`
    issue

## Open Issues

15 open: 0 critical, **8 normal**, 7 low. All 8 `normal` issues can be worked now: the c-ffi-api
type names, the Go `iscc_clean` port, Go `decodeBase32` spare-bit aliases, text-utils U+001C–U+001F
trimming, wasm npm metadata, the Ruby `json` alert, the Ruby locale fix, and the `uv.lock` alerts.
The core `iscc_clean` issue is closed.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 10 surfaces, with every
harness running the seq vectors. Also needed: strict-base32 and cleaning parity in Rust tests and
Go, and clearing the 8 open `normal` issues that block the release.
