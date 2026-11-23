---
name: conformance-runners
description: Rust conformance.rs and Go conformance.go data.json runners — structure, parsers, wrong-type rule, count proofs
metadata:
  type: project
---

# Conformance runners (data.json selftests)

- Conformance selftest (276): ONE runner per language — Rust `run_section` + generic `report_case`
    over `Case<T> = Option<(IsccResult<T>, T)>` (section logic in closures: not CRAP-counted); Go
    `runSection[V any]` + `run<X>Case(tc) (got, want, err)`, `errUnparsable`, `unmarshalInputs`. A
    new section = one closure/case fn. rustfmt reflow shifts lines → regen CRAP baseline AFTER check
- Rust data.json (291): only `conformance.rs` parses it — `pub(crate)` `test_data()`, `cases()`,
    `expected_iscc`, `<x>_inputs(tc) -> Option<…>` (`.get(i)?`, never `inputs[i]`); in-crate tests
    call `conformance::<x>_inputs(tc).unwrap_or_else(|| panic!(..))`. New CRAP fns = "new", not
    regressions → baseline untouched. Count-proof: diff sorted `--lib -- --list` vs archive copy
- Go data.json (290): ONE parse (`vectorEntry{Inputs, Outputs json.RawMessage}`, all sections);
    per-section input parsers (`metaInputs`, `streamInputs`, …) + `expectedISCC` → `(string, error)`
    shared by runners and `code_*_test.go` via `conformanceSection(t, name)` (sync.OnceValues). Bare
    hex / missing `iscc` = `errUnparsable` (Rust policy). Count-proof: diff sorted
    `go test -v | grep -- '--- PASS'` vs a `git archive HEAD` copy in /tmp
- Wrong-type rule (293, decisions 2026-11-22): both runners reject wrong type / fraction / out of
    range as unparsable. Rust `try_from` + strict `string_entries`, `check_sections`; Go
    `containsNull` token scan in `unmarshalInputs` (meta `*json.RawMessage` exempt), typed
    `uint32`/`int32` targets, pixels via `[]int` (never `[]uint8`: base64). Teeth proof = new tests
    on HEAD code in a throwaway worktree; never print 1024-entry inputs in failure msgs (`%.60s`)
- Rust section table (306): `SECTIONS: &[(&str, CaseCheck)]` in conformance.rs is the ONLY section
    list (meta/text rows `#[cfg]`-gated); `<x>_case(tc) -> Option<Verdict>` per section, generic
    `judge`. Tests use `check_of(section)` / `run(data, section)`; `test_sections_match_data_json`
    (meta-code) = set + len guard. lib.rs keeps only meta/text/mixed/instance extra-field loops.
    `iscc_inputs` stays `pub(crate)` (codec.rs tests). Selftest loop raised CRAP 4.1→5.6: baseline
    moved
- WASM conformance (306): strict readers shared with data_edge in `tests/common/mod.rs` (only the
    readers BOTH files use → no dead_code allow); file-local readers stay local
