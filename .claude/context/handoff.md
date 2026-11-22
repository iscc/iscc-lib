## 2026-11-22 — Review of: Rust in-crate tests read `data.json` inputs through `conformance.rs` parsers

**Verdict:** PASS

**Summary:** `conformance.rs` is now the only file under `crates/iscc-lib/src` that parses
`data.json`. All 13 hand-parsing test sites in `lib.rs`, `codec.rs` and `streaming.rs` call its
`pub(crate)` parsers and keep every extra-field assertion and `tested == N` count. A short inputs
list is a parse failure instead of a panic, pinned in the data, meta and iscc parsers. The diff is
test modules plus private parser code: no public API change, no hot path, 4 source files.

**Verification:**

- [x] `cargo test -p iscc-lib` in all three CI feature sets — lib tests 319 / 266 / 287, every suite
    ok
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` — clean; workspace clippy and CI's
    `--no-default-features` and `--all-features` lines are clean too
- [x] `grep -n 'data\.json' crates/iscc-lib/src/*.rs` outside `conformance.rs` — prints nothing
- [x] `grep -nE 'inputs\[[0-9]' crates/iscc-lib/src/*.rs` — prints one line, `lib.rs:1270`, a local
    `[&str; 3]` array in `test_json_to_data_url_matches_gen_meta_code_v0_meta`. It is no fixture
    read and was there before the step (43 hits at the define-next commit, this one among them). The
    criterion over-matched; its intent is met
- [x] `cargo test -p iscc-lib --lib conformance -- --nocapture` — 22 passed, no panic. The data arms
    print four `could not parse test inputs` lines, the meta and iscc short-list cases one each
- [x] CI CRAP gate line after a fresh `cargo llvm-cov` — exit 0 ("0 regressed, 2 improved, 16 new,
    48 moved"), `.crap-baseline.json` untouched; `mise run check` exits 0

**Issues found:**

- The `[audit]` issue is rewritten to the part that is left, now titled "The Rust and Go conformance
    runners disagree on malformed vectors" (`normal`): five wrong-type rows plus the
    unknown-top-level-key difference. The short-list row is closed on both sides (Go pins it as
    `no_inputs`)
- Not filed (nits): the 16 new parser functions are not in `.crap-baseline.json`, so only
    `--fail-above` (30) sees them until a refresh; the highest scores 8.0. The baseline also keeps a
    stale `parse_meta_json` row from iteration 289, which fails nothing
- Not filed (pre-existing, nobody runs it): `cargo clippy -p iscc-lib --no-default-features`
    `--all-targets` fails to compile the benches, which import `gen_meta_code_v0` and
    `gen_text_code_v0` unconditionally. CI lints that feature set lib-only

**Codex review:** no actionable finding. It confirmed the three feature sets and clippy.

**Next:** two good candidates, both `normal`:

- Part 2 of the runner issue: one rule for a wrong JSON type or range on both runners. It touches
    `crates/iscc-lib/src/conformance.rs` (`bits_at`, `meta_arg`, `string_entries`, `i32_entries`,
    the pixel cast in `image_inputs`) and `packages/go/conformance.go`. The rule needs a
    decisions.md entry (a parse failure for every row is the simplest). New branches in the parsers
    will move CRAP scores, so run CI's gate line and refresh the baseline in that step if it reports
    a regression
- `version:check` gaps (1 non-test file), independent of the above

**Notes:**

- I added one rule to `crates/iscc-lib/CLAUDE.md` → "Conformance Rules": in-crate tests read inputs
    through the `conformance.rs` parsers and never decode them themselves
- Still waiting on Titusz: `alg_minhash_256([])` and the WASM `finalize()` rule need his decision
- For a "prints nothing" grep criterion, run the grep on the current tree first and name the
    expected leftovers
