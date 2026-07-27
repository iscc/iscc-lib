# Handoff

## 2026-07-27 — Review of: Land the Unicode 16.0.0 differential sweep as a permanent, fail-closed gate

**Verdict:** PASS_WITH_NOTES

**Summary:** The throwaway sweep re-invented at iterations 148/155/156 is now a committed gate:
`scripts/unicode_sweep.py` + `mise run unicode:sweep` + a standalone `unicode-sweep` CI job, with 9
pytest cases pinning its fail-closed machinery. Every verification criterion passes, the file budget
(3 non-test/non-doc files) was hit exactly, and every `Not In Scope` boundary held. Review
independently mutation-probed all four guards, proved the divergence-reporting path end-to-end, and
proved the eight-context set actually discriminates the superseded delete-filter design — then
committed that last proof as a tenth test, because nothing else guarded it. Two real hardening gaps
(both also surfaced by Codex) are filed rather than fixed here.

**Verification:**

- [x] `uv run scripts/unicode_sweep.py` exits 0, last stdout line byte-exact
    `TOTAL 17793024 comparisons, 0 divergences` — confirmed with `od -c` and a string equality test;
    provenance line `oracle: iscc-core 1.3.0, unidata 16.0.0, python 3.14.6` above it
- [x] `mise run unicode:sweep` exits 0 (run 3× this review: initial, post-stale-probe, final) — ~60
    s wall clock including the ~2 s incremental maturin rebuild
- [x] Stale-extension guard fires — `touch crates/iscc-lib/src/utils.rs` →
    `uv run scripts/unicode_sweep.py` exits **1**, naming the stale `.so` and the
    `mise run unicode:sweep` rebuild, and prints **nothing** (it fails before the sweep);
    `mise run unicode:sweep` green afterwards, `git status --porcelain` clean
- [x] Comparison-count guard fires on the **count**, not a divergence — with `CONTEXTS` shortened by
    one row (in-memory, file untouched): `expected 17793024 comparisons, got 28`, stdout empty at
    exit, so the failure cannot be mistaken for success. Scalar-count guard probed separately:
    `expected 1112064 scalar values, got 2`
- [x] `uv run pytest -q` → **389 passed** (379 pre-existing + 9 from advance + 1 review-added);
    `tests/test_unicode_sweep.py` alone → 10 passed in 0.8 s
- [x] `uv run ruff check` / `ruff format --check` / `ty check` all exit 0 (no new `[tool.ty.src]`
    exclusion needed, as next.md predicted)
- [x] next.md's PyYAML job-shape one-liner exits 0 — `unicode-sweep` is job key 21/21, name
    `Unicode sweep (16.0.0 differential)`, has a `--release` build step and a `unicode_sweep.py`
    step
- [x] `uv run zensical build` exits 0 with "No issues found"; `uv run scripts/check_docs_nav.py` →
    `OK: 23 documentation pages consistent` (existing page, no nav wiring needed)
- [x] `git status --porcelain -- crates/ .crap-baseline.json .iai-baseline.json specs/` empty — no
    Rust source, baseline or spec moved
- [x] `mise run check` exits 0 (all 17 prek hooks), working tree clean apart from the runner-owned
    `iterations.jsonl`
- [x] Extra (review): `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] Extra (review): the eight contexts **discriminate the superseded delete-filter design** —
    running `sweep()` with an `iscc-core`-based delete filter as the subject lights up exactly
    `text_clean/base_mark`, `text_clean/jamo`, `text_collapse/sigma` (831 unassigned scalars each),
    while `bare`/`ascii`/`marks`/`space`/`upper` score 0. This is the spec's "a per-code-point sweep
    is not sufficient" requirement, empirically confirmed rather than assumed
- [x] Extra (review): gate-circumvention scan over `@{upstream}..HEAD` — the only skip is the
    `pytest.mark.skipif(unidata != "16.0.0")` on the redundant zero-divergence sample test, which
    next.md prescribed and which the unconditional CI job renders non-load-bearing. No lint
    suppressions, no threshold changes, no hook weakening

**Issues found:**

- **Freshness guard is blind to dependency/toolchain-only changes** (also Codex P2, also flagged in
    the advance handoff's Notes). `check_extension_fresh` watches only `*.rs` mtimes, so a
    `cargo update` moving `unicode-normalization` — or a rustc bump — leaves a stale `.so` reporting
    a false green from a bare `uv run scripts/unicode_sweep.py`. That is exactly the upgrade the
    gate exists to validate. A missing source directory also passes vacuously
    (`max(..., default=0.0)`, reproduced here). **Not a blocker:** both authoritative paths (CI job,
    mise task) rebuild unconditionally. Filed as `normal` `[review]`; rationale and the interim
    operating rule (*always* go through `mise run unicode:sweep`) → `decisions.md` 2026-07-27 and
    learnings.md
- **Unbounded divergence retention** (Codex P2). `sweep()` keeps every `Divergence` though only 20
    are printed; a broad regression could retain millions of tuples (several GB) and OOM-kill CI
    before any diagnostic reaches the log. Filed in the same `[review]` issue
- **Minor doc imprecision, fixed here.** `docs/unicode.md` called all eight rows "sequence contexts"
    (`bare` is a single code point) and claimed a toolchain-induced change "would surface here and
    nowhere else" — the `U+A7F1` boundary vector demonstrably catches one such normalization change.
    Reworded both; also `x 8 sequence contexts` → `x 8 contexts` in the script docstring
- **Gap the arithmetic pin does not cover, closed here.** The pinned
    scalars-times-contexts-times-functions total catches a *shrunken* context set but not a
    *swapped* one — eight ASCII-neighbour shapes would keep every existing test green while making
    the sweep blind to the delete-filter design. Added
    `test_contexts_discriminate_the_delete_filter_design` (version-independent: `iscc-core` is both
    oracle and subject base, only the filter placement differs). Mutation-verified: with the three
    sequence shapes replaced by ASCII ones, the probe exposes nothing and the test reds

**Codex review:** Two P2 findings, both accepted and filed (see above): (1) direct execution can
validate a stale binary after dependency-/toolchain-only changes, since freshness considers only
Rust source mtimes; (2) the divergence list retains up to 17.8M tuples on a broad regression and can
exhaust runner memory before printing diagnostics. No false positives this run — both are real, and
(2) is the kind of finding a green gate run can never surface.

**Next:** Propagation slice 5 for the boundary fixture — 3 binding surfaces left (C FFI, C++,
Swift), plus the four sibling `data.json` copies. C++ and Swift are **blocked in this container**
(no `cmake`, no `swift`), so the only autonomously-completable piece is C FFI: `tests/test_iscc.c`
has no JSON reader and no text-function coverage at all, so it needs either a generated vector
header or hand-pinned expected strings — and `gcc` is present. Alternatively define-next could take
the freshly-filed **"Harden the Unicode differential sweep gate"** issue, which is small, fully
verifiable here, and closes two known holes in a gate that just landed; it is arguably the better
next step, since the C-FFI slice will need a design call about how a C test reads the fixture.

**Notes:**

- The 21st CI job is added but `specs/ci-cd.md`'s job table still has 14 rows — that drift is owned
    by the human-authorized "Make the CI job table in `specs/ci-cd.md` exhaustive" issue, and
    next.md correctly kept it out of scope. Whoever takes that issue should now expect **21** jobs.
- The gate's oracle is the *installed* `iscc-core` (1.3.0, pinned by `uv.lock`), not a vendored
    reference. If upstream ever adopts the freeze rule itself (proposed in `iscc-core#137`), the
    oracle changes shape and the sweep's meaning changes with it — worth re-reading the script
    docstring at that point. Only `unidata_version` is version-asserted today, not the `iscc-core`
    version.
- The sweep takes ~60 s and is deliberately **not** in `mise run test`, default `pytest`, or the
    pre-push hooks. The pytest suite exercises `sweep()` with small explicit scalar lists only, so
    the fast path stays fast — do not "helpfully" wire the full sweep into pytest.
- No API surface, no benchmarked hot path, no Rust code touched; no baseline refresh needed and none
    was made.
- `iterations.jsonl` is runner-owned and was left unstaged throughout.
- **Self-inflicted, disclosed:** the review-added test first used direct attribute assignment
    (`us.FUNCTION_PAIRS = …`) on the importlib-loaded module, which the **pre-push `ty` gate**
    rejected (`unresolved-attribute on type ModuleType`) — `mise run check` cannot see it, since
    `ty` is pre-push-only. Rewritten to `monkeypatch.setattr`, matching the sibling test; `ty` /
    ruff / pytest re-run clean and the review commit was amended. The advance agent's work was never
    implicated, so the verdict stands. Lesson for future reviews: after any step-9 edit to a `.py`
    file, re-run the **pre-push** gates (`ty check`, ruff `S`/`C901`), not just `mise run check`.
