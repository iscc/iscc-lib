<!-- assessed-at: fd3f6e3b5f72dbf67de5b96bfb50a7b352b9a06f -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — ISCC-SEQ in Rust core + Python (1 of 11 surfaces); CI red on the develop tip

CI on the pushed `develop` tip `5abda9f` ended as `failure`. The only failing job is Swift, which
was cancelled because no hosted macOS runner picked it up; none of its steps ran. The other 20
check-runs passed. HEAD `fd3f6e3` is one unpushed loop-log commit that touches only `.claude/`
files, so the failing run covers HEAD's code. Strict base32 is now pinned in Rust and Go. ISCC-SEQ
is bound on Python only, and 8 `normal` issues are open.

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root. Tier 2 `read_unit` + `SEQ_MAINTYPES` are in `codec.rs`.
    `crates/iscc-lib` has 415 `#[test]`: 44 in `tests/test_seq.rs`, 17 in `tests/codec_clean.rs` and
    7 in `tests/test_strict_base32.rs`. `conformance_selftest` runs the seq vectors
- Strict base32 is met: `test_strict_base32.rs` checks that `ISCC:GAAAAAAAAB`,
    `ISCC:GAAAAAAAAA======` and an interior newline are rejected with `base32 decode error`, and
    that the two canonical forms are accepted. `codec::iscc_clean` trims the Python strip set
- **Missing (`normal` issue):** decoding upper-cases with Unicode rules, so non-ASCII letters (e.g.
    U+0131, `ß`, `ﬁ`) decode as aliases of canonical codes. iscc-core rejects them. The fix is on
    the iai-gated codec path
- **Missing (`normal` issue):** `text_trim` and `text_remove_newlines` keep U+001C–U+001F, which
    iscc-core strips
- iai Perf and Coverage + CRAP passed on `5abda9f`. semver-checks is informational. The v1.0.0 cut
    and the MSRV criterion are human-gated

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols, including `encode_seq` (returns `bytes`) and `decode_seq` (returns
    `list[str]`), with `.pyi` stubs and `__all__`. abi3-py310 wheels for 4 platforms; 12 `py.detach`
    sites; 0.7.0 on PyPI
- `tests/test_seq.py` has 29 test functions, which meet the `rust-core.md` Python seq differential
    criterion
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

- Pure Go, 33 Tier 1 symbols, 182 `func Test`, `CGO_ENABLED=0`; IDv1 at the reference API shape
- `decodeBase32` re-encodes and rejects any non-canonical input, so spare-bit aliases, padding and
    interior `\r`/`\n` all fail (3 new strict tests). The spare-bit issue is closed
- **Missing (`normal` issue):** an `isccClean` helper with the Python strip set, U+001C–U+001F
    included. Go now also rejects a trailing-newline code that the reference strips; the issue
    tracks that too
- **Missing (`normal` issue):** the non-ASCII case-fold alias, shared with Rust
- **Missing:** Go `EncodeSeq` / `DecodeSeq`, a seq-vector loader, and seq test cases

## Java, Ruby, .NET, C++, Swift, Kotlin Bindings

**Status**: partially met

- All six expose the 33-symbol surface and shipped 0.7.0: Java and Kotlin on Maven Central, Ruby on
    RubyGems, .NET on NuGet, C++ through vcpkg/Conan metadata, Swift as an SPM XCFramework
- **Missing on all six:** `encode_seq` / `decode_seq` and the seq vectors in each conformance
    harness. Swift and Kotlin hard-code per-section counts. The .NET work depends on the C FFI step
- Ruby has two open `normal` issues: `json` in `Gemfile.lock` (Dependabot) and the `File.read` call
    in `test_conformance.rb` that passes no encoding
- Swift did not run on `5abda9f` (runner cancel), so the tip has no Swift result

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

- `ci.yml` has 20 jobs; macOS jobs run on `macos-15`. `release.yml` is `workflow_dispatch`-only with
    8 registry toggles. 21 version-sync targets are at 0.7.0
- `deny.toml` ignores `RUSTSEC-2026-0374` with a documented reason (no upstream fix)
- **Latest CI: FAILING** on `5abda9f`: https://github.com/iscc/iscc-lib/actions/runs/44321640827.
    The Swift job was `cancelled` with the annotation "The job was not acquired by Runner of type
    hosted" (macOS arm64 capacity). No steps ran and the code did not fail; the other 20 check-runs
    passed. The last fully green run is on `3e1f8e5`
- **Missing:** the `uv.lock` Dependabot bump (`soupsieve`, `setuptools`), tracked as a `normal`
    issue

## Open Issues

15 open: 0 critical, **8 normal**, 7 low. All 8 `normal` issues can be worked now: the c-ffi-api
type names, the Go `iscc_clean` port, base32 non-ASCII case-fold aliases (Rust + Go), text-utils
U+001C–U+001F trimming, wasm npm metadata, the Ruby `json` alert, the Ruby locale fix, and the
`uv.lock` alerts. The Go spare-bit alias issue is closed.

## Next Milestone

Get CI green on the `develop` tip first; the only red job is the runner-cancelled Swift job. Then
continue v0.8.0 ISCC-SEQ: `encode_seq` / `decode_seq` on the remaining 10 surfaces, with every
harness running the seq vectors, and clear the 8 open `normal` issues that block the release.
