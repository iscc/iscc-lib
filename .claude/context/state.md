<!-- assessed-at: 38cb7009c1492b66589dd67ea25e73829eaa3a62 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ on Rust core, Python and Go (2 of 11 surfaces); CI green

The pushed `develop` tip `c19184a` passes CI on all 21 check-runs. HEAD `38cb700` adds only a
`cid(log)` commit, so that run covers HEAD's code. Iteration 202 added Go `EncodeSeq` / `DecodeSeq`
with seq conformance runners. ISCC-SEQ is now on 2 of 11 surfaces (Python, Go). 7 `normal` issues
are open.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 421 `#[test]`, with 44 in `tests/test_seq.rs`. `conformance_selftest` runs
    the seq vectors
- Strict base32 is met, with ASCII-only upper-casing at all 3 decoding sites. The `encode_seq`
    divergence from iscc-core (case-fold aliases) is recorded in `rust-core.md` and decisions.md
- **Missing (`normal` issue):** `text_trim` and `text_remove_newlines` keep U+001C–U+001F, which
    iscc-core strips
- iai Perf, Coverage + CRAP and semver-checks passed on `c19184a` (semver is informational). The
    v1.0.0 cut and the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols, including `encode_seq` (returns `bytes`) and `decode_seq` (returns
    `list[str]`), with `.pyi` stubs and `__all__`. abi3-py310 wheels for 4 platforms; 12 `py.detach`
    sites; 0.7.0 on PyPI
- `tests/test_seq.py` pins `STRICT_BASE32_DIVERGENCES` (padding, spare bits, two case-fold rows)
- **Missing (`normal` issue):** the `encode_seq` docstrings don't mention the case-fold divergence
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

- Pure Go, 35 Tier 1 symbols: `EncodeSeq` (`codec.go:879`) and `DecodeSeq` (`:919`) port the Rust
    code, with a private `readUnit`. `CGO_ENABLED=0`; IDv1 at the reference API shape; strict
    base32; `isccClean` at all 5 string-input sites, `EncodeSeq` included
- 260 `func Test` (197 before), including `seq_test.go`. `ConformanceSelftest` runs the `encode_seq`
    / `decode_seq` vectors. `asciiUpper` (`codec.go:555`) has a no-copy fast path
- **Missing (falls under the `normal` text-utils issue):** `TextTrim` uses `strings.TrimSpace` and
    `TextRemoveNewlines` uses `strings.Fields` (`utils.go:91,101,109`). Go's whitespace set leaves
    out U+001C–U+001F, so Go has the same gap as Rust
- Stale doc: `packages/go/CLAUDE.md` still says "all 9 gen functions" and "all 46 conformance"

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
- 10 of 16 iai benches run 3.5–7.7% below the committed baseline (review 198), which loosens the
    effective tolerance. A baseline refresh needs authorisation

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; Swift is the only job on macOS (`macos-15`). `release.yml` is
    `workflow_dispatch`-only with 8 registry toggles. 21 version-sync targets are at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: PASSING** on `c19184a`: https://github.com/iscc/iscc-lib/actions/runs/44371959542.
    All 21 check-runs succeeded. No unpushed code changes at HEAD
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve`, `setuptools`), tracked as a `normal`
    issue

## Open Issues

14 open: 0 critical, **7 normal**, 7 low. The 7 `normal` issues: c-ffi-api type names, wasm npm
metadata, the Ruby `json` alert, the Ruby locale fix, the Python `encode_seq` case-fold docstrings,
the `uv.lock` alerts, and text-utils U+001C–U+001F trimming. None is blocked.

## Next Milestone

Continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 9 surfaces, with every
conformance harness running the seq vectors. Also clear the 7 open `normal` issues that block the
release.
