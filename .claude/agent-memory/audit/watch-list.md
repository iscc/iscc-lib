---
name: audit-watch-list
description: Verified audit findings not filed yet (5-per-pass cap or ownership), first candidates for the next audit pass
metadata:
  type: project
---

All items verified true in the iter-270 sweep (HEAD 34303a6). Re-check against the tree before
filing; drop any a commit has fixed.

**Why:** each survived a skeptic pass; only the 5-issue cap or ownership kept them out. **How to
apply:** hand every item to the verify phase next pass; file the top ones first. target.md/decisions
items are `low` + `**Triage:** untriaged` (human-owned).

Code (CID-fixable), ranked:

- WASM hasher `finalize()`: bits rejected by `opt_u32` (-1, 1.5) leave the hasher usable, bits
    rejected by the core (13) consume it (`iscc-wasm/src/lib.rs:703-718`, 758, 817); wasm.md:338
    says always consumed; Go consumes on any first Finalize.
- Data-edge harness duplication: `iscc-lib/tests/test_data_edge.rs` vs
    `iscc-wasm/tests/   data_edge.rs` (10 byte-identical helpers, drivers drifted); Python schema
    rules twice (`tests/test_data_edge.py` vs `scripts/gen_data_edge_vectors.py`: SURFACE_IDS,
    output keys).
- npm parity: napi `Vec<f64>` rejects typed arrays/ArrayLike that @iscc/wasm accepts; JS range
    errors print Rust f64 Display (`inf`) and tests pin it.
- alg_cdc_chunks defaults: .NET alone makes utf32 optional, C++ alone has no avg_chunk_size default.
- Ruby gen_instance_code_v0 doc: bits "accepted for API consistency" but changes output.
- JNI encode_component/algCdcChunks hand-write negative checks instead of `non_negative`.
- Vacuous tests: WASM `test_iscc_decode_has_all_fields` (no asserts); napi alg_cdc_chunks "respects
    utf32 mode"/"accepts custom avg_chunk_size".
- .NET null string -> IsccException, null array -> ArgumentNullException.
- Go: MetaTrimMeta unused / no meta size limit; GenMetaCodeV0 three meta passes + duplicated tail;
    one-shot Data/Instance handle Push errors that cannot occur.
- WASM scalar params pass the name twice (`number()` and `to_*()`); `tests/unit.rs` 1604-line
    catch-all whose header says non-gen only.
- Complexity: core gen_iscc_code_v0, iscc_decompose, gen_meta_code_v0.
- Duplication: error-mapping closures (py/wasm/napi); DataHasher/InstanceHasher wrapper pairs in
    FFI/.NET/C++; Python stream-read loop x5; soft_hash_meta_v0 vs meta_name_simhash; bits_to_bytes
    twice; iscc_decode re-decodes base32.
- Dead: unused `serde` in iscc-wasm and iscc-lib dev-deps; `pytest-cov` dev dep unused.
- Gate latency: `tests/test_gil.py` text test is ~6 s of ~23 s pytest test time (16 threads x 215k
    chars on a debug build).

Docs (CID-fixable):

- Ruby how-to Error Handling names only RuntimeError (RangeError/TypeError raised too).
- Getting-started Go streaming example ignores Push/read errors; says Go 1.21+ (go.mod 1.26.1).
- `_lowlevel.pyi` docstrings (encode_component ranges, meta trim units).
- docs/development.md hook/mise/tree/tools tables stale; README omits Swift; registry READMEs omit
    Tier 1 functions.

Context drift (CID-fixable; consider one combined entry once the open spec entry is resolved):

- Root CLAUDE.md: `iscc_lib::api` module does not exist (streaming module omitted); reference map
    names a missing file; OIDC list omits npm; says mise pins tools.
- Specs: nodejs-bindings bridge sketch (camelCase, u32, objects); go-bindings API table/structs;
    rust-core signatures (alg_cdc_chunks, alg_simhash, soft_hash_video_v0, json_to_data_url);
    ci-cd.md + ruby-bindings.md describe nonexistent hooks; edge-vectors.md says recover the
    generator from git (it exists); java-bindings says JUnit 5 (6); 8 specs pin stale CI snippets.
- `crates/iscc-wasm/CLAUDE.md`: conventions no longer match lib.rs, flat-file threshold exceeded.
- Human-owned: archived decision "JS-number IDv1 surfaces take f64" contradicted by wasm `JsValue`.

Loop tooling (meta-improve/human, not the CID queue):

- advance.md step 6 never runs ty/clippy (pre-push only); review fixed advance's ty failure in 270.
- metrics.py gate timings measure other commands than pre-push, one warm sample (noise); counts
    gitignored/generated files; TODO/skip counts false positives.
- release.yml build/test jobs run only on real publish. Review ~47% of iteration cost.
