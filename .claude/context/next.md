# Next Work Package

## Step: Unicode 16.0.0 freeze rule — vendored unassigned-range table, generator script, and the pre-normalization filter

## Goal

Implement the first of the three unmet Unicode criteria in `specs/rust-core.md`: strip code points
unassigned in Unicode 16.0.0 from the input **before** any normalization or category lookup in
`text_clean` / `text_collapse`, using a vendored range table produced by a checked-in generator
script. This makes Rust-core text output invariant under future Unicode table upgrades and fixes one
real, observable divergence today (`text_clean("a\u{A7F1}b")` currently returns `"aSb"` because
`unicode-normalization` ships Unicode 17.0 tables; it must return `"ab"`).

## Scope

**File budget: 3 source files** (`scripts/gen_unicode16_unassigned.py`,
`crates/iscc-lib/src/utils.rs`, and — only if needed, see Implementation Notes — `pyproject.toml`).
`crates/iscc-lib/src/utils/unicode16.rs` is tool output from the checked-in generator and the two
`*-baseline.json` files are gate baselines, so neither counts against the budget (same rule as
`Cargo.lock`); tests and docs are excluded by protocol.

- **Create**:
    - `scripts/gen_unicode16_unassigned.py` — checked-in generator; emits the vendored table from
        `unicodedata2==16.0.0`.
    - `crates/iscc-lib/src/utils/unicode16.rs` — **generated** data module (`UNASSIGNED_RANGES`).
- **Modify**:
    - `crates/iscc-lib/src/utils.rs` — declare the submodule, add the lookup helper, apply the filter
        in `text_clean` and `text_collapse`, update both doc comments, add tests.
    - `.crap-baseline.json` — refresh (the CRAP `--fail-regression` gate is CI-only; a new
        branch/function in covered code reddens CI otherwise).
    - `.iai-baseline.json` — **only if** `mise run bench:iai:check` fails; see Implementation Notes.
    - `crates/iscc-lib/CLAUDE.md` (docs, unbudgeted) — the "Text normalization order matters" pitfall
        now starts with the freeze filter.
    - `pyproject.toml` — **only if** `uv run ty check` reports `unresolved-import` for the generator
        (see Implementation Notes).
- **Reference**:
    - `.claude/context/specs/rust-core.md` → "Unicode data version is part of the conformance
        contract" (the four numbered requirements and the `Verified when` checkboxes).
    - `.claude/context/issues.md` → "Declare and gate a Unicode data version (DECIDED)" — points 1–4
        of the resolution.
    - `crates/iscc-lib/src/utils.rs` (current `is_c_category` / `is_cmp_category` / `text_clean` /
        `text_collapse`).
    - `mise.toml` tasks `coverage`, `crap:baseline`, `bench:iai:check`, `bench:iai:baseline`.
    - `.github/workflows/ci.yml` lines ~30–46 (the `--no-default-features` clippy/test jobs) and
        ~389–397 (the exact CRAP gate command).

## Not In Scope

- **Boundary conformance vectors in the bindings** (spec criterion 3 / issue step (b)) — Python,
    Node, WASM, FFI, JNI, Ruby, Go, C#, C++, Swift, Kotlin conformance suites and the five
    `data.json` copies stay untouched this step.
- **The full-code-space differential sweep** (spec criterion 4) — proving output-equivalence to
    uniform Unicode 16.0.0 tables across all 1,112,032 code points is its own step with its own
    harness. Do not build a sweep harness here; the targeted boundary tests below are the guard for
    this step.
- **The Go package's Unicode-15.0 tables** — the vendor-the-delta-vs-skip decision named in the
    issue belongs to the binding step.
- **Removing the `GeneralCategory::Unassigned` arms** from `is_c_category` / `is_cmp_category` —
    they stay as a defence for table versions other than 16.0.
- **Touching `unicode-normalization` / `unicode-general-category` pins** — the spec explicitly says
    exact pins are not required once the freeze rule exists.
- **Adding `unicodedata2` to the project's dependency groups** — it would need a permanent hold-back
    in every future dependency refresh.
- **Docs-site prose about the Unicode contract** (`docs/*.md`) — the existing examples are all ASCII
    and stay valid; a docs pass can follow the binding step.

## Implementation Notes

**Verified facts from scoping (do not re-derive):**

- `unicodedata2==16.0.0` yields exactly **731** maximal `Cn` ranges covering **819,533** code
    points; the first range is `U+0378..=U+0379`. These match the spec's numbers.

- Current behaviour of the four boundary code points (measured against the installed binding):

    | Code point                     | `text_clean("a?b")` today | expected after this step |
    | ------------------------------ | ------------------------- | ------------------------ |
    | `U+A7F1` (Cn in 16, Lm in 17)  | `"aSb"` ← the bug         | `"ab"`                   |
    | `U+1FAE9` (So, new in 16)      | retained                  | retained (unchanged)     |
    | `U+113C5` (Mc, new in 16)      | retained                  | retained (unchanged)     |
    | `U+20C1` (Cn in 16, new in 17) | `"ab"`                    | `"ab"` (unchanged)       |

    `U+A7F1` is the single 16→17 normalization-drift code point and is the only observable behaviour
    change in this step — NFKC under 17.0 tables maps it to `S` before the category filter can drop
    it. Filtering before normalization is what fixes it.

- `rustfmt` keeps a long array literal one element per line, so a generated
    `[(0x0378, 0x0379),\n    ...]` block with 4-space indent and trailing commas is format-stable
    (checked with `rustfmt --edition 2024`).

**Generator script** (`scripts/gen_unicode16_unassigned.py`):

- Use PEP 723 inline script metadata pinning `unicodedata2==16.0.0`, run it as
    `uv run --script scripts/gen_unicode16_unassigned.py`. This keeps `pyproject.toml` / `uv.lock`
    free of a generator-only dependency. (Network is available; the uv cache for this package is
    already warm.)
- Iterate `range(0x110000)`, treat `unicodedata2.category(chr(cp)) == "Cn"` as unassigned, merge
    into maximal inclusive ranges, and write `crates/iscc-lib/src/utils/unicode16.rs`.
- Have the script assert its own invariants before writing (731 ranges / 819,533 code points /
    sorted / non-adjacent) so a future `unicodedata2` mistake fails loudly.
- The emitted file must be **data only** — a module doc comment saying it is generated and must not
    be hand-edited, the declared Unicode version, and
    `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731] = [...];`. Keep the lookup logic in
    `utils.rs` so regeneration can never clobber hand-written code.
- If `uv run ty check` then reports `unresolved-import` for `unicodedata2`, append the script path
    to the existing `[tool.ty.src] exclude` list in `pyproject.toml` with a comment mirroring the
    `packages/cpp/conanfile.py` precedent ("generator-only dependency, not a project dependency").
    Do **not** silence it with an inline ignore and do **not** add the package to
    `[dependency-groups]`.

**Core change** (`crates/iscc-lib/src/utils.rs`):

- `#[cfg(feature = "text-processing")] mod unicode16;` (the file lives at `src/utils/unicode16.rs`;
    a `utils.rs` + `utils/` pair is valid in edition 2018+). CI runs
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` and
    `cargo test -p iscc-lib --no-default-features`, so the module, the helper, and its tests must
    all be feature-gated or the build breaks on dead code.

- Helper, also feature-gated:

    ```rust
    fn is_unassigned_in_unicode16(c: char) -> bool {
        let cp = c as u32;
        if cp < unicode16::UNASSIGNED_RANGES[0].0 {
            return false; // fast path: everything below the first gap is assigned
        }
        unicode16::UNASSIGNED_RANGES
            .binary_search_by(|&(lo, hi)| { /* Greater if cp < lo, Less if cp > hi, else Equal */ })
            .is_ok()
    }
    ```

- Fuse the filter into the existing iterator chains — do **not** allocate an extra intermediate
    `String`:

    - `text_clean`:
        `let text: String = text.chars().filter(|&c| !is_unassigned_in_unicode16(c)).nfkc().collect();`
    - `text_collapse`:
        `text.chars().filter(|&c| !is_unassigned_in_unicode16(c)).nfd().collect::<String>().to_lowercase()`
    - `UnicodeNormalization` is implemented for any `Iterator<Item = char>`, so this compiles as-is.
    - Leave every later step (newline handling, empty-line collapsing, C/M/P filtering, final NFKC)
        exactly as it is.

- Update both public doc comments to state that code points unassigned in Unicode **16.0.0** are
    removed before normalization, and why (declared data version / output invariance).

**Tests to add** (in the existing `mod tests` in `utils.rs`, all
`#[cfg(feature = "text-processing")]`):

1. The four boundary assertions from the table above for `text_clean`, plus
    `text_collapse("a\u{113C5}b") == "ab"` (Mc mark dropped by the C/M/P filter) and
    `text_collapse("a\u{1FAE9}b") == "a\u{1FAE9}b"`.
2. Helper unit test: `is_unassigned_in_unicode16` is `true` for `U+0378`, `U+A7F1`, `U+20C1` and
    `false` for `'a'`, `U+A7CB`, `U+1FAE9`, `U+113C5` (check any additional code point against the
    generated table rather than assuming).
3. Table invariant test: length is 731, ranges are strictly ascending, non-overlapping and
    non-adjacent (`prev.1 + 1 < next.0`), each `lo <= hi`, and the covered total is 819,533.
4. A regression test that existing behaviour is untouched for ASCII (`text_clean` and
    `text_collapse` on a plain sentence) — cheap insurance that the filter did not reorder
    anything.

**Gate handling (do this in the same commit, not after CI tells you):**

- CRAP: `mise run coverage`, then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`. If
    it fails, refresh with `mise run crap:baseline` and sanity-check the JSON diff — only
    `text_clean` / `text_collapse` / the new helper should move materially. A brand-new function
    above 30.0 means the helper is too complex; simplify instead of raising the threshold.
- Perf: run `mise run bench:iai:check` (needs valgrind; already installed). The filter is one
    integer compare per ASCII char, so `bench_meta_code.*` / `bench_text_code.chars_1000` should
    stay well inside the 10% Ir budget. Only if the check fails: refresh with
    `mise run bench:iai:baseline` and state the measured per-bench delta and why it is justified in
    the commit message. Never widen `tolerance_pct`.

## Verification

- `cargo test -p iscc-lib` passes — 320 existing tests plus at least 4 new ones, 0 failures.
- `cargo test -p iscc-lib --no-default-features` passes and
    `cargo test -p iscc-lib --no-default-features --features text-processing` passes (mirrors the CI
    feature matrix).
- `cargo test --workspace` passes (all binding crates still green — the vendored `data.json` vectors
    must be unaffected).
- `cargo clippy --workspace --all-targets -- -D warnings` is clean, and
    `cargo clippy -p iscc-lib --no-default-features -- -D warnings` is clean.
- `cargo fmt --check` is clean (in particular for the generated `utils/unicode16.rs`).
- Behaviour assertions, as `#[test]`s in `crates/iscc-lib/src/utils.rs`:
    - `text_clean("a\u{A7F1}b") == "ab"`
    - `text_clean("a\u{20C1}b") == "ab"`
    - `text_clean("a\u{1FAE9}b") == "a\u{1FAE9}b"`
    - `text_clean("a\u{113C5}b") == "a\u{113C5}b"`
    - `text_collapse("a\u{113C5}b") == "ab"`
- Table assertions, as `#[test]`s: `UNASSIGNED_RANGES.len() == 731`; ranges sorted, non-overlapping
    and non-adjacent; covered code points sum to `819_533`.
- Regeneration is deterministic: `uv run --script scripts/gen_unicode16_unassigned.py` followed by
    `git status --porcelain crates/iscc-lib/src/utils/unicode16.rs` prints nothing.
- `grep -c 'pub(crate) const UNASSIGNED_RANGES' crates/iscc-lib/src/utils/unicode16.rs` → 1, and the
    file contains a "generated by `scripts/gen_unicode16_unassigned.py`" header line.
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exits
    0 after `mise run coverage` (reproduces the enforcing CI gate against the working tree).
- `mise run bench:iai:check` exits 0.
- `mise run check` — all hooks pass with nothing rewritten (covers `ty check`, ruff, `cargo fmt`,
    clippy, tests).

## Done When

`text_clean` and `text_collapse` strip Unicode-16.0.0-unassigned code points before any
normalization or category lookup using the vendored, regenerable range table, and every verification
command above passes on the working tree with both quality-gate baselines consistent.
