<!-- assessed-at: e6e5c705f8a169ff31351e78d41e93e181168785 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ codec not started; 6 `normal` issues open

v0.7.0 shipped 2026-10-17 and CI is green on the pushed `develop` tip `a0cf7a2`. HEAD `e6e5c70` is
two unpushed human commits ahead: a sharper ISCC-SEQ spec and CID-runner changes (`tools/cid.py`,
`tests/test_cid.py`, `mise.toml`, `CLAUDE.md`) that no CI run covers yet. The v0.8.0 goal,
`encode_seq` / `decode_seq` on the Rust core and all 11 surfaces (Tier 1 33 → 35), still has **zero
implementation anywhere**. The seq vectors are vendored in all 5 `data.json` copies but no harness
runs them.

## Rust Core Crate

**Status**: partially met

- Tier 1 = 33 symbols at crate root; 353 `#[test]` in `crates/iscc-lib`; all `gen_*_v0` conformance
    sections pass in CI; `codec::iscc_clean` is shared by all Rust codec-input sites
- **Missing:** `encode_seq`, `decode_seq` (Tier 1), `read_unit`, `SEQ_MAINTYPES` (Tier 2), the
    `test_seq.py` port, and runs of the seq sections of the conformance harness
- **Missing (spec sharpened at `88513bb`):** explicit IEP-0020 accept/reject decoder vectors
    (SubType 7, Version-1-then-0 ordering, the 7 Table 2 rejects) and a guard that `iscc_decode`,
    `iscc_decompose` and `encode_component` keep their current accept/reject behaviour (the spec
    rules out widening `SubType` / `Version`; `read_unit` needs a raw-integer header codec)
- **Missing:** `iscc_decompose` doc comment (`codec.rs`) not yet aligned with iscc-core 1.5.0 (no
    pointer to `decode_seq`)
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

- 33 Tier 1 symbols with TypeScript declarations; `@iscc/lib` 0.7.0 published via OIDC
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors

## WASM Bindings

**Status**: partially met

- 33 Tier 1 symbols; SIMD release build (`+simd128`, `wasm-opt --enable-simd`) met
- **Missing:** `encode_seq` / `decode_seq` and the seq conformance vectors
- Open `normal` issue, still unresolved: `crates/iscc-wasm/Cargo.toml` has no `license` or
    `description`, so `@iscc/wasm` on npm ships unlicensed

## C FFI

**Status**: partially met

- 49 `#[unsafe(no_mangle)]` exports, committed `iscc.h` with a freshness check, C test program,
    `examples/iscc_sum.c` + CMake, `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** `iscc_encode_seq` / `iscc_decode_seq` and the seq vectors in `tests/test_iscc.c`
- Open `normal` issue, still unresolved: `docs/c-ffi-api.md` uses unprefixed type names (0 uses of
    the header's `iscc_IsccDecodeResult`)

## Go Bindings

**Status**: partially met

- Pure Go, 33 Tier 1 symbols, 179 `func Test`, `CGO_ENABLED=0`; IDv1 at reference API shape;
    `parseConformanceData` deliberately loads only `gen_*` sections
- **Missing (prerequisite):** no Go `isccClean` helper (0 hits) — `normal` issue "Go codec input
    cleaning diverges", which blocks the Go `encode_seq`
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, own seq test cases, and a seq-vector loader

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0 (Maven Central Java + Kotlin, RubyGems,
    NuGet, vcpkg/Conan metadata, SPM XCFramework)
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness (Swift and Kotlin hard-code per-section counts)
- Ruby: two open `normal` issues still unresolved — `json` 2.21.1 in `Gemfile.lock` (Dependabot
    alert #1 open, patched 2.21.2) and `test_conformance.rb:13` `File.read` with no encoding

## Documentation

**Status**: met (for 0.7.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; deploys from `main`; root README + 12 per-crate/package READMEs present
- Pending for v0.8.0: seq API entries and the 33 → 35 count in crate `CLAUDE.md`, `notes/00`,
    `notes/04`, `docs/architecture.md` and the binding docs

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind (`crates/iscc-lib/benches/`); pytest-benchmark iscc-core vs
    iscc-lib in `benchmarks/python/`; speedups published in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles, 28 of 29 jobs
    carry the `!cancelled() && !failure()` guard (all but `prepare-release`), and it creates the
    `vX.Y.Z` + `packages/go/vX.Y.Z` tags — matches the revised target and `ci-cd.md`
- Latest CI: **passing** on `a0cf7a2` (21/21 check-runs success) —
    https://github.com/iscc/iscc-lib/actions/runs/39066853481. **Does not cover HEAD**: the 2
    unpushed commits change `tools/cid.py` (+217/−) and `tests/test_cid.py` (+192/−)
- **Untracked:** Dependabot alerts #2–#6 on `uv.lock` — `soupsieve` 2.9.1 (2 high, 2 medium; patched
    ≥ 3.0.2/4.0.0, via `mdformat-mkdocs` → `beautifulsoup4`) and `setuptools` 83.0.0 (low; via
    `yamlfix`/`mdformat-mkdocs`). Dev-group only, but `ci-cd.md` "Dependency Freshness" says
    releases never ship vulnerable pins; no `issues.md` entry
- 21 version-sync targets at 0.7.0

## Open Issues

13 open: 0 critical, **6 normal**, 7 low. Five `normal` are actionable and unresolved (c-ffi-api
type names, Go `iscc_clean`, wasm npm metadata, Ruby `json` alert, Ruby locale). The sixth,
"`cid:run` passes a fallback model identical to the primary", is contradicted by human commit
`e6e5c70`: `mise.toml` now passes `--fallback-model claude-opus-5` (differs from the primary, so the
title's premise is gone), but the issue's exit criterion (no `--fallback-model` at all) is unmet —
the human or review must close or re-scope it.

## Next Milestone

v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` (+ Tier 2 `read_unit` / `SEQ_MAINTYPES`) in the Rust
core and on all 11 surfaces with every conformance harness running the seq vectors, plus clearing
the actionable `normal` issues that block the release.
