# Next Work Package

## Step: One section table drives Rust conformance; WASM conformance uses strict shared readers

## Goal

Picks up the `[audit]` issue "Rust conformance runs each `data.json` section up to three times and
lists the sections by hand in three places". One table in `conformance.rs` becomes the only section
list, so a section without a runner fails a test. The WASM conformance tests read vectors through
the strict readers the data-edge harness already uses, with no silent casts.

## Alternatives Considered

- **Chosen:** the conformance runner issue. The handoff names it first, it touches 2 non-test files,
    and the test-only WASM part does not count toward the cap.
- **Rejected:** the spec/API-doc drift issue. It is 9 files and needs an audit-scope or a specs/docs
    split. It blocks v0.8.0 too, so it comes next, with nothing lost by waiting.

## Scope

- **Create**: `crates/iscc-wasm/tests/common/mod.rs` (test file)
- **Modify**: `crates/iscc-lib/src/conformance.rs`, `crates/iscc-lib/src/lib.rs` (test module only);
    test files `crates/iscc-wasm/tests/conformance.rs` and `crates/iscc-wasm/tests/data_edge.rs`
- **Reference**: `crates/iscc-lib/tests/data.json`, decision 2026-11-22 "A wrong-type or
    out-of-range vector value makes a conformance case unparsable"

Rule invoked: default (2 non-test files; the 3 WASM test files are tests).

## Not In Scope

- The meta/text/mixed/instance `*_conformance` loops in `lib.rs`. They check extra output fields, so
    keep them, hard-coded counts included.
- Any change to what `conformance_selftest` checks or reports, or to its public signature. It is a
    Tier 1 symbol, and a signature change is an `API-BREAK` stop.
- The data-edge runners of other surfaces, `crates/iscc-lib/tests/*.rs`, and other bindings'
    conformance harnesses.
- The spec/doc drift issue.

## Implementation Notes

**`conformance.rs`:**

- Add one `const` table of sections, e.g. `&[(&str, fn(&Value) -> bool)]`, or a small struct if
    `probe` also needs a parse check per section. Put `#[cfg(feature = "meta-code")]` /
    `#[cfg(feature = "text-processing")]` on the meta/text elements; cfg on array-literal elements
    works on stable. Give each runner its section name in one place only: the table entry or the
    runner, not both. Today every `run_*_tests` repeats the string.
- `conformance_selftest` keeps `test_data()` + `check_sections()`, then runs every table entry. Do
    not stop at the first failure: every runner must still run and print its failures.
- `probe()` (test module) looks sections up in the same table, not in its own `match`. Its `parsed`
    flag needs a per-section parse check: either a third field in the table, or a small test-only
    table of input parsers next to `probe`. Pick whichever keeps the table the only list of section
    names. Today `probe` has no instance entry; it does not need one.
- Delete `test_gen_sections_run`. In `test_seq_sections_run`, keep the two count asserts or remove
    the test; `test_conformance_selftest_passes` already runs the seq runners.
- Add a test under `#[cfg(feature = "meta-code")]` (meta-code enables text-processing): the table's
    section names equal the `data.json` keys minus `_metadata`, compared as sets, with a length
    check so a duplicate row fails. This is the guard the issue says is missing.
- Keep the existing failure-arm tests (`test_gen_section_failure_arms_fail` etc.) passing. They call
    runners directly, and that is fine.

**`lib.rs` test module:** delete `test_gen_image_code_v0_conformance`, `..._audio_...`,
`..._video_...`, `..._data_...` and `..._iscc_code_v0_conformance`. Keep the other tests that read
`conformance::cases` (e.g. `test_soft_hash_image_v0_256_quadrant_layout`). Any `conformance::`
helper left without a caller can lose its `pub(crate)` or be deleted, as long as clippy stays clean
in all 3 CI feature sets.

**WASM tests:**

- Move the strict readers in `data_edge.rs` (`str_arg`, `str_list_arg`, `bytes_item`, `u32_arg`,
    `frames_arg`, `u32_list_arg`, …) into `tests/common/mod.rs`. Declare `mod common;` in both
    files, and use the readers in both. If one file leaves a helper unused, rustc warns and CI
    clippy runs `--all-targets -- -D warnings` on the host, so handle that: put
    `#[allow(dead_code)]` on the `mod common;` declaration, or keep the helper in the file that uses
    it.
- `conformance.rs` needs readers that `data_edge.rs` lacks: a `stream:<hex>` reader (an empty hex is
    valid), a `u8` pixel list (from `serde_json::from_value::<Vec<u8>>`, or `u8::try_from(as_u64)`),
    and a meta reader that maps `null` to `None`. The data-edge `meta_arg` rejects `null` on
    purpose, so keep that one as it is. Remove every `as u8` / `as i32` / `as u32` cast on vector
    values. A value out of range must fail the test, not be truncated.
- Replace each hard-coded `assert_eq!(tested, N)` with the section's case count read from
    `data.json`, plus a floor (`> 0`) so an empty section fails.
- `cargo test -p iscc-wasm` runs none of these tests. Only `wasm-pack test --node` does (in `$PATH`
    here).

**Gates:** `conformance.rs` is a non-test file in a crate the CI-only CRAP `--fail-regression` gate
measures. A table loop should lower `conformance_selftest`'s score. If any function scores higher,
move the baseline in this step (`mise run coverage && mise run crap:baseline`). No hot path is
touched, so the iai gate does not apply.

## Verification

- `cargo test -p iscc-lib` and `cargo test -p iscc-lib --no-default-features` pass, and so does
    `cargo test -p iscc-lib --no-default-features --features text-processing`
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` and
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` are clean
- `RUSTFLAGS="-C target-feature=+simd128" wasm-pack test --node crates/iscc-wasm --features conformance`
    passes
- `git grep -nE '"gen_(image|audio|video|data|iscc)_code_v0"|"gen_(meta|text|mixed|instance)_code_v0"|"(en|de)code_seq"' crates/iscc-lib/src/conformance.rs`
    shows each section name once outside the test module (inside the table)
- `git grep -nE 'fn test_gen_(image|audio|video|data|iscc)_code_v0_conformance|fn test_gen_sections_run' crates/iscc-lib/src`
    prints nothing, and
    `git grep -nE 'as (u8|i32|u32)\b|tested, [0-9]' crates/iscc-wasm/tests/conformance.rs` prints
    nothing
- The table-vs-`data.json` test fails if one table row is removed (shown once by the advance agent
    and the result reported in the handoff; the tree stays unchanged)

## Done When

All six verification criteria pass, and `conformance.rs` holds the only list of `data.json` sections
in the Rust core.
