---
name: data-edge
description: data-edge.json harness locations, section rules, generator/header, mutation and throwaway-vector proofs
metadata:
  type: project
---

# Data-edge vectors (iters 215-253)

- Edge vectors (215): `crates/iscc-lib/tests/data-edge.json` (spec `specs/edge-vectors.md`), no
    `_metadata`; Rust `tests/test_data_edge.rs` (runtime read, excluded from package; CRAP-clean
    caller table, see ci-gates.md), Python `tests/test_data_edge.py` (schema + lib + iscc_core
    iff-`diverges` + SEED_SET; raw call apart from converter, rejections on raw call only).
    `iscc_decode` output = `[mt, st, vs, li, "bytes:<hex>"]`. Group 1 harnesses DONE (217): Go
    `data_edge_test.go`, Node `__tests__/data_edge.test.mjs`, WASM `tests/data_edge.rs`, JNI
    `DataEdgeTest.java`, .NET `DataEdgeTests.cs` (vendored), Ruby `test/test_data_edge.rb`. Group 2
    (218): Swift/Kotlin `DataEdge*`. Group 3 (219): C/C++ read generated
    `crates/iscc-ffi/tests/data_edge_vectors.h` (`scripts/gen_data_edge_vectors.py`, rows carry
    resolved c/cpp expectation; gate `tests/test_gen_data_edge_vectors.py`). `mise run check` =
    pre-commit only — run `prek run --hook-stage pre-push --files …` too. Sections (226): 6 incl.
    `alg_simhash` — first section with `bytes:<hex>` INPUTS: every harness has a per-section input
    preparer (Py `PREPARERS`, Node/Ruby 3rd tuple slot) applied OUTSIDE the raw call; generator
    `byte_list_arrays` (binary items, explicit lens). fixture =
    `json.dumps(indent=2,   ensure_ascii=False)`. Mutation-check by sed'ing an expected output in
    the copy a harness reads
- Sections (252): 8 incl. `gen_meta_code_v0` (meta OBJECT input; harnesses serialize it, C/C++ get
    `json.dumps(ensure_ascii=False)` text) + `gen_video_code_v0` (C struct `iscc_edge_frames`,
    `uintptr_t` lens). napi/WASM/JNI/.NET meta return bare iscc → `UNCOMPARED` name/meta/metahash.
    No cmake in container: `uvx cmake`. Swift toolchain persists in `/tmp/swifttc`. Reusable
    mutation-check = a `/tmp` script that edits expected values in the copy each harness reads. Rust
    harness `call_meta` sits at cc 5 (CRAP 30.0 = limit)
- Throwaway-vector proof (253): write vectors with iscc-core 1.5.0 into all 5 copies, regen header,
    run 12 surfaces, then `git checkout --` those 6 paths only. .NET local run needs
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`
    (`packages/dotnet` alone = MSB1003). iscc-core 1.5.0 has no `json_to_data_url`
- Output subset (236): gen `outputs` keys ⊆ `data.json` per-function keys, `iscc` required;
    validator `validate_vector(section, name, vector)` + generator `allowed_output_keys` derive it
    from data.json (gen = `gen_` prefix). Harnesses fix uncompared keys statically per result type
    (237: .NET/Java/napi/WASM `UNCOMPARED` map keyed by section; `compare(section, …)` walks
    expected)
- Swift scalars (232): corelibs AND Darwin `NSNumber` casts are loose (`false as? Int`=0,
    `0 as? Bool` =false). Data-edge harness classifies via `DataEdge.scalar` (re-serialise `[v]`
    after `isValidJSONObject`); synthetic Swift docs → `parsedDoc(#"json"#)`, never literals
- Sections (264): 9 incl. `alg_cdc_chunks` (`["bytes:<hex>", bool, int]` → list of `bytes:`).
    **Fixture is HAND-FORMATTED** (compact int rows in video, a `\u2028` escape in meta): never
    `json.dumps` the whole file. Splice the new section as text before the final `\n}\n`
    (`json.dumps({sec: ...}, indent=2)[1:-2]`) and write the same bytes to all 5 copies. C struct
    takes `uint32_t` for u32 inputs (literal `NNNu`); error rows zero a list as `{0, 0, 0}`.
    iscc-core builds differ for CDC avg >= 2^29 (compiled wheel py3.10-3.14 vs pure 3.15 sdist):
    check every vector with `uv run --no-project --python <3.10|3.15> --with iscc-core==1.5.0`
