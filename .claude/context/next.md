# Next Work Package

## Step: Propagate the Unicode boundary fixture to the Python and Go test suites (propagation slice 1)

## Goal

Wire the completed `crates/iscc-lib/tests/unicode_boundary.json` fixture into the first two of the
11 binding surfaces — Python (reads the canonical fixture by relative path) and the pure-Go package
(vendored `//go:embed` copy plus the ruled skip list) — so the Unicode 16.0.0 sentinel freeze rule
is gated outside the Rust crate for the first time. This is the handoff's "Next" (issues.md:
"Declare and gate a Unicode data version (DECIDED)", remainder **(b)**), sliced to the two surfaces
that are fully runnable in this container with no build-artifact rebuild.

## Scope

- **Create**:
    - `tests/test_unicode_boundary.py` — Python binding boundary-vector tests
    - `packages/go/testdata/unicode_boundary.json` — byte-identical `cp` of the canonical fixture
    - `packages/go/unicode_boundary_test.go` — Go boundary-vector tests with the ruled skip list
- **Modify**:
    - `docs/unicode.md` — say which suites exercise the fixture; name the three vectors Go skips
    - `packages/go/CLAUDE.md` — add the vendored fixture to the file table and a Test Patterns bullet
- **Reference**:
    - `crates/iscc-lib/tests/unicode_boundary.json` (the fixture; do not edit)
    - `crates/iscc-lib/tests/test_unicode_boundary.rs` (the guard pattern to mirror, loosely)
    - `tests/test_conformance.py` (Python vector-loader idiom: `Path(__file__).parent.parent / ...`,
        `pytest.param(tc, id=name)`)
    - `packages/go/code_content_text_test.go` and `packages/go/conformance.go` (`vectorEntry`,
        `parseConformanceData`, `//go:embed testdata/...`)
    - `.claude/context/decisions.md` 2026-07-26 "Go skips the Unicode-16 boundary vectors until
        go1.27" (the ruling the skip list implements)

## Not In Scope

- **The other nine binding surfaces** (napi, WASM, Ruby, C FFI, JNI/Java, UniFFI, Kotlin, Swift, C#,
    C++). They are later slices. Note the checked-out napi artifact
    (`crates/iscc-napi/iscc-lib.linux-x64-gnu.node`) is **stale** — it still returns `aSb` for
    `text_clean("a" U+A7F1 "b")` — so a napi slice must rebuild it first; that cost is exactly why
    napi is not in this step.
- **Criterion 4 / remainder (a2)** — the 1,112,064-scalar + sequence-class differential sweep. Not
    this step.
- **Copying `delete_filter_output` oracles into the binding tests.** The corrected oracles live in
    exactly two places (the `SEQUENCE_VECTORS` const in `test_unicode_boundary.rs` and the issues.md
    table). Equality against `outputs.result` already catches a delete-filter regression;
    replicating the oracle 11 times would re-open the mislabeling hazard that hit iteration 149.
- **Editing the canonical fixture** — no new vectors, no renames, no reformatting. It is a finished
    propagation source.
- **Extending the public Go `ConformanceSelftest()`** or any other public API. The boundary vectors
    are test-only; `ConformanceSelftest` stays scoped to the `gen_*_v0` vectors from
    `testdata/data.json`.
- **Touching `packages/go/utils.go`** — no freeze table, no 15.0→16.0 delta, no `cases.Caser` hoist
    (the per-call `Caser` is deliberate).
- **A drift gate for vendored vector copies.** There is none today for the five `data.json` copies
    either; adding one is a separate, arguably human-gated step. This step verifies byte-identity
    once, with `cmp`.
- **Any baseline refresh.** No Rust source and no Rust test changes, so neither
    `.crap-baseline.json` nor `.iai-baseline.json` may move.

## Implementation Notes

### Verified facts measured this iteration — do not re-derive

**Fixture shape** (`crates/iscc-lib/tests/unicode_boundary.json`, pure ASCII, 2344 bytes):

```text
{"_metadata": {"description": ..., "unicode_data_version": "16.0.0"},
 "text_clean":    { <7 cases> },
 "text_collapse": { <5 cases> }}
```

Every case is `{"inputs": ["<one string>"], "outputs": {"result": "<string>"}}`. Case keys are
`test_0000_u1fae9_assigned_so_retained`, `test_0001_u113c5_assigned_mc_retained` (`text_clean`) /
`test_0001_u113c5_assigned_mc_mark_dropped` (`text_collapse`),
`test_0002_u20c1_unassigned_16_stripped`, `test_0003_ua7f1_unassigned_16_stripped`, then the
sequence cases `test_0004_seq_u0378_blocks_canonical_composition`,
`test_0005_seq_u0378_blocks_hangul_composition`, `test_0006_seq_ua7f1_no_decomposition_leak`
(`text_clean`) and `test_0004_seq_u0378_preserves_final_sigma` (`text_collapse`).

**Python passes all 12 vectors today.** Probed this iteration against the installed editable
extension (`crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so`, built today): 12 of 12 exact matches,
0 failures. The "stale wheel" warning from earlier iterations no longer applies. If a vector
unexpectedly fails, rebuild with `uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`
before suspecting the fixture.

**Go passes 9 of 12; exactly three fail.** Probed this iteration by running `TextClean` /
`TextCollapse` from a throwaway module against the canonical fixture:

| section         | case                                    | Go result                                   |
| --------------- | --------------------------------------- | ------------------------------------------- |
| `text_clean`    | `test_0000_u1fae9_assigned_so_retained` | **FAIL** — Go drops U+1FAE9 (Cn under 15.0) |
| `text_clean`    | `test_0001_u113c5_assigned_mc_retained` | **FAIL** — Go drops U+113C5 (Cn under 15.0) |
| `text_collapse` | `test_0000_u1fae9_assigned_so_retained` | **FAIL** — same cause                       |
| all nine others | —                                       | PASS                                        |

Two consequences to honour:

1. **All four sequence vectors PASS in Go** (including
    `text_collapse/test_0004_seq_u0378_preserves_final_sigma` — the iteration-147 `Final_Sigma` fix
    is live). They must be run, not skipped.
2. **`text_collapse/test_0001_u113c5_assigned_mc_mark_dropped` PASSES** — Go removes the mark as
    category `C`, the expected output removes it as category `M`, and both land on `ab`. So the
    skip list is **per case, not per code point**: skip exactly the three rows above.

### Copying the fixture (escape hazard)

Copy with the shell, never with Write/Edit:

```bash
cp crates/iscc-lib/tests/unicode_boundary.json packages/go/testdata/unicode_boundary.json
```

Writing the file through a tool payload decodes its `\uXXXX` escapes into literal UTF-8 and silently
corrupts an ASCII-escaped fixture (this bit both advance and review in iteration 149). Verify the
copy with `cmp`, not by eye.

### Python test (`tests/test_unicode_boundary.py`)

Mirror `tests/test_conformance.py`: module docstring, a `FIXTURE` path constant built from
`Path(__file__).parent.parent`, a small `load_cases(section)` helper returning
`[pytest.param(tc, id=name) for name, tc in section.items()]`, then two parametrized tests
(`text_clean`, `text_collapse`) asserting `fn(*tc["inputs"]) == tc["outputs"]["result"]`. Import
`text_clean` / `text_collapse` from `iscc_lib`.

Add one non-parametrized guard, `test_boundary_fixture_metadata`, asserting
`unicode_data_version == "16.0.0"` and the per-section case counts (7 and 5). Without it a truncated
or renamed fixture would silently degrade to zero parametrized cases and still report success.

### Go test (`packages/go/unicode_boundary_test.go`, package `iscc`)

- Embed the vendored copy: `import _ "embed"` plus `//go:embed testdata/unicode_boundary.json` on a
    `var unicodeBoundaryData string`. Embedding from a `_test.go` file works and `testdata/` is
    embeddable (`conformance.go` already embeds `testdata/data.json`).
- Reuse the existing `parseConformanceData` helper: it skips `_`-prefixed keys and its `vectorEntry`
    (`Inputs []json.RawMessage`, `Outputs map[string]interface{}`) fits these cases —
    `json.Unmarshal(vec.Inputs[0], &in)` and `vec.Outputs["result"].(string)`. Parse `_metadata`
    with a separate tiny unmarshal in the guard test.
- One test per section (`TestPureGoUnicodeBoundaryTextClean`,
    `TestPureGoUnicodeBoundaryTextCollapse`) with `t.Run(name, ...)` subtests, so each skip shows up
    as its own `--- SKIP` line.
- Skip list as a package-level map keyed `"<section>/<case>"` with a reason string per entry, e.g.
    `"go1.27 (~Aug 2026) brings Unicode 16/17 tables to the stdlib and x/text; Go 1.26 classifies   U+1FAE9 as unassigned. Ruled 2026-07-26 (decisions.md): skip, do not vendor the 15.0-to-16.0   delta."`
    Call `t.Skipf` with it.
- Guard test `TestPureGoUnicodeBoundaryFixtureMetadata`: assert `unicode_data_version == "16.0.0"`,
    section counts 7 and 5, **and that every skip-map key names a case that exists in the fixture**
    (`t.Errorf` otherwise) — that keeps a stale skip from silently masking a renamed vector.
- Keep `gofmt` clean (tabs, standard import grouping); CI runs `go vet ./...` too.

### Docs

- `docs/unicode.md`, the sentence that currently reads "checked into the repository as
    `crates/iscc-lib/tests/unicode_boundary.json` and exercised by the Rust test suite" — widen it:
    the Rust suite, the Python test suite, and the pure-Go package via the vendored copy at
    `packages/go/testdata/unicode_boundary.json`.
- `docs/unicode.md`, the `!!! note "The pure-Go package"` admonition — state that Go runs 9 of the
    12 boundary vectors, including all four sequence vectors, and skips exactly three (U+1FAE9 in
    both functions and U+113C5 in `text_clean`) until go1.27 ships newer Unicode tables. Keep the
    existing freeze-rule caveat.
- `packages/go/CLAUDE.md` — add a `testdata/unicode_boundary.json` row to the file table (near the
    existing `testdata/data.json` row) and a Test Patterns bullet describing
    `unicode_boundary_test.go` and its three ruled skips.
- Do not add a new docs page — `scripts/check_docs_nav.py` must stay at 23 pages.

## Verification

- `uv run pytest tests/test_unicode_boundary.py -q` — 0 failures, at least 13 tests collected (12
    vector cases + the metadata guard)
- `uv run pytest -q` — whole Python suite passes (no regressions)
- `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go` — exit 0 (CI-exact command)
- `go vet ./...` in `packages/go` — exit 0
- `CGO_ENABLED=0 go test -count=1 -v -run UnicodeBoundary ./... | grep -c -- '--- SKIP'` in
    `packages/go` — output is exactly `3`
- `cmp crates/iscc-lib/tests/unicode_boundary.json packages/go/testdata/unicode_boundary.json` —
    exit 0 (byte-identical, still pure ASCII)
- `gofmt -l .` in `packages/go` — empty output
- `cargo test -p iscc-lib` — 336 passed, 0 failed (unchanged)
- `git status --porcelain crates/iscc-lib/ .crap-baseline.json .iai-baseline.json` — empty (the
    canonical fixture, the Rust core and both baselines are untouched)
- `mise run check` — every prek hook passes with no reformats
- `uv run zensical build` — exit 0, "No issues found"; `uv run scripts/check_docs_nav.py` — exit 0,
    23 pages
- `grep -F -c 'unicode_boundary.json' docs/unicode.md` — at least `2` (canonical path + vendored Go
    copy); `grep -F -c 'testdata/unicode_boundary.json' packages/go/CLAUDE.md` — at least `1`

## Done When

Python runs all 12 boundary vectors green, the pure-Go package runs 9 and skips exactly the 3 ruled
table-dependent ones from a byte-identical vendored copy, the docs name both suites and the three
skips, and every verification command above passes.
