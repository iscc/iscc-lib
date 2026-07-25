# Next Work Package

## Step: Land the Unicode 16.0 boundary vectors as a Rust-core fixture

## Goal

Convert the three inline `text_clean` / `text_collapse` boundary assertions in
`crates/iscc-lib/src/utils.rs` into a standalone JSON vector file plus a loader test, so the
declared Unicode data version (16.0.0) is enforced by a data-driven fixture that step (b) can
propagate to every binding **unchanged** once the parked ordering ruling lands. This is the Rust
half of `specs/rust-core.md` criterion 3 ("a conformance vector set covering the Unicode 16.0
boundary … is exercised by the Rust test suite and by every binding's conformance test") and it is
option 1 of the iteration-140 handoff — the first library-side move after seven consecutive tooling
iterations.

## Scope

- **Create**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — the vector file (test asset)
    - `crates/iscc-lib/tests/test_unicode_boundary.rs` — loader + assertions (test)
- **Modify**: `crates/iscc-lib/CLAUDE.md` — one line under "Conformance Rules" naming the second
    vector file, its purpose (declared Unicode version gate) and that it is the propagation source
    for the bindings (doc file)
- **Reference**:
    - `crates/iscc-lib/src/utils.rs` lines ~255–295 — the inline boundary assertions being lifted
    - `crates/iscc-lib/tests/test_text_utils.rs` — the `#[cfg(feature = "text-processing")]` gating
        style to copy
    - `crates/iscc-lib/src/conformance.rs` — the `include_str!` + `data["<fn>"]["<case>"]["inputs"]`
        parse pattern the fixture schema mirrors
    - `crates/iscc-lib/tests/data.json` — `_metadata` block shape and `test_0000_*` case-key style
    - `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
        contract" (requirement 3)

Non-doc, non-test file budget used: **0** files. Nothing under `crates/iscc-lib/src/` moves.

## Not In Scope

- **No binding wiring.** Do not touch `crates/iscc-py`, `iscc-napi`, `iscc-wasm`, `iscc-ffi`,
    `iscc-jni`, `iscc-rb`, `iscc-uniffi` or `packages/*`. Step (b) also needs a Go decision (Go is
    on Unicode 15.0 tables until go1.27) and is blocked on the parked HUMAN REVIEW ruling.
- **Do not copy the fixture** into the four sibling `data.json` locations (`packages/go/testdata/`,
    `packages/dotnet/Iscc.Lib.Tests/testdata/`, `packages/swift/Tests/IsccLibTests/`,
    `packages/kotlin/src/test/resources/`) — that is step (b)'s job.
- **Do not merge these vectors into `crates/iscc-lib/tests/data.json`.** It is a vendored upstream
    artifact with hardcoded vector-count asserts in the Rust core and WASM tests; a second file
    keeps both re-vendoring and count asserts untouched.
- **No sequence vectors.** No `base + Cn + mark`, `jamo + Cn + jamo` or `Σ + Cn + cased` cases — the
    open `[review]` issue "Freeze-rule ordering diverges from iscc-core on sequences" is exactly
    about those, and expected values would have to be re-derived after the ruling.
- **No change under `crates/iscc-lib/src/`** — in particular do **not** delete the now-duplicated
    inline assertions in `utils.rs` (they cover the private `is_unassigned_in_unicode16` helper, and
    removing them would move coverage and force a CRAP baseline refresh).
- **Do not extend `conformance_selftest()`** to run the new file. That changes the semantics of a
    Tier 1 symbol exposed in all 12 bindings; it is a deliberate, separately scoped decision.
- No `gen_text_code_v0` / `gen_meta_code_v0` expected-ISCC vectors — those expected values could
    only be derived from the implementation under test. Keep the fixture at the text-utility layer.
- Do not flip the `specs/rust-core.md` checkbox for criterion 3 (bindings are still missing) and do
    not start criterion 4's full-code-space differential sweep.
- Do not edit `.claude/context/issues.md` — the review agent resolves issues.
- Do not add a `scripts/` file, a prek hook or a CI step (the `release.yml` static-check gate stays
    its own future package), and do not refresh `.crap-baseline.json` or `.iai-baseline.json`.

## Implementation Notes

### Verified facts — do not re-derive

Expected outputs were derived twice while scoping: independently from Unicode 16.0 semantics with
`unicodedata2==16.0.0` (strip `Cn` → NFKC / NFD+lower+filter C,M,P+NFKC), and by running
`cargo test -p iscc-lib --lib utils::` at HEAD. Both agree exactly:

| Code point | 16.0 category | Input | `text_clean`             | `text_collapse`          |
| ---------- | ------------- | ----- | ------------------------ | ------------------------ |
| `U+1FAE9`  | `So`          | `a…b` | `a\u{1FAE9}b` (retained) | `a\u{1FAE9}b` (retained) |
| `U+113C5`  | `Mc`          | `a…b` | `a\u{113C5}b` (retained) | `ab` (mark dropped)      |
| `U+20C1`   | `Cn`          | `a…b` | `ab` (stripped)          | `ab` (stripped)          |
| `U+A7F1`   | `Cn`          | `a…b` | `ab` (stripped)          | `ab` (stripped)          |

- `U+113C5` decomposes canonically to `U+113C2 U+113C2` under NFD — that is what makes it the spec's
    "16.0 character with a canonical decomposition".
- `U+A7F1` is the one case that is a real regression guard rather than a no-op: it is `Cn` in 16.0
    but `Lm` with an `<super> 0053` compatibility decomposition in 17.0, so NFKC-before-filter leaks
    `"aSb"`. Include it — it is the only vector that fails if the freeze filter is ever moved after
    normalization.
- Neighbours are deliberately plain ASCII `a` / `b`: they cannot compose with each other, so these
    vectors stay valid whatever the parked sequence-ordering ruling decides.
- JSON escapes (verified with `json.dumps(..., ensure_ascii=True)` — note the surrogate pairs, do
    not hand-compute them): `U+1FAE9` → `\ud83e\udee9`, `U+113C5` → `\ud804\udfc5`, `U+20C1` →
    `\u20c1`, `U+A7F1` → `\ua7f1`.

### Fixture file

Write `crates/iscc-lib/tests/unicode_boundary.json` ASCII-only using `\uXXXX` escapes (no literal
astral or unassigned bytes — the file must survive copying into four other repos' test resources).
Mirror `data.json`'s shape so a binding can reuse its existing parse helper: a top-level `_metadata`
object (`unicode_data_version: "16.0.0"`, plus `description`), then one section per function name,
each mapping `test_NNNN_<slug>` → `{"inputs": [<string>], "outputs": {"result": <string>}}`.
Sections: `text_clean` and `text_collapse`, four cases each (the four code points above). Encode the
code point and the expectation in the case slug (e.g. `test_0002_u20c1_unassigned_16_stripped`) —
JSON has no comments, so the keys are the documentation.

### Loader test

`crates/iscc-lib/tests/test_unicode_boundary.rs`:

- `const BOUNDARY_DATA: &str = include_str!("unicode_boundary.json");` — **ungated**, and parsed by
    an ungated metadata test, so no `dead_code` warning in the `--no-default-features` build that CI
    runs (`cargo test -p iscc-lib --no-default-features`,
    `cargo clippy -p iscc-lib   --no-default-features -- -D warnings`).
- `serde_json` is a normal (non-optional) dependency of `iscc-lib`, so it is linkable from `tests/`
    in every feature configuration — no new dev-dependency.
- Test 1 (ungated): `_metadata.unicode_data_version == "16.0.0"` and both sections parse with
    exactly 4 cases each. This is the anti-silent-skip guard: an empty or renamed section fails here
    even when `text-processing` is off.
- Test 2 + 3 (`#[cfg(feature = "text-processing")]`, one per function): iterate the section, call
    `iscc_lib::text_clean` / `iscc_lib::text_collapse` on `inputs[0]`, `assert_eq!` against
    `outputs.result`, include the case key in the assertion message, and assert the executed-case
    counter equals 4 at the end.
- Match `test_text_utils.rs` for gating style; keep helpers `#[cfg(feature = "text-processing")]` so
    nothing is unused when the feature is off.

### Gate interactions

No file under `crates/iscc-lib/src/` changes, so the iai-callgrind Ir baseline is untouched and the
CI-only CRAP `--fail-regression` check can only see coverage improve (new integration tests exercise
already-covered functions). Do **not** regenerate either baseline in this step.

## Verification

Run from the repo root; `mise run format` before staging.

- `cargo test -p iscc-lib --test test_unicode_boundary` → `3 passed; 0 failed`

- `cargo test -p iscc-lib` → exits 0, `0 failed` in every target

- `cargo test -p iscc-lib --no-default-features` → exits 0 (the fixture target compiles and its
    ungated metadata test runs with `text-processing` off)

- `cargo test -p iscc-lib --no-default-features --features text-processing` → exits 0 with all 3
    boundary tests running

- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` → exits 0

- `cargo clippy -p iscc-lib --no-default-features -- -D warnings` → exits 0

- `cargo fmt --all --check` → exits 0

- Fixture structure and coverage of the four code points:

    ```bash
    uv run python - <<'PY'
    import json
    d = json.load(open("crates/iscc-lib/tests/unicode_boundary.json"))
    assert d["_metadata"]["unicode_data_version"] == "16.0.0"
    want = {0x1FAE9, 0x113C5, 0x20C1, 0xA7F1}
    for sec in ("text_clean", "text_collapse"):
        assert len(d[sec]) == 4, (sec, len(d[sec]))
        seen = {ord(c) for tc in d[sec].values() for c in tc["inputs"][0]} - set(b"ab")
        assert seen == want, (sec, sorted(map(hex, seen)))
    print("fixture OK")
    PY
    ```

    Expected output: `fixture OK`, exit 0.

- Expected values are reproducible from Unicode 16.0 data independently of the Rust implementation
    (prints `derivation OK`, exit 0):

    ```bash
    uv run --no-project --with 'unicodedata2==16.0.0' python - <<'PY'
    import json, unicodedata2 as ud
    assert ud.unidata_version == "16.0.0"
    strip = lambda t: "".join(c for c in t if ud.category(c) != "Cn")
    def clean(t):
        t = ud.normalize("NFKC", strip(t))
        return "".join(c for c in t if not ud.category(c).startswith("C") or c == "\n").strip()
    def collapse(t):
        t = ud.normalize("NFD", strip(t)).lower()
        t = "".join(c for c in t if not c.isspace() and ud.category(c)[0] not in "CMP")
        return ud.normalize("NFKC", t)
    d = json.load(open("crates/iscc-lib/tests/unicode_boundary.json"))
    for sec, fn in (("text_clean", clean), ("text_collapse", collapse)):
        for k, tc in d[sec].items():
            assert fn(tc["inputs"][0]) == tc["outputs"]["result"], (sec, k)
    print("derivation OK")
    PY
    ```

- `uv run prek run check-json --files crates/iscc-lib/tests/unicode_boundary.json` → `Passed`

- Core untouched:
    `git status --porcelain crates/iscc-lib/src/ crates/iscc-lib/tests/data.json   .crap-baseline.json .iai-baseline.json`
    prints nothing, and `grep -c '!is_unassigned_in_unicode16(c)' crates/iscc-lib/src/utils.rs` →
    `2` (freeze filter still fused into both functions)

- `grep -c 'unicode_boundary.json' crates/iscc-lib/CLAUDE.md` → at least `1`

- `mise run check` → all hooks Passed, exit 0

## Done When

`crates/iscc-lib/tests/unicode_boundary.json` and its loader test exist, all four Unicode 16.0
boundary code points are asserted for both `text_clean` and `text_collapse` from data rather than
inline literals, every verification command above exits 0, and no file under `crates/iscc-lib/src/`
or any binding changed.
