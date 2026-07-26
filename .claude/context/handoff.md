# Handoff

## 2026-07-26 — Propagate the Unicode boundary fixture to the Python and Go test suites (propagation slice 1)

**Done:** Wired the canonical Unicode 16.0.0 boundary fixture into the first two binding surfaces
outside the Rust crate. Python reads `crates/iscc-lib/tests/unicode_boundary.json` directly and runs
all 12 vectors green; the pure-Go package embeds a byte-identical vendored copy and runs 9 vectors
(including all four sequence vectors), skipping exactly the 3 table-dependent cases ruled on
2026-07-26. Docs updated to name both suites and the three Go skips.

**Files changed:**

- `tests/test_unicode_boundary.py` (new): 12 parametrized vector tests (`text_clean` ×7,
    `text_collapse` ×5) against the canonical fixture, plus a `test_boundary_fixture_metadata` guard
    asserting `unicode_data_version == "16.0.0"` and per-section counts (7/5) so a truncated or
    renamed fixture cannot silently collect zero cases
- `packages/go/testdata/unicode_boundary.json` (new): byte-identical `cp` of the canonical fixture
    (verified with `cmp`, never written through a tool payload — pure ASCII preserved)
- `packages/go/unicode_boundary_test.go` (new): `//go:embed`s the vendored copy, reuses
    `parseConformanceData`, runs each section via `t.Run` subtests with a package-level skip map
    keyed `"<section>/<case>"` (3 entries, each with the go1.27 ruling as reason);
    `TestPureGoUnicodeBoundaryFixtureMetadata` asserts version, section counts, and that every skip
    key names an existing fixture case (stale-skip guard)
- `docs/unicode.md`: the "Both vector families" sentence now names the Rust, Python, and Go suites +
    the vendored copy path; the pure-Go admonition states 9-of-12 run / 3 skipped (`U+1FAE9` both
    functions, `U+113C5` in `text_clean`) until go1.27, keeping the existing freeze-rule caveat
- `packages/go/CLAUDE.md`: `testdata/unicode_boundary.json` row added to the file table; Test
    Patterns bullet describing `unicode_boundary_test.go` and its three ruled skips

**Verification:** (all run this session)

- `uv run pytest tests/test_unicode_boundary.py -q` — **13 passed** (12 vectors + metadata guard;
    criterion: ≥ 13 collected, 0 failures)
- `uv run pytest -q` — **371 passed**, no regressions
- `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go` — **ok** (exit 0, CI-exact command)
- `go vet ./...` — exit 0
- `CGO_ENABLED=0 go test -count=1 -v -run UnicodeBoundary ./... | grep -c -- '--- SKIP'` — exactly
    **3** (verbose log confirms the 3 skips are precisely the ruled cases; all 4 sequence vectors
    and the `text_collapse` U+113C5 case PASS, per next.md's "per case, not per code point" note)
- `cmp crates/iscc-lib/tests/unicode_boundary.json packages/go/testdata/unicode_boundary.json` —
    exit 0, re-checked after all hooks ran; `isascii()` still true
- `gofmt -l .` in `packages/go` — empty
- `cargo test -p iscc-lib` — **336 passed, 0 failed** (281+28+22+4+1, unchanged)
- `git status --porcelain crates/iscc-lib/ .crap-baseline.json .iai-baseline.json` — empty (no Rust
    source, no fixture edit, no baseline movement)
- `mise run check` — every prek hook Passed, no reformats after the initial `mise run format` pass
- `uv run zensical build` — "No issues found", exit 0; `uv run scripts/check_docs_nav.py` — OK, 23
    pages (no new page)
- `grep -F -c 'unicode_boundary.json' docs/unicode.md` — **2** (≥ 2);
    `grep -F -c 'testdata/unicode_boundary.json' packages/go/CLAUDE.md` — **2** (≥ 1)

**Next:** Continue propagation with the next binding slice. Note the checked-out napi artifact
(`crates/iscc-napi/iscc-lib.linux-x64-gnu.node`) is stale (still returns `aSb` for
`text_clean("a" U+A7F1 "b")` per next.md) — a napi slice must rebuild it first. The four sibling
`data.json` copies (dotnet, swift, kotlin, go) do not carry boundary vectors and were deliberately
untouched. After propagation, the criterion-4 differential sweep (remainder a2) is the last Unicode
item.

**Notes:**

- No `delete_filter_output` oracles were copied into either test (per Not In Scope) — both suites
    assert equality against `outputs.result` only; the oracle stays pinned solely in
    `test_unicode_boundary.rs`'s `SEQUENCE_VECTORS` and the issues.md table.
- Go vet gotcha: `t.Skipf(reason)` with a non-constant format string trips vet's printf check; the
    test uses `t.Skipf("%s", reason)`.
- The verbose Go run also matches the pre-existing `TestUtilsTextTrimUnicodeBoundary` (name contains
    "UnicodeBoundary"); it passes and does not affect the SKIP count.
- `mise run format` timed out once at the default 2-minute Bash timeout mid-run (mdformat had
    already applied the CLAUDE.md table alignment); the re-run completed cleanly with no further
    changes. No hook weakened, nothing suppressed.
- The canonical fixture was not edited; the vendored copy was made with `cp` per next.md's escape
    hazard note and stayed byte-identical through the full hook run.
