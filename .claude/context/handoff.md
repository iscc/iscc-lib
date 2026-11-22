## 2026-11-22 — Rust in-crate tests read `data.json` inputs through `conformance.rs` parsers

**Done:** `conformance.rs` now has `pub(crate)` items: `test_data()`, `cases(data, section)`,
`expected_iscc`, and one inputs parser per gen section (`meta_inputs` → `MetaInputs` struct,
`text_inputs`, `image_inputs`, `audio_inputs`, `video_inputs`, `mixed_inputs`, `stream_inputs`,
`iscc_inputs`). `run_section` and every `run_*_tests` use them, and so do all 13 hand-parsing test
sites in `lib.rs`, `codec.rs` and `streaming.rs`. Every positional read uses `.get(i)?`/`.first()?`,
so a short inputs list is now a parse failure instead of a panic.

**Files changed:**

- `crates/iscc-lib/src/conformance.rs`: parsers split out, plus small private helpers (`inputs`,
    `bits_at`, `i32_entries`, `meta_arg`); runners are now parser + call. Tests: two data short-list
    arms (`[]`, `["stream:"]`) added to `test_gen_section_failure_arms_fail`. New
    `test_gen_section_short_inputs_fail` covers meta `["Name", "", null]` (fails at the `bits` read)
    and iscc `[]`, one case per call; each also asserts the parser returns `None`
- `crates/iscc-lib/src/lib.rs` (tests only): 10 conformance tests now get inputs from the parsers,
    including the single-vector `test_soft_hash_image_v0_256_quadrant_layout`. Every extra-field
    assertion (metahash, name, description, meta presence, characters, parts, datahash, filesize)
    and every `tested == N` count is unchanged
- `crates/iscc-lib/src/codec.rs` (tests only): decompose round-trip uses `iscc_inputs` and
    `expected_iscc`
- `crates/iscc-lib/src/streaming.rs` (tests only): Instance and Data hasher conformance tests use
    `stream_inputs`

**Verification:**

- `cargo test -p iscc-lib`: all suites ok in all three CI feature sets (lib tests: 319 default, 266
    `--no-default-features`, 287 `--features text-processing`)
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean. `--tests` and lib-only clippy
    are also clean under `--no-default-features` with and without `text-processing`, so there is no
    dead parser in any cfg
- `grep -n 'data\.json' crates/iscc-lib/src/*.rs | grep -v conformance.rs` prints nothing
- `grep -nE 'inputs\[[0-9]' crates/iscc-lib/src/*.rs` prints one line: `lib.rs:1270`
    `json_to_data_url(inputs[2])`. That is a local `[&str; 3]` array in
    `test_json_to_data_url_matches_gen_meta_code_v0_meta`, not a fixture read, and cannot panic. I
    left it alone because it is outside this step's scope; the grep over-matches the name
- `cargo test -p iscc-lib --lib conformance -- --nocapture`: 22 passed. The data runner prints four
    `could not parse test inputs` lines (`00ff`, no `iscc`, `[]`, `["stream:"]`) plus the mismatch
    and error lines. The new test prints one line each for `gen_meta_code_v0.case` and
    `gen_iscc_code_v0.case`. No panic
- Proof that no test was lost: the sorted `cargo test --lib -- --list` output from a
    `git archive   HEAD` copy compared with the working tree is 318 → 319 lines. The only difference
    is `conformance::tests::test_gen_section_short_inputs_fail`
- CRAP: ran `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info`, then CI's gate line
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`. It
    exits 0 with "0 regressed, 2 improved, 16 new, 48 moved". Highest new score: `meta_inputs` 8.0.
    **Case: no regression, so `.crap-baseline.json` is untouched**
- `mise run check` exits 0

**Next:** part 2 of the `[audit]` issue: one wrong-type/range rule on both runners. In Rust it now
touches only the parsers: `bits_at` (`as u32` truncation, `64.0`), `meta_arg` (number or array →
`None`), `string_entries` (drops non-strings), and the `as u8`/`as i32` casts in
`image_inputs`/`i32_entries`.

**Notes:**

- Runner verdicts are unchanged except for short lists. Two of the in-crate tests now inherit looser
    readings, as next.md intended. Mixed and iscc code lists used to panic on a non-string entry and
    now drop it via `string_entries`. The meta test's panic on an unknown meta type is now the
    runner's `None`. Neither case occurs in the real fixture
- The CRAP output lists `parse_meta_json` under "Removed since baseline". That comes from iter 289,
    not this step. The baseline still has the stale row; it does not fail the gate
- The `lcov.info` produced by the coverage run is gitignored (not in `git status`)
- Budget: 4 source files changed, all within the declared scope. No public API change, no hot path
    touched (so no bench was needed)
