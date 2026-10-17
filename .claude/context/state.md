<!-- assessed-at: ad69c8e6024e4a7167038936019fcc00d304c73e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ codec not started; 6 `normal` issues open

v0.7.0 shipped 2026-10-17 to all registries and CI is fully green at the `develop` tip `d49b953`.
HEAD `ad69c8e` adds only `.claude/` context changes. The v0.8.0 goal, `encode_seq` / `decode_seq` on
the Rust core and all 11 surfaces (Tier 1 33 → 35), has **zero implementation anywhere**. The four
`encode_seq` and four `decode_seq` vectors are vendored (5 identical `data.json` copies) but no
harness runs them.

## Rust Core Crate

**Status**: partially met

- Tier 1 = 33 symbols at crate root; 353 `#[test]` across `crates/iscc-lib`; all `gen_*_v0`
    conformance sections pass in CI; Image-Code DCT-quadrant fix (iscc-core 1.5.0) landed
- `codec::iscc_clean` shared by all Rust codec-input sites (reference parity, empty-cleaned guard)
- **Missing:** `encode_seq`, `decode_seq` (Tier 1), `read_unit`, `SEQ_MAINTYPES` (Tier 2), the
    `test_seq.py` port, and runs of the seq sections of the conformance harness
- **Missing:** `iscc_decompose` doc comment (`codec.rs:609`) not yet aligned with iscc-core 1.4.0
    (no pointer to `decode_seq`)
- iai Perf + CRAP regression gates green; semver-checks informational; the v1.0.0 cut and MSRV
    criterion are human-gated and out of scope for v0.8.0

## Python Bindings

**Status**: partially met

- 33 Tier 1 symbols, abi3-py310 wheels (Linux x86_64/aarch64, macOS universal2, Windows x64), 12
    `py.detach` sites on the heavy paths; 0.7.0 on PyPI
- **Missing:** `encode_seq` / `decode_seq`, the seq conformance vectors, and the differential pytest
    against `iscc_core` 1.5.0 (`uv.lock` already pins 1.5.0)

## Node.js Bindings

**Status**: partially met

- 33 Tier 1 symbols with TypeScript declarations; `@iscc/lib` 0.7.0 published via OIDC trusted
    publishing
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors

## WASM Bindings

**Status**: partially met

- 33 Tier 1 symbols; SIMD release build (`+simd128`, `wasm-opt --enable-simd`) met
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors
- Open `normal` issue: `crates/iscc-wasm/Cargo.toml` has no `license` or `description`, so
    `@iscc/wasm` on npm is published unlicensed with a badge-markdown description

## C FFI

**Status**: partially met

- 49 `#[unsafe(no_mangle)]` exports, committed `iscc.h` with a freshness check, C test program,
    `examples/iscc_sum.c` + CMake, `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** `iscc_encode_seq` / `iscc_decode_seq` and the seq vectors in `tests/test_iscc.c`
- Open `normal` issue: `docs/c-ffi-api.md` uses unprefixed type names (6 hits); the header declares
    `iscc_IsccDecodeResult` etc. (0 prefixed uses on the page)

## Go Bindings

**Status**: partially met

- Pure Go, 33 Tier 1 symbols, 179 `func Test`, `CGO_ENABLED=0`; IDv1 at reference API shape;
    `parseConformanceData` deliberately loads only `gen_*` sections (`conformance.go:27`)
- **Missing (prerequisite):** no Go `isccClean` helper (0 hits); the four codec-input sites still
    clean ad-hoc — `normal` issue "Go codec input cleaning diverges", blocks the Go `encode_seq`
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, own seq test cases, and a seq-vector loader

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0 (Maven Central Java + Kotlin, RubyGems,
    NuGet, vcpkg/Conan metadata, SPM XCFramework); JNI 34 natives, Ruby 34 `define_*`, UniFFI 33
    exports
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness (Swift and Kotlin hard-code per-section counts)
- Ruby has two open `normal` issues: `json` 2.21.1 in `Gemfile.lock` (Dependabot alert #1 `open`),
    and `test_conformance.rb:13` calls `File.read` with no explicit encoding (locale-dependent)

## Documentation

**Status**: met (for 0.7.0 scope)

- 22 docs pages (`docs/unicode.md` removed with the Unicode pin), 11 how-to guides, Diátaxis
    navigation, ISCC branding, `llms-full.txt`, Development section; deploys from `main`
- Root README and 12 per-crate/package READMEs present
- Pending for v0.8.0: seq API entries plus the 33 → 35 count in crate `CLAUDE.md`, `notes/00` and
    `notes/04`, `docs/architecture.md` and the binding docs (to land with the seq step)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind (`crates/iscc-lib/benches/`); pytest-benchmark iscc-core vs
    iscc-lib in `benchmarks/python/`; speedups published in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs (the `unicode-sweep` job was removed), all green; `release.yml` has
    `workflow_dispatch` with 8 registry toggles; npm is on OIDC (no `NPM_TOKEN`); 21 version-sync
    targets all `OK` at 0.7.0
- PR #48 (`fix/image-code-dct-quadrants` → `main`) is still open although that fix shipped in 0.7.0
    via #49
- Latest CI: **passing** — `GitHub Actions` check-suites `success` on `d49b953`, including Perf,
    Coverage + CRAP, Semver, Swift and Kotlin

## Open Issues

13 open: 0 critical, **6 normal** (c-ffi-api type names, Go `iscc_clean`, wasm npm metadata, Ruby
`json` alert, Ruby locale, `cid:run` fallback model; all actionable, none blocked on the human or
upstream), 7 low. All 6 `normal` issues re-verified as still unresolved.

## Next Milestone

v0.8.0 ISCC-SEQ: land `encode_seq` / `decode_seq` (+ Tier 2 `read_unit` / `SEQ_MAINTYPES`) in the
Rust core with the ported `test_seq.py` cases, then the Go `iscc_clean` prerequisite, then the
remaining surfaces and conformance harnesses. The six actionable `normal` issues also block the
release.
