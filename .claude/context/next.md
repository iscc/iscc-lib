# Next Work Package

## Step: Propagate the Unicode boundary fixture to the WASM and Ruby test suites (propagation slice 2)

## Goal

Gate the Unicode 16.0.0 sentinel freeze rule on two more binding surfaces — `iscc-wasm` and
`iscc-rb` — by running the canonical `crates/iscc-lib/tests/unicode_boundary.json` fixture through
their `text_clean` / `text_collapse` exports. This is the handoff's "Next" (issues.md: "Declare and
gate a Unicode data version (DECIDED)", remainder **(b)**), sliced to the pair the handoff named as
cheapest in-container; it takes criterion 3 from 2 of 11 surfaces to 4 of 11.

## Scope

- **Create**:
    - `crates/iscc-wasm/tests/unicode_boundary.rs` — boundary vectors for the WASM exports
    - `crates/iscc-rb/test/test_unicode_boundary.rb` — boundary vectors for the Ruby exports
- **Modify** (docs only — this step changes **zero** non-test, non-doc source files):
    - `docs/unicode.md` — widen the "exercised by" sentence to name the WASM and Ruby suites
    - `crates/iscc-wasm/CLAUDE.md` — add `unicode_boundary.rs` to the `tests/` layout block
    - `crates/iscc-rb/CLAUDE.md` — add a `test/test_unicode_boundary.rb` row to the file table
- **Reference**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — the fixture (do **not** edit)
    - `tests/test_unicode_boundary.py` and `packages/go/unicode_boundary_test.go` — the two loader
        patterns landed in slice 1; mirror them idiomatically
    - `crates/iscc-wasm/tests/conformance.rs` — the fixture-loading idiom to copy
        (`include_str!("../../iscc-lib/tests/data.json")` parsed with `serde_json`)
    - `crates/iscc-rb/test/test_conformance.rb` — the `define_method` per-vector idiom and the
        `File.expand_path("../../iscc-lib/tests/…", __dir__)` path shape
    - `crates/iscc-wasm/tests/unit.rs` lines ~142–215 — the existing `text_clean` / `text_collapse`
        WASM tests this file extends

## Not In Scope

- **The vendored-copy byte-identity drift gate** (issues.md `normal` `[review]`, "Gate byte-identity
    of the vendored test-vector copies"). Deliberately deferred once more: both surfaces in this
    slice read the canonical fixture **in place**, so this step creates no new vendored copy and the
    gate protects nothing new yet. Land it immediately *before* the `packages/{dotnet,kotlin,swift}`
    slice, which is the one that adds three more copies.
- **The seven other ungated surfaces**: napi, C FFI, JNI/Java, UniFFI, Kotlin, Swift, C#, C++. In
    particular do **not** rebuild `crates/iscc-napi/iscc-lib.linux-x64-gnu.node` here (it is stale
    and untracked — `crates/iscc-napi/.gitignore:4` ignores `*.node`); napi is its own slice.
- **Criterion 4 / remainder (a2)** — the 1,112,064-scalar + sequence-class differential sweep.
- **Any skip list, `#[ignore]`, or `Minitest#skip`.** Both surfaces execute the same Rust core, so
    all 12 vectors must pass. Skips are Go-only, authorized for Go alone by `decisions.md`
    2026-07-26. A failing vector here means the local build artifact is stale — rebuild it, never
    skip it. Do not touch `packages/go/`.
- **Copying `delete_filter_output` oracles into these tests.** Equality against `outputs.result`
    already reds a delete-filter regression (mutation-verified in the iteration-150 review); the
    oracles stay in `SEQUENCE_VECTORS` (Rust) and the issues.md table only.
- **Editing the canonical fixture** — no new vectors, no renames, no reformatting.
- **Any baseline refresh.** `mise run coverage` / `cargo crap` are `-p iscc-lib` only and
    `.crap-baseline.json` contains zero `iscc-wasm` entries; no hot-path code moves. Neither
    `.crap-baseline.json` nor `.iai-baseline.json` may change.
- **Touching `crates/iscc-rb/Gemfile`, `Gemfile.lock`, the gemspec, or `Rakefile`**, and no
    dependency bumps of any kind.

## Implementation Notes

### Verified facts measured this iteration — do not re-derive

**Fixture shape** (pure ASCII, 2344 bytes): top-level keys `_metadata`, `text_clean` (7 cases),
`text_collapse` (5 cases). `_metadata.unicode_data_version == "16.0.0"`. Every case is
`{"inputs": ["<one string>"], "outputs": {"result": "<string>"}}`. Case keys are
`test_0000_u1fae9_assigned_so_retained`, `test_0001_u113c5_assigned_mc_retained` (`text_clean`) /
`test_0001_u113c5_assigned_mc_mark_dropped` (`text_collapse`),
`test_0002_u20c1_unassigned_16_stripped`, `test_0003_ua7f1_unassigned_16_stripped`, plus the
sequence cases `test_0004_seq_u0378_blocks_canonical_composition`,
`test_0005_seq_u0378_blocks_hangul_composition`, `test_0006_seq_ua7f1_no_decomposition_leak`
(`text_clean`) and `test_0004_seq_u0378_preserves_final_sigma` (`text_collapse`).

**Both toolchains work offline in this container, measured now:**

- `wasm-pack test --node crates/iscc-wasm --features conformance` → **exit 0 in ~2 min** (wasm-pack
    0.13.1; its `wasm-bindgen` and `wasm-opt` are already in `~/.cache/.wasm-pack`, no download).
    `unit.rs` currently reports **78 passed, 0 failed**.
- `crates/iscc-rb`: `bundle exec rake compile` → exit 0 in ~1.5 min; `bundle exec rake test` → **111
    runs, 299 assertions, 0 failures, 0 errors, 0 skips**; `bundle exec standardrb` → clean.

**The Ruby extension on disk was stale and has just been rebuilt.** Before `rake compile`,
`IsccLib.text_clean("a" + U+A7F1 + "b")` returned `"aSb"` (pre-freeze-rule); after it, `"ab"` — i.e.
Ruby now passes the discriminating vector. `crates/iscc-rb/lib/iscc_lib/iscc_rb.so` is gitignored
(`crates/iscc-rb/.gitignore:3`), so recompiling leaves **no** tree diff. Run
`bundle exec rake compile` before `rake test` anyway (CI does the same), and re-probe that one
discriminating value if anything looks odd. Note: the U+0378 rows do **not** discriminate a stale
build (U+0378 is `Cn` in every Unicode version), only the U+A7F1 rows do.

**WASM needs no artifact management** — `wasm-pack test` compiles the current core every run.

### WASM test (`crates/iscc-wasm/tests/unicode_boundary.rs`)

- Load the canonical fixture at compile time, no copy:
    `const BOUNDARY_JSON: &str = include_str!("../../iscc-lib/tests/unicode_boundary.json");` (same
    relative shape `conformance.rs` uses for `data.json`).
- Parse with `serde_json` (already a dev-dependency) into a `serde_json::Value`; iterate
    `value["text_clean"].as_object().unwrap()`.
- Do **not** add `#[cfg(feature = "conformance")]`. That feature only gates the
    `conformance_selftest` test in `unit.rs`; these vectors need no core feature. Mirror
    `conformance.rs`, which is ungated.
- Three `#[wasm_bindgen_test]` functions: one per section looping all cases, plus a metadata guard
    asserting `unicode_data_version == "16.0.0"` and per-section counts 7 and 5 (without that guard
    a truncated fixture silently degrades to a zero-iteration loop that still reports success).
    `wasm-bindgen-test` has no parametrization — loop inside one test and pass the case name into
    the `assert_eq!` message so a failure identifies its vector.
- The file must also compile for the **host** target: CI runs
    `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` and
    `cargo test --workspace --exclude iscc-rb`. On host these tests compile but run as **0 tests**
    (measured: `cargo test -p iscc-wasm` reports 0 for every target today) — that is expected, not a
    problem to fix. `wasm-pack test --node` is the only runner that executes them.

### Ruby test (`crates/iscc-rb/test/test_unicode_boundary.rb`)

- `# frozen_string_literal: true` header, module docstring comment, then `require "test_helper"` +
    `require "json"`, mirroring `test_conformance.rb`.
- Path constant:
    `BOUNDARY_JSON = File.expand_path("../../iscc-lib/tests/unicode_boundary.json", __dir__)`. **Use
    fresh constant names** (e.g. `BOUNDARY_JSON` / `BOUNDARY_DATA`): `rake test` loads every test
    file into one process and `test_conformance.rb` already defines top-level `DATA_JSON` and
    `CONFORMANCE_DATA`, so reusing those names triggers "already initialized constant" warnings.
- `class TestUnicodeBoundary < Minitest::Test` with a `define_method` per fixture case (matching
    `test_conformance.rb`), so each vector is its own named test:
    `assert_equal tc["outputs"]["result"], IsccLib.text_clean(*tc["inputs"])`. Same for
    `text_collapse`. Add the metadata guard test (version + counts 7/5) as a normal `def test_…`.
- Only read strings from the fixture — never type a non-ASCII or `\u`-escaped literal into the test
    file. `JSON.parse(File.read(...))` yields UTF-8 strings; `assert_equal` compares them directly.
- `standardrb` is CI-enforced but is **not** a prek hook, so run `bundle exec standardrb` explicitly
    (`--fix` is available for mechanical style).

### Docs

- `docs/unicode.md:99-101` — the sentence "exercised by the Rust test suite, by the Python test
    suite … and by the pure-Go package via the vendored copy" should also name the **WASM** and
    **Ruby** binding suites, both of which read the canonical fixture directly. Keep the pure-Go
    admonition and its three-skip wording as-is.
- `crates/iscc-wasm/CLAUDE.md` — the `tests/` block near line 22 lists `conformance.rs` and
    `unit.rs`; add a one-line `unicode_boundary.rs` entry.
- `crates/iscc-rb/CLAUDE.md` — add a `test/test_unicode_boundary.rb` row to the file table next to
    `test/test_conformance.rb`.
- Do not add a docs page — `scripts/check_docs_nav.py` must stay at 23 pages.

## Verification

- `wasm-pack test --node crates/iscc-wasm --features conformance` — exit 0, `0 failed` in every
    target (baseline for comparison: `unit.rs` 78 passed before this step). Per-target narrowing, if
    it works, is `… --features conformance -- --test unicode_boundary`; the full run is the
    authoritative criterion.
- The `unicode_boundary` WASM target's `test result:` line shows `ok` with **at least 3 passed, 0
    failed**.
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — exit 0 (new wasm test
    target compiles on host) and `cargo clippy -p iscc-rb -- -D warnings` — exit 0.
- `cargo fmt --all --check` — exit 0.
- In `crates/iscc-rb`: `bundle exec rake compile` — exit 0; then `bundle exec rake test` — exit 0
    reporting **at least 123 runs** (111 baseline + 12 vectors) and `0 failures, 0 errors, 0 skips`.
- In `crates/iscc-rb`: `bundle exec standardrb` — exit 0, no output.
- No skip construct in either new file:
    `grep -n '#\[ignore\]' crates/iscc-wasm/tests/unicode_boundary.rs` finds nothing, and
    `grep -nE '(^|[^_a-z])skip[ (]' crates/iscc-rb/test/test_unicode_boundary.rb` finds nothing
    (both exit 1).
- `find . -name unicode_boundary.json -not -path './target/*' -not -path './.git/*'` — exactly
    **two** paths (`crates/iscc-lib/tests/…` and `packages/go/testdata/…`); this slice vendors no
    new copy.
- `git status --porcelain crates/iscc-lib/ packages/go/ .crap-baseline.json .iai-baseline.json` —
    empty (canonical fixture, Go package and both baselines untouched).
- `cargo test -p iscc-lib` — 336 passed, 0 failed (unchanged).
- `mise run check` — every prek hook passes, exit 0, no reformats of the new files.
- `uv run zensical build` — exit 0, "No issues found"; `uv run scripts/check_docs_nav.py` — exit 0,
    23 pages.
- `grep -F -c 'unicode_boundary.rs' crates/iscc-wasm/CLAUDE.md` ≥ 1;
    `grep -F -c 'test_unicode_boundary.rb' crates/iscc-rb/CLAUDE.md` ≥ 1; `docs/unicode.md` names
    both the WASM and the Ruby suite in its boundary-vector paragraph.

## Done When

The WASM and Ruby suites each run all 12 Unicode boundary vectors green with no skips against the
canonical fixture, the three docs files name them, and every verification command above passes.
