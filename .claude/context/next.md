# Next Work Package

## Step: Classify every public `iscc-lib` path in one tier list and fix the false Rust API sentences

## Goal

Pick up the `[audit]` issue "Rust public API docs misclassify `IsccError`, omit `SumHasher`, and
contradict `gen_sum_code_v0`": every public path of `iscc-lib` gets a tier, and the cited false
sentences match `lib.rs`. These pages tell Rust users and agents what SemVer covers before v1.0.0.

## Alternatives Considered

- **Chosen:** tier docs. It is the handoff's Next, it needs no change in behaviour, and the one
    design call (where `SumHasher` belongs) follows from the specs (see Implementation Notes)
- **Rejected:** one-shot/streaming dedup (`normal`, 3 files). It changes the Rust hot path and needs
    the CI-only iai/CRAP gates. Better as the next step, not one stacked on unpushed commits
- **Not candidates:** `alg_minhash_256([])` and the WASM `finalize()` rule both need a behaviour
    decision from Titusz. The duplicate `data.json` parsers come after the dedup

## Scope

- **Modify**: `docs/architecture.md`, `docs/rust-api.md`, `CLAUDE.md` (root),
    `crates/iscc-lib/CLAUDE.md`, `crates/iscc-lib/src/lib.rs` (crate doc comment only),
    `.claude/context/specs/rust-core.md` (Tier 2 paragraph and the API-compatibility bullet)
- **Reference**: `crates/iscc-lib/src/streaming.rs` (`SumHasher`, line 157 on),
    `scripts/check_tier1_docs.py` (`EXCLUDED_ITEMS`), `tests/test_rust_api_signatures.py`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java:2`

Rule: docs only. The `lib.rs` edit is a `//!` comment, and comment-only edits do not count against
the cap. No code file changes.

## Not In Scope

- Making `SumHasher` Tier 1 (a crate-root re-export or binding it on the other 9 surfaces). That
    would change target.md's count of 35, which only Titusz can do
- Editing `target.md`: its Tier 2 line names only `codec`. If it is now incomplete, review files a
    `low` issue for Titusz
- Changing visibility (`pub(crate)`, `#[doc(hidden)]`) of `streaming`, `types` or `SumHasher`
- The one-shot dedup (`gen_sum_code_v0` in WASM copying `SumHasher`), and the version-sync lists in
    `docs/development.md` (separate `low` issue)
- `notes/`: it is the design record and is not swept

## Implementation Notes

**The classification** (the design call; review records it in decisions.md):

- `iscc_lib::streaming::SumHasher` → **Tier 2**: public Rust API, covered by the SemVer and
    `cargo-semver-checks` contract, and outside the every-surface Tier 1 contract. Python and WASM
    wrap it as an extra. It cannot be Tier 1 without the target.md change above
- `iscc_lib::types` → its `*CodeResult` structs are the Tier 1 return types, re-exported at the
    crate root by `pub use types::*`. The `types::X` path is the same item, not a separate tier
- `IsccError` / `IsccResult` → crate-root support types of Tier 1 (the error every Tier 1 call
    returns). They are not counted among the 35 symbols, as `EXCLUDED_ITEMS` in
    `check_tier1_docs.py` already says. They are not codec types
- `DataHasher` / `InstanceHasher` stay Tier 1 (crate-root re-exports). Their `streaming::` path is
    the same item

**Sentences to fix** (from the issue, re-derived on HEAD):

- `docs/architecture.md` "Tier 2 — Public Rust API" (around line 141): name `codec` and
    `streaming::SumHasher`. Say that Python and WASM wrap `SumHasher` (so do not write "not exposed
    through FFI bindings" for it). Add a line on `types` and `IsccError`
- `docs/architecture.md:163` "takes bytes, not file paths": `gen_sum_code_v0` takes a path. State
    the rule with that one exception
- `docs/rust-api.md` "## Types" (around 679): the note says to import `IsccError` from `codec`. Move
    `IsccError` out of the codec framing (it is `iscc_lib::IsccError`). Add `SumHasher` to "##
    Streaming" as Tier 2 at its `iscc_lib::streaming::SumHasher` path, with its `impl` block copied
    from `streaming.rs` (`finalize(self, bits: u32, wide: bool, add_units: bool)`).
    `tests/test_rust_api_signatures.py` checks every printed `pub fn` against the source
- Root `CLAUDE.md:154` `iscc_lib::api`: there is no such module. The bound surface is the crate-root
    re-exports of `lib.rs`. `CLAUDE.md:217`: 9 of the 10 `gen_*_v0` functions have `data.json`
    vectors (`gen_sum_code_v0` has none). Do not make a blanket 9→10 change
- `crates/iscc-lib/src/lib.rs:3-5`: Tier 1 is the crate-root symbols (35), not "all 10 `gen_*_v0`
    functions"
- `crates/iscc-lib/CLAUDE.md` Tier 2 block (line 40): add `streaming::SumHasher` and list
    `IsccError`/`IsccResult`. `rust-core.md:699` and the API-compatibility bullet (line 725): name
    `streaming::SumHasher` beside `codec`
- `IsccLibTest.java:2` claims data.json vectors for all 10 gen functions. Fix the comment if the
    file has no `gen_sum_code_v0` vectors (it is a comment-only edit)

Sweep with a repo-wide `git grep` before you finish. Key phrases: `iscc_lib::api`,
`bytes, not file paths`, `Import codec types`, `only Tier 2`, and "All 10". The "All 10 … follow the
same pattern" lines in the how-to guides describe calling conventions and are correct. Check that
every Rust example you add compiles (in a `/tmp` cargo project with a path dep).

No Rust code changes, so the CRAP, iai and semver gates are unaffected.

## Verification

- `git grep -n 'iscc_lib::api' -- ':!notes' ':!.claude'` prints nothing
- `git grep -n -e 'bytes, not file paths' -e 'Import codec types' -- docs CLAUDE.md` prints nothing
- `git grep -n 'SumHasher' -- docs/architecture.md docs/rust-api.md crates/iscc-lib/CLAUDE.md .claude/context/specs/rust-core.md`
    prints a match in each of the four files
- `uv run pytest tests/test_rust_api_signatures.py` passes, and
    `uv run python scripts/check_tier1_docs.py` exits 0
- `uv run zensical build` exits 0, and `mise run check` exits 0

## Done When

Every public `iscc-lib` path is classified in one tier list, and all checks above pass.
