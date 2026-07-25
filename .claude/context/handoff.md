# Handoff

## 2026-07-25 — Unicode 16.0.0 freeze rule — vendored unassigned-range table, generator script, and the pre-normalization filter

**Done:** Implemented the Unicode 16.0.0 freeze rule in the Rust core: `text_clean` and
`text_collapse` now strip code points unassigned in Unicode 16.0.0 **before** any normalization or
category lookup, using a vendored 731-range table (`crates/iscc-lib/src/utils/unicode16.rs`)
produced by a checked-in PEP 723 generator script pinned to `unicodedata2==16.0.0`. This fixes the
one observable 16→17 normalization-drift divergence: `text_clean("a\u{A7F1}b")` now returns `"ab"`
(was `"aSb"`).

**Files changed:**

- `scripts/gen_unicode16_unassigned.py` (new): PEP 723 script (`uv run --script ...`), pins
    `unicodedata2==16.0.0`, merges `Cn` code points into maximal inclusive ranges, self-checks
    invariants (731 ranges / 819,533 code points / sorted / non-adjacent / first range
    `0x0378..=0x0379`) via `SystemExit` (no `assert`, avoids ruff S101), writes the data-only Rust
    module.
- `crates/iscc-lib/src/utils/unicode16.rs` (new, generated): module doc header declaring Unicode
    16.0.0 + "do not edit by hand", and `pub(crate) const UNASSIGNED_RANGES: [(u32, u32); 731]`.
- `crates/iscc-lib/src/utils.rs`: declared `#[cfg(feature = "text-processing")] mod unicode16;`,
    added feature-gated `is_unassigned_in_unicode16` (fast path below first gap + binary search),
    fused `.filter(|&c| !is_unassigned_in_unicode16(c))` into the existing iterator chains of
    `text_clean` (before `.nfkc()`) and `text_collapse` (before `.nfd()`) — no extra intermediate
    String. Updated both public doc comments. Added 5 feature-gated tests.
- `pyproject.toml`: added the generator script to `[tool.ty.src] exclude` (unresolved-import for
    `unicodedata2` confirmed before; `uv run ty check` → "All checks passed!" after), mirroring the
    `conanfile.py` precedent per next.md's instructions.
- `.crap-baseline.json`: refreshed via `mise run crap:baseline`. Diff sanity-checked: only utils.rs
    line-number moves plus one new entry (`is_unassigned_in_unicode16`, CRAP 2.0 — far below the 30
    threshold). Note the gate already passed *before* the refresh (0 regressed, 1 new).
- `crates/iscc-lib/CLAUDE.md` (docs, unbudgeted): "Text normalization order matters" pitfall now
    leads with the freeze filter and the regeneration command.

**Verification:** (each item reproduced with a command in this session)

- `cargo test -p iscc-lib` — 326 passed, 0 failed (275 unit incl. the 5 new tests + 28 + 22
    integration + 1 doctest).
- Feature matrix: `--no-default-features` 258 passed,
    `--no-default-features --features   text-processing` 296 passed, `--all-features` green — all 0
    failed (mirrors CI).
- `cargo test --workspace` — exit 0, every `test result:` line ok (binding crates unaffected).
- Clippy clean with `-D warnings`: `--workspace --all-targets`,
    `-p iscc-lib   --no-default-features`, `-p iscc-lib --all-features`. `cargo fmt --check` exit 0
    (generated file is rustfmt-stable).
- Behaviour assertions all pass as `#[test]`s: `text_clean("a\u{A7F1}b") == "ab"`,
    `text_clean("a\u{20C1}b") == "ab"`, U+1FAE9 and U+113C5 retained by `text_clean`,
    `text_collapse("a\u{113C5}b") == "ab"`, `text_collapse("a\u{1FAE9}b")` retained; helper
    true/false checks; table invariants (len 731, ascending/non-adjacent, 819,533 covered); ASCII
    regression guard.
- Regeneration deterministic: verified **twice** (before and after ruff-format touched the script) —
    `uv run --script scripts/gen_unicode16_unassigned.py` then `git status --porcelain` on the
    generated file shows staged-add only, no working-tree modification.
- CRAP gate:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression   --fail-above`
    exit 0 (both against the old baseline and trivially after refresh).
- Perf gate: `mise run bench:iai:check` exit 0 — 16/16 benches within the 10% Ir tolerance. Hot
    paths did not regress; measured deltas: `bench_text_code.chars_1000` −3.27%,
    `bench_meta_code.name_desc` −1.45%, `bench_meta_code.name_only` −1.48% (Ir *decreased* —
    ASCII-heavy bench inputs take one integer compare per char; the reductions are code-layout
    effects). Largest increase anywhere: `bench_mixed_code.two_codes` +1.96%. No
    `.iai-baseline.json` refresh needed.
- `mise run check` — exit 0, all 15 hooks Passed. (First run auto-fixed: ruff-format rewrapped the
    generator script, mdformat rewrapped the CLAUDE.md paragraph; determinism re-verified after.)

**Next:** Step (b) of the Unicode plan: boundary conformance vectors (retained U+1FAE9, decomposing
U+113C5, stripped-post-16.0 U+20C1) across every binding's conformance suite — note the Python
binding's installed extension predates this change, so `uv run maturin develop` is needed first.
Then step (c): the full-code-space differential sweep (spec criterion 4). The Go package's
Unicode-15.0 tables decision also belongs to the binding step.

**Notes:**

- The CRAP gate passed even without the baseline refresh (the new helper scores 2.0, no function
    regressed); I refreshed anyway because next.md's Scope lists `.crap-baseline.json` and the
    refresh keeps line numbers current for future `--fail-regression` runs.
- `iai-callgrind-runner` prints a spurious "No version information found for iai-callgrind" warning
    when invoked with bare `--version`; the actual bench run pairs fine (16 finished, 0 regressed).
- The generator emits uppercase hex (`0xA7F1`) padded to 4 digits; rustfmt confirms format-stable
    (one tuple per line, 4-space indent, trailing commas) — matches next.md's verified fact.
- Out-of-scope observation (spec bookkeeping, for update-state): the `Verified when` checkbox "Code
    points unassigned in Unicode 16.0.0 are removed before normalization..." in `specs/rust-core.md`
    is now satisfied for the Rust core; the binding-vector and differential-sweep checkboxes remain
    open.
- `GeneralCategory::Unassigned` arms in `is_c_category` / `is_cmp_category` intentionally retained
    as defence for non-16.0 table versions, per next.md's Not-In-Scope.
