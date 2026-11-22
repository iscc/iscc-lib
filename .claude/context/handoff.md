## 2026-11-22 — One section table drives Rust conformance; WASM conformance uses strict shared readers

**Done:** `conformance.rs` has one `SECTIONS: &[(&str, CaseCheck)]` table (meta/text rows
cfg-gated). `conformance_selftest` runs every row without stopping at a failure. Each `run_*_tests`
closure became a named `<x>_case(tc) -> Option<Verdict>` check, so a section name appears only in
its table row. `probe()` and the failure-arm tests look the check up in the table (`check_of`). A
`meta-code` test compares the table names with the `data.json` keys as sets, with a length check.
The five duplicate gen loops left `lib.rs`. The WASM conformance tests read vectors through strict
readers (shared ones in `tests/common/mod.rs`), with no casts and counts taken from `data.json`.

**Files changed:**

- `crates/iscc-lib/src/conformance.rs`: `SECTIONS` table, `CaseCheck`/`Verdict` types, generic
    `judge`, eleven `<x>_case` fns. Deleted `run_*_tests` and `Case<T>`.
    `audio_inputs`/`video_inputs` are now private (`iscc_inputs` stays `pub(crate)`, because
    codec.rs tests call it). Tests: added `check_of`/`run` helpers and
    `test_sections_match_data_json`; deleted `test_gen_sections_run`; `test_seq_sections_run` →
    `test_seq_section_counts` (count asserts only)
- `crates/iscc-lib/src/lib.rs` (tests only): deleted the image/audio/video/data/iscc `_conformance`
    tests
- `crates/iscc-wasm/tests/common/mod.rs` (new): `str_arg`, `str_list_arg`, `bytes_item`,
    `bytes_arg`, `frames_arg`, `u32_arg`, moved from `data_edge.rs`
- `crates/iscc-wasm/tests/data_edge.rs`: `mod common;` plus import; the moved readers are deleted
- `crates/iscc-wasm/tests/conformance.rs`: rewritten as `check_gen_section(section, caller)` plus
    nine `call_*` fns. Readers used only here: `opt_meta_arg` (null → None), `pixels_arg` (serde
    `Vec<u8>`), `i32_list_arg`, `stream_arg`. Asserts `tested == cases.len()`, and `section_cases`
    fails an empty section
- `.crap-baseline.json`: refreshed (see Verification)
- Docs: `crates/iscc-lib/CLAUDE.md` and `crates/iscc-wasm/CLAUDE.md` described the per-gen test
    loops and the hard-coded counts. Both now describe the table and the strict readers

**Verification:**

- `cargo test -p iscc-lib`: 468 passed / 0 failed. `--no-default-features`: 397/0.
    `--no-default-features --features text-processing`: 432/0
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` is clean, and so is
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` (also `--tests` under none and
    under text-processing)
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    exits 0: conformance 11 passed, data_edge 8, unit 136, no warnings in the log
- Section grep: lines 39–50 (inside `SECTIONS`) are the only matches before `mod tests` (line 403)
- `fn test_gen_(…)_conformance|fn test_gen_sections_run` grep: empty.
    `as (u8|i32|u32)\b|tested, [0-9]` grep on the WASM conformance.rs: empty
- Teeth: with the `("gen_audio_code_v0", audio_case)` row deleted, `test_sections_match_data_json`
    FAILED ("table rows differ from data.json sections"; right side lists `gen_audio_code_v0`). A
    duplicated `decode_seq` row FAILED ("a section has two table rows"). The file was restored from
    a /tmp copy both times, and `cmp` reported it identical
- CRAP gate: before the refresh, `cargo crap --baseline … --fail-regression --fail-above` exited 1:
    `conformance_selftest` went 4.08 → 5.58 (cc 5, 71.4%; the two early-return error arms are
    uncovered and the loop adds a decision). Ran `mise run crap:baseline` after `mise run check`;
    the gate now exits 0 (0 regressed, 191 unchanged). A baseline diff by score shows changes only
    in `conformance.rs`: 12 new `*_case`/`judge` (max 7.0, `decode_seq_case`), 11 `run_*_tests`
    removed
- `mise run check`: exit 0. The first run reformatted the new files (rustfmt, mdformat); the second
    run was clean

**Next:** the spec/API-doc drift `[audit]` issue (9 files, needs an audit-scope or a specs/docs
split).

**Notes:**

- next.md expected the table loop to lower `conformance_selftest`'s CRAP score. It rose, because the
    old body was straight-line `&=` calls. The baseline moved in this step, as the gate rule
    requires.
- `probe`'s parse flag is `check(&tc).is_some()`. That also requires `outputs.iscc` to parse, which
    every probe row supplies, so it matches the old `<x>_inputs().is_some()`. No second table was
    needed.
- The WASM strictness relies on serde (`Vec<u8>`, `Vec<i32>`, `Vec<Vec<i32>>`) and `u32::try_from`.
    No wasm-side negative test was added, because next.md asked for none. Not probed with a bad
    vector.
