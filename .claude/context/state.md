<!-- assessed-at: 70bd975f1d944d96423de35b67e3737b02c65961 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 4 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 234 fixed the WASM `alg_cdc_chunks` return type (issue closed). Titusz
then added a "dependencies are secure" criterion to `target.md` and filed two `normal` issues (a
dependency/runtime security pass, and partial-key compares in 4 data-edge harnesses), so 4 `normal`
issues now block v0.8.0. CI is green on the pushed tip.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met — 1 `normal` `[human]` issue

- `crates/iscc-lib/tests/data-edge.json` has 6 sections: `iscc_decode`, `encode_seq`,
    `gen_text_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `alg_simhash`; harnesses on all 12
    surfaces; the 5 tracked copies are byte-identical and match `VENDORED_COPIES`
- **`normal`:** the .NET, Java, napi and WASM harnesses compare only the keys their result exposes,
    so expected keys like `characters` are dropped in harness code. This breaks the spec rule that
    every deviation is declared in the file. Ruby was made strict at 218; these four were not

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- `rust-core.md` Tier 1 table sits in a `tier1:begin/end` block that the doc gate checks against
    `lib.rs`
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green

## Node.js Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- `ci.yml` builds napi on Node `20`, which is end-of-life (part of the security-pass issue)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met
- `alg_cdc_chunks` returns `Vec<js_sys::Uint8Array>` (`.d.ts`: `Uint8Array[]`); issue closed at 234.
    The runtime shape of a published export changed — worth a v0.8.0 release-notes line

## C FFI

**Status**: met (v0.8.0 scope)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row (incl. `alg_simhash`) except the 2 marked `c: skip`

## Java Bindings

**Status**: met (v0.8.0 scope); 1 `normal` doc issue

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections (partial-key compare, see above),
    green in CI; 0.7.0 on Maven Central
- **`normal`:** `docs/howto/java.md` `genSumCodeV0` example passes 3 args (method takes 4) and calls
    accessors on `public final` fields; it does not compile
- `low` issue: the pending-exception guard has no test

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `data_edge_test.go` runs all sections, green in CI

## C# / .NET Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- `EncodeSeq` / `DecodeSeq`; `PinnedRows` pins all jagged-array inputs; `DataEdgeTests.cs` runs all
    6 sections, green in CI; 0.7.0 on NuGet
- `docs/howto/dotnet.md` "Error handling" doesn't list `ArgumentNullException` (no issue filed)

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run from the generated
    header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections
    strictly, green in CI; 0.7.0 on RubyGems
- CI tests on Ruby `3.1`, the supported floor, which is end-of-life — a floor only the human raises
- `low` issue: integer list elements accept a Float and truncate it (Titusz decides)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift 30 tests (19 conformance, 11 data-edge); Kotlin `DataEdgeTest` runs all 6
    sections; both green in CI and shipped 0.7.0

## Documentation

**Status**: partially met — 1 `normal` `[human]` issue (narrowed)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs
- Tier 1 doc gate `scripts/check_tier1_docs.py` (prek hook + 15 pytest tests) checks registered docs
    name every symbol derived from `lib.rs`
- **Remaining in the `[human]` issue:** the gate fails open on `lib.rs` forms it does not parse;
    `docs/howto/rust.md`, `docs/howto/c-cpp.md`, `docs/rust-api.md` (stale `IsccResult<String>`
    return types) and `docs/architecture.md` are unregistered and incomplete; hand-written counts
    and file lists remain in specs and per-crate `CLAUDE.md` files

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met — "dependencies are secure" criterion unverified (`normal` `[human]`)

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Missing:** the security criterion added to `target.md` at `70bd975`. Only Rust has an advisory
    gate (`cargo deny`); no other ecosystem is audited. Test runtimes include Node `20` (EOL); the
    `ci-cd.md` freshness table omits runtimes and lists a `mise.toml` `[tools]` row that doesn't
    exist. Dependabot (tracks `main`) shows 6 open alerts: `soupsieve` x4, `setuptools`, `json`
- **Latest CI: PASSING** on `origin/develop` = `7af33e9` (`github-actions` suite `success`, 21/21
    check-runs `success`), covering the WASM fix
- Unpushed: 5 commits; besides `.claude/`, they change `tools/cid.py` + `tests/test_cid.py` (loop
    infra), which no CI run has covered yet

## Open Issues

11 open: 0 critical, **4 normal**, 7 low. Normal: derived-fact drift `[human]`, Java how-to
`genSumCodeV0` example, data-edge partial-key compare `[human]`, dependency/runtime security pass
`[human]`. issues.md is at 230 of 300 lines.

## Next Milestone

Clear the 4 open `normal` issues — the remaining v0.8.0 release blockers: the dependency and runtime
security pass, strict data-edge compares on .NET/Java/napi/WASM, the broken Java how-to example, and
the remaining derived-fact drift.
