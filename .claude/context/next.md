# Next Work Package

## Step: Rust in-crate tests read `data.json` inputs through `conformance.rs` parsers

## Goal

Part 1 of the `[audit]` issue "Rust tests re-parse `data.json`, and the two runners still disagree
on malformed vectors". After this step `conformance.rs` is the only file under `crates/iscc-lib/src`
that parses `data.json`. The short-inputs panic goes away along the way, so `conformance_selftest`
keeps its "Does not panic" promise.

## Alternatives Considered

- **Chosen:** Rust dedup part 1. It finishes the half-done issue (Go landed in 290), it is a pure
    test refactor plus one panic fix in a single crate, and it sets up part 2 (the shared wrong-type
    rule) to touch only the parsers.
- **Rejected:** the `version:check` gaps issue (1 file, also `normal`). It is a good next pick, but
    the handoff and the issue's own split favour finishing the runner dedup while 290's context is
    fresh. The WASM init-docs issue (5 files) also waits. `alg_minhash_256([])` and WASM
    `finalize()` need Titusz's decision. Go `EncodeComponent` needs a recorded rule plus a
    12-surface vector fan-out.

## Scope

- **Modify**: `crates/iscc-lib/src/conformance.rs` (split out the parsers),
    `crates/iscc-lib/src/lib.rs`, `crates/iscc-lib/src/codec.rs` and
    `crates/iscc-lib/src/streaming.rs` (test modules only), and `.crap-baseline.json` only if the
    CRAP gate needs it (generated).
- **Reference**: `packages/go/conformance.go` (the Go shape: one inputs parser per section, used by
    both the runner and the tests), `crates/iscc-lib/CLAUDE.md` → "Conformance Rules"

## Not In Scope

- Part 2 of the issue: one rule for a wrong JSON type or range (`bits` `64.0`, `null` description,
    `meta` number/array, non-string code-list entries, fractional pixels) on both runners. Keep the
    Rust runner's current verdict on every table row except "inputs list too short"
- Any change to `packages/go`, to `crates/iscc-lib/tests/*.rs` (integration tests are outside the
    issue's "under `src`" criterion), or to the public API
- Rewriting what the in-crate tests assert: keep every extra-field check (metahash, name,
    description, meta presence, characters, `parts`, the decompose round-trip, streaming
    equivalence, the `tested == N` counts)

## Implementation Notes

- 13 hand-parsing sites (`include_str!("../tests/data.json")`): 10 in `lib.rs` (meta, text, image,
    the single-vector `test_0003_img_256` DCT test near :1560, audio, video, mixed, data, instance,
    iscc), `codec.rs:1806` (decompose round-trip) and `streaming.rs:294/439` (Instance and Data
    hasher).
- Give `conformance.rs` `pub(crate)` items the tests call. A suggested shape, mirroring Go:
    - a fixture accessor (parse `TEST_DATA` once per call; `OnceLock` is optional)
    - `cases(data, section)`, which `run_section` uses too
    - one inputs parser per gen section returning `Option<…>`: `meta_inputs` (a small struct with
        name, `Option` description, `Option<String>` meta and bits), `text_inputs`, `image_inputs`,
        `audio_inputs`, `video_inputs`, `mixed_inputs`, `stream_inputs` (shared by data and instance),
        `iscc_inputs`, plus `expected_iscc`. Each `run_*_tests` becomes parser + call
- The private items stay invisible to `lib.rs`'s `mod tests` until they are made `pub(crate)`.
    Nothing new is re-exported at the crate root, so the Tier 1 doc gate must stay green.
- **Panic fix:** every positional read in the parsers uses `inputs.get(i)?`, never `inputs[i]`, so a
    short list is a parse failure (as in Go). Add one-bad-case-per-call failure arms for
    `"inputs": []` and `"inputs": ["stream:"]` to `test_gen_section_failure_arms_fail`. Add a
    short-list case for meta and for iscc (for example `"inputs": []`), so the `.get()` is pinned in
    more than one parser.
- In the tests, `parser(tc).unwrap_or_else(|| panic!("unparsable inputs in {tc_name}"))` replaces
    the hand decoding. The meta test's `panic!` on an unknown meta type goes away; the runner's
    current `None` reading applies until part 2.
- **CRAP gate (CI-only):** `conformance.rs` is scored. After the change run `mise run coverage`,
    then CI's gate line
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` `--fail-above`.
    If it reports a regression, run `mise run crap:baseline` and commit the refresh in this step.
    Say which case applied in the handoff. Neither the text hot path nor any benchmark is touched,
    so the iai gate does not apply.

## Verification

- `cargo test -p iscc-lib` passes, as do `--no-default-features` and
    `--no-default-features --features text-processing` (CI runs all three; the cfg gates must still
    compile)
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean
- `grep -n 'data\.json' crates/iscc-lib/src/*.rs | grep -v '^crates/iscc-lib/src/conformance.rs'`
    prints only comment or doc lines: no `include_str!`, no `serde_json::from_str` of the fixture
- `grep -nE 'inputs\[[0-9]' crates/iscc-lib/src/*.rs` prints nothing
- `cargo test -p iscc-lib conformance -- --nocapture` shows the short-inputs failure arms each
    printing a `could not parse test inputs` line and the test passing (no panic)
- The CI CRAP gate line above exits 0 against the committed `.crap-baseline.json`, and
    `mise run check` exits 0

## Done When

All six verification criteria pass, and the in-crate tests run every assertion they ran before, with
their inputs coming from `conformance.rs`.
