<!-- assessed-at: 5aa8414b3b285ac738b5490eb9147eafa9477bbf -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features complete; six `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 251 made Go's
canonical `meta` JSON emit `&`, `<`, `>`, U+2028/9 raw, matching the core; that issue is now reduced
to its shared vector, like the ragged-video one. Six `normal` issues remain open, none blocked on
the human or upstream. CI is green on the pushed tip `837e884`.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met

- `crates/iscc-lib/tests/data-edge.json` has 6 sections; harnesses on all 12 surfaces; the 5 tracked
    copies are byte-identical and match `VENDORED_COPIES`
- **Missing (collecting rule, 2 `normal` issues):** no `gen_meta_code_v0` vector with raw `&`, `<`,
    `>`, U+2028/9 in `meta`, and no `gen_video_code_v0` ragged-`frame_sigs` error vector (needs a
    `diverges` marker, since iscc-core truncates and returns a digest)
- Not filed: the `parts` entry is untested on any surface; the WASM `compare` has no explicit
    non-object branch

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 308 lib tests
- `soft_hash_video_v0` / `gen_video_code_v0` reject ragged input via `check_frame_sig_lengths`
    (`lib.rs:726`); no binding test pins it until the shared vector lands
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification (both `low`)

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- Integer-range issue: raises `OverflowError` where `docs/howto/python.md` documents `ValueError`
- Python 3.10 floor is past end of life — `low` issue, human-only decision

## Node.js Bindings

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests green;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- **Bug (`normal`):** `u32`/`i32` parameters wrap silently on out-of-range input (e.g. bits
    `2**32+64` yields a 64-bit code); only `gen_iscc_id_v1` runs `checked()`
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests green; SIMD release build met
- **Bug (`normal`):** same silent integer wrap as napi
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — needs a v0.8.0 release-notes line

## C FFI

**Status**: partially met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- **Bug (`normal`):** `(NULL, 0)` is rejected by some exports and accepted by others (tests pin
    both); pointer-array loops pass inner pointers to `from_raw_parts` unchecked (UB on NULL)

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections strictly; 0.7.0 on Maven Central
- Integer-range issue: JNI casts `bits as u32` unchecked; `low`: pending-exception guard untested

## Go Bindings

**Status**: met (v0.8.0 scope, pending the shared vectors)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green
- `parseMetaJSON` (`code_meta.go:242`) encodes with `SetEscapeHTML(false)` and writes U+2028/9 raw;
    no `json.Marshal(` left in the file; 4 escaping tests pin it — published 0.7.0 still escapes
- Ragged frame signatures rejected (`code_content_video.go:43`) with 6 named tests
- `low` (new): Go canonical `meta` differs from the core on UTF-16 key order, `-0` and lone
    surrogates (contrived input only)

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 6 sections strictly; 0.7.0 on NuGet
- .NET 8 floor past end of life — `low`, human-only

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job green;
    0.7.0 vcpkg/Conan
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq`; `test_data_edge.rb` runs all 6 sections strictly; 0.7.0 on RubyGems
- Integer-range issue: raises `RangeError` where `docs/ruby-api.md` documents `RuntimeError`
- `low`: source gems up to 0.7.0 do not build; Ruby 3.1 floor EOL; Float accepted for integers

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors and all 6 data-edge sections; green in CI, shipped 0.7.0

## Documentation

**Status**: partially met

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- **Bug (`normal`):** published docs contradict what ships — README prints `result.iscc` from a
    string return, WASM snippet lacks `await init()`, Java/Kotlin/Swift docs deny shipped
    distribution, `architecture.md` and `development.md` carry stale claims (~11 files)
- `low`: Sum-Code comments show `KAA`; pkg-config example says `Version: 0.2.0`; `ci-cd.md`
    describes a tag trigger `release.yml` does not have

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 21 jobs (job table gated); `release.yml` is `workflow_dispatch`-only with 8 registry
    toggles; 21 version-sync targets at 0.7.0
- `normal` (audit): the pytest gate rose 25.6 → 44.1 s; `tests/test_benchmarks.py` runs timing loops
    every gate and the Tier 1 doc tests copy the docs tree per case
- **Latest CI: PASSING** on `origin/develop` = `837e884` (Go escaping fix):
    https://github.com/iscc/iscc-lib/actions/runs/44882682839
- HEAD is ahead of origin only by the `cid(log)` commit; green CI covers all code at HEAD

## Open Issues

20 open: 0 critical, 6 normal (2 `[human]`, 4 `[audit]`, all CID-workable), 14 low (human-directed;
9 `untriaged`). issues.md is at its 300-line budget.

## Next Milestone

Close the six `normal` issues that block v0.8.0: the two missing shared `data-edge.json` vectors
(raw JCS characters in `meta`, ragged video) on every surface, the FFI NULL handling, the
integer-range bugs, the published-docs corrections and the pytest gate time. The release itself
stays with Titusz.
