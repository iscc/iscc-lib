# Next Work Package

## Step: Tier 1 doc-parity check derived from the `iscc-lib` crate root

## Goal

Part 1 of the `normal` `[human]` issue "Derived facts in guidance and docs are written by hand and
drift": a pre-commit + pytest gate that fails when a doc enumerating the Tier 1 API disagrees with
the public API at the `iscc-lib` crate root. It lands green, so the drift it finds today is fixed in
the same step.

## Alternatives Considered

- **Chosen:** the parity check. The issue and handoff order it first, because it decides which lists
    stay. Today's drift shows it is needed: `docs/java-api.md` says "All 35 Tier 1 symbols" but
    documents neither `genSumCodeV0` nor `META_TRIM_META`, though both exist in `IsccLib.java`
- **Rejected:** the counts/file-lists/line-numbers sweep of specs and per-crate `CLAUDE.md` (part
    2). It comes after the check, which settles which enumerations are kept under a gate

## Scope

- **Create**: `scripts/check_tier1_docs.py`, `tests/test_check_tier1_docs.py`
- **Modify**: `.pre-commit-config.yaml` (new hook); docs (do not count against the cap):
    `docs/java-api.md`, `docs/howto/java.md`, `docs/howto/go.md`, `packages/go/README.md`,
    `docs/c-ffi-api.md`, `docs/howto/python.md`, `docs/howto/nodejs.md`, `docs/howto/wasm.md`,
    `.claude/context/specs/rust-core.md` + `crates/iscc-lib/CLAUDE.md` (markers only),
    `docs/development.md` (gate list), plus any other registered file the script flags
- **Reference**: `scripts/check_spec_paths.py` + `tests/test_check_spec_paths.py` (script/test
    shape, `importlib` loading), `crates/iscc-lib/src/lib.rs` lines 18–47 + top-level `pub fn`s,
    `docs/howto/kotlin.md` → "Algorithm primitives" (section to mirror)

## Not In Scope

- Part 2 of the issue (removing unchecked counts, file lists and line numbers). This includes count
    phrases such as "All 35 Tier 1 symbols": leave them as they are, the sweep decides their wording
- The per-crate READMEs: target.md asks only for "the 9 `gen_*_v0` functions and key utilities", so
    they are partial by design and stay below the discovery threshold
- `.claude/context/target.md`: human-owned and it enumerates by category count. Register it as
    exempt and never edit it
- `docs/architecture.md` → "Tier 1": it lists only the 10 gen functions, with stale
    `IsccResult<String>` signatures, so the threshold does not catch it. Leave it for the sweep
- Any binding code change. Every symbol found missing is a doc gap, not an API gap (verified below)

## Implementation Notes

**Source of truth, parsed from `lib.rs` and never typed out:** column-0 `pub fn NAME`,
`pub const NAME` and named `pub use path::{a, b}` / `pub use path::name` items. Exclude the
`types::*` glob, `pub mod`, `pub enum IsccError` and the result structs. Today this gives 15 + 5 +
15\. Assert a floor on the derived set (e.g. it contains `gen_meta_code_v0` and `DataHasher`) so a
parse collapse to an empty set cannot pass.

**Matching a symbol in a doc:** lowercase each identifier token and drop `_`, so that
`genSumCodeV0`, `GenSumCodeV0`, `gen_sum_code_v0` and `META_TRIM_META`/`MetaTrimMeta` all match.
Also accept a leading `iscc` C prefix (`iscc_meta_trim_meta`). Streaming types additionally match a
token that starts or ends with the type name (`NewDataHasher`, `iscc_data_hasher_new`). Never use
plain substring matching: `isccdecode` occurs inside `IsccDecodeResult`.

**A registry in the script, with fail-closed discovery:**

- `FULL`: files that must name every Tier 1 symbol (one-way check, any casing)
- `EXACT`: `rust-core.md` → "Complete Tier 1 API Summary" table and the `crates/iscc-lib/CLAUDE.md`
    Tier 1 list. Wrap each in `<!-- tier1:begin -->` / `<!-- tier1:end -->` comment lines. The set
    of backticked identifiers between the markers must EQUAL the derived set: extras fail too
- `EXEMPT`: path → reason (`target.md`, and any other file you justify)
- Discovery: every tracked `*.md` (via `git ls-files`, as `check_spec_paths.py` does; skip `notes/`,
    `.claude/agent-memory/` and the context logs) that names ≥ 30 distinct Tier 1 symbols must be in
    one of the three sets. Otherwise the gate fails and names the file
- Errors name the file and the missing or extra symbols. Exit 1 on any finding

**Measured gaps at HEAD** (from a read-only probe at threshold 30; re-measure with the real script):

- `META_TRIM_META` is missing from `docs/java-api.md`, `docs/howto/java.md`, `docs/howto/go.md`,
    `packages/go/README.md`, `docs/c-ffi-api.md` (`iscc_meta_trim_meta`, `lib.rs:58` of iscc-ffi)
    and from the python/nodejs/wasm how-tos. It is exposed on every surface
- `genSumCodeV0` is missing from `docs/java-api.md` (`IsccLib.java:161`)
- `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks` and `alg_simhash` are missing from
    `docs/howto/{python,nodejs,wasm}.md`. Py `__all__`, napi and WASM export all four in snake_case.
    Add an "Algorithm primitives" section modelled on kotlin.md
- `crates/iscc-ffi/CLAUDE.md` (~32 hits) and `packages/go/CLAUDE.md` (~30 hits, no constants) cross
    the threshold: complete them as `FULL`, or `EXEMPT` them with a reason

**Hook** (copy the `check-spec-paths` shape): `uv run scripts/check_tier1_docs.py`,
`pass_filenames: false`,
`files: ^(crates/iscc-lib/src/lib\.rs|.*\.md|scripts/check_tier1_docs\.py)$`. The script must run on
Python 3.10 (no `tomllib`). Test cases: an anchor test on the real tree, plus fire tests with one
bad case per call on `tmp_path` copies: a `pub fn` added to a copied `lib.rs`, a symbol deleted from
a FULL doc, an extra backticked name inside an EXACT block, and an unregistered doc over the
threshold.

## Verification

- `uv run scripts/check_tier1_docs.py` exits 0 on the working tree
- `uv run pytest tests/test_check_tier1_docs.py` passes, including the 4 fire tests (each asserts
    exit/finding on exactly one injected defect)
- `grep -c 'tier1:begin' .claude/context/specs/rust-core.md crates/iscc-lib/CLAUDE.md` prints a
    non-zero count for both files
- `.pre-commit-config.yaml` has a hook whose `entry` runs `scripts/check_tier1_docs.py` and whose
    `files:` pattern matches `crates/iscc-lib/src/lib.rs`
- `uv run zensical build` exits 0, and `mise run check` exits 0

## Done When

The advance agent is done when the gate passes on the tree, its fire tests prove it catches each
defect class, and all verification commands above exit 0.
