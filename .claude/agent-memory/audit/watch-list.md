---
name: audit-watch-list
description: Verified audit findings not filed yet (5-per-pass cap or ownership), first candidates for the next audit pass
metadata:
  type: project
---

Items marked (270) were re-verified at db1fe55 (iter 280); items marked (280) are first verified at
db1fe55. Re-check against the tree before filing; drop any a commit has fixed.

**Why:** each survived a skeptic pass; only the 5-issue cap or ownership kept them out. **How to
apply:** hand every item to the verify phase next pass; file the top ones first. target.md/decisions
items are `low` + `**Triage:** untriaged` (human-owned).

Code (CID-fixable), ranked:

- (270) Data-edge harness duplication: `iscc-lib/tests/test_data_edge.rs` vs
    `iscc-wasm/tests/data_edge.rs` (same helper set, drivers drifted); Python schema rules twice
    (`tests/test_data_edge.py` vs `scripts/gen_data_edge_vectors.py`).
- (280) napi/WASM: six identical narrowing helpers (`checked`, `to_u8/u32/i32`, `to_*_vec`; napi
    :21-65, wasm :100-143); decisions-archive.md:469-481 rejected sharing at "~5 lines".
- (280) C++ narrow int params (`iscc.hpp:266`, :592) wrap silently, against decisions.md:277-287
    rationale; no `-Wconversion` in packages/cpp.
- (280) Rust `conformance_selftest` "does not panic" doc (conformance.rs:30) vs direct indexing;
    Rust `n as u8` (:161) / Go `byte(v)`, `int32(v)` wrap bad vector values.
- (270) npm parity: napi `Vec<f64>` rejects typed arrays that @iscc/wasm accepts; Rust `inf`
    Display.
- (270) alg_cdc_chunks defaults: only .NET makes utf32 optional.
- (270) Ruby gen_instance_code_v0 doc: bits "accepted for API consistency" but changes output.
- (270) JNI encode_component/algCdcChunks hand-write checks instead of `non_negative`.
- (270/280) Tests that cannot fail: WASM `test_iscc_decode_has_all_fields`; napi alg_cdc_chunks
    utf32/avg; Rust "import path" tests (`test_text_utils.rs:144-157`); C++/.NET `alg_simhash([[]])`
    tests accept any outcome; JNI testConstants compares literals; uniffi text_trim 32-bit tests
    cannot fail on 64-bit hosts; Go codec_test.go:1105/1175 skip when monorepo data.json missing.
- (270) .NET null string -> IsccException, null array -> ArgumentNullException.
- (270) Go: GenMetaCodeV0 three meta passes + duplicated tail; unreachable Push errors;
    GenIsccCodeV0 129-line function (280); Go exports non-Tier-1 `AlgWtahash` (280).
- (270) WASM scalar params pass the name twice; `tests/unit.rs` 1604-line catch-all.
- (270) Complexity: core gen_iscc_code_v0, iscc_decompose, gen_meta_code_v0.
- (270/280) Duplication: error-mapping closures (py/wasm/napi); FFI Result->C mapping by hand (byte
    buffer / string array / code-pointer loop x3); JNI nested new_string/throw match x14; C++ vector
    overloads duplicate pointer overloads; JNI resource path built 4x; Python tests redefine
    data.json loaders (no conftest); Python video entry points x4; FFI/UniFFI hand-rolled hex
    decoders; Hasher wrapper pairs; Python stream-read loop x5; soft_hash_meta_v0 vs
    meta_name_simhash; bits_to_bytes twice; iscc_decode re-decodes base32.
- (270/280) Dead: unused `serde` (iscc-wasm, iscc-lib dev-deps); `pytest-cov`; csbindgen not in
    workspace.dependencies; `iscc_sum.c` example linked in docs, built by no job.
- (270) Gate latency: `tests/test_gil.py` 16 threads x 215k chars on a debug build; (280) pre-push
    compiles iscc-rb (CI excludes it) and rebuilds the Python extension every push.

Docs (CID-fixable):

- (280) vcpkg is filed; C++ README + root README pair the tarball install with an include path the
    tarball lacks (README.md:127-131 vs :244).
- (280) `cargo bench -p iscc-lib` docs also run the iai bench (needs valgrind).
- (280) Root README Implementors Guide points to iscc-core data.json for gen_sum_code_v0, omits
    Swift.
- (280) Go README states no Go version floor; LoaderProbe Javadoc above `package`.
- (270) Ruby how-to Error Handling names only RuntimeError.
- (270) Getting-started Go streaming ignores Push/read errors; says Go 1.21+.
- (270) `_lowlevel.pyi` encode_component says mtype 0-6 (core: rejects 5, accepts 7).
- (270) docs/development.md tables stale; registry READMEs omit Tier 1 functions.

Context drift (CID-fixable):

- (270) Root CLAUDE.md: reference map names missing `code_iscc.py`; OIDC list omits npm; hook
    inventory vs .pre-commit-config.yaml (also ci-cd.md:410-416, swift-bindings.md:363-367).
- (270) Specs: nodejs-bindings bridge sketch; go-bindings tables; rust-core signatures; ci-cd/ruby
    nonexistent hooks; edge-vectors generator; java JUnit 5; stale CI snippets.
- (270) `crates/iscc-wasm/CLAUDE.md`: 500-line flat threshold (lib.rs 840), Vec<String> row wrong.
- (280) Human-owned: target.md:36-44 v0.8.0 section spells out a finished rollout + dead issue ref.

Loop tooling (meta-improve/human, not the CID queue):

- (280) review-patterns.md agent memory grew 497 -> 1,088 lines in 3 days; topic files unbounded.
- (280) Untriaged `low` entries pile up (8 open); one-shot dedup `normal` deferred ~22 iterations;
    meta-improve proposal unanswered since iter 248.
- (270) advance.md step 6 never runs ty/clippy; metrics.py gate timings noisy single samples;
    release.yml jobs run only on real publish.
