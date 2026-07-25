# Handoff

## 2026-07-25 — Land the Unicode 16.0 boundary vectors as a Rust-core fixture

**Done:** Created the data-driven Unicode 16.0.0 boundary fixture
`crates/iscc-lib/tests/unicode_boundary.json` (ASCII-only, `data.json`-shaped: `_metadata` +
`text_clean`/`text_collapse` sections, 4 single-code-point cases each covering U+1FAE9, U+113C5,
U+20C1, U+A7F1) and its loader test `tests/test_unicode_boundary.rs` (1 ungated metadata/shape guard
\+ 2 `text-processing`-gated vector tests). The inline assertions in `utils.rs` remain untouched per
Scope; nothing under `crates/iscc-lib/src/` changed.

**Files changed:**

- `crates/iscc-lib/tests/unicode_boundary.json`: new fixture, generated via
    `json.dumps(ensure_ascii=True)` so escapes match next.md's verified surrogate pairs exactly
    (`🫩`, `𑏅`, `⃁`, `꟱`); byte-verified pure ASCII
- `crates/iscc-lib/tests/test_unicode_boundary.rs`: new loader test — ungated `include_str!` +
    metadata test (anti-silent-skip: section rename/emptying fails even with `text-processing` off),
    gated `run_boundary_section` helper asserting each case with the case key in the message and an
    executed-case counter == 4
- `crates/iscc-lib/CLAUDE.md`: one bullet under "Conformance Rules" naming the second vector file,
    its declared-Unicode-version-gate purpose, and its role as binding propagation source

**Verification:** All next.md criteria reproduced this session, each from actual command output:

- [x] `cargo test -p iscc-lib --test test_unicode_boundary` → 3 passed; 0 failed
- [x] `cargo test -p iscc-lib` → exit 0, all targets `0 failed` (275 lib + integration targets)
- [x] `cargo test -p iscc-lib --no-default-features` → exit 0; boundary target ran exactly the
    ungated `test_boundary_fixture_metadata` (1 passed)
- [x] `cargo test -p iscc-lib --no-default-features --features text-processing` → exit 0; boundary
    target 3 passed
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` → exit 0
- [x] `cargo clippy -p iscc-lib --no-default-features -- -D warnings` → exit 0
- [x] `cargo fmt --all --check` → exit 0 (after one rustfmt pass reflowed two `assert_eq!`)
- [x] Fixture-structure heredoc → `fixture OK`; four code points exact per section
- [x] Independent derivation heredoc (`unicodedata2==16.0.0`) → `derivation OK` — expected values
    reproduced from Unicode 16.0 data without consulting the Rust implementation
- [x] `prek run check-json --files crates/iscc-lib/tests/unicode_boundary.json` → Passed
- [x] Core untouched: `git status --porcelain` on `src/`, `data.json`, both baselines → empty;
    `grep -c '!is_unassigned_in_unicode16(c)' src/utils.rs` → 2
- [x] `grep -c 'unicode_boundary.json' crates/iscc-lib/CLAUDE.md` → 1
- [x] `mise run check` → 15/15 hooks Passed, exit 0

**Next:** Step (b) — propagate the fixture to the bindings' conformance tests — remains blocked on
the parked HUMAN REVIEW sequence-ordering ruling and the Go Unicode-15.0-tables decision; do not
start it without those. Unblocked alternatives from the iteration-140 review: the `release.yml`
static-check script (issue filed, scoped: one `scripts/` file + prek hook + CI step), or the
`rubygems/configure-rubygems-credentials` pin once Titusz answers the tag-vs-SHA question.

**Notes:**

- The fixture deliberately duplicates the four inline `utils.rs` assertions rather than replacing
    them — next.md's Not In Scope keeps the inline tests covering the private
    `is_unassigned_in_unicode16` helper and avoids a CRAP-baseline refresh. The review should not
    flag this as duplication debt; it is the specified end state for this step.
- `run_boundary_section` takes `fn(&str) -> String`, which `iscc_lib::text_clean`/`text_collapse`
    coerce to directly — no closures, no generics.
- The first Write of the JSON produced literal (non-ASCII) characters; it was immediately
    regenerated via Python `json.dumps(ensure_ascii=True)` with a `max(bytes) < 128` assertion, so
    the committed file is byte-verified ASCII with LF endings and a trailing newline.
- No `Cargo.toml` change was needed: `serde_json` is a normal dependency of `iscc-lib`, linkable
    from `tests/` in every feature configuration, as next.md predicted.
- No hot path touched, no baseline regenerated, no binding changed — the only tracked-file diff
    besides the two new test assets is the 3-line CLAUDE.md addition (mdformat reflowed it once
    during `mise run format`).
