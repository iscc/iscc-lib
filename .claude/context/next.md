# Next Work Package

## Step: Make the Tier 1 doc gate fail closed on crate-root items it does not parse

## Goal

Second part of the `normal` issue "Derived facts in guidance and docs are written by hand and
drift": every column-0 `pub` item in `crates/iscc-lib/src/lib.rs` is either derived as a Tier 1
symbol or matched by an explicit exclusion, and any other form fails `scripts/check_tier1_docs.py`
(today it prints `OK` for all six forms the issue names).

## Alternatives Considered

- **Chosen:** fail-closed `lib.rs` parse — one script plus its tests; the issue's smallest
    self-contained part, and the precondition for trusting the derived set that the other two parts
    check docs against.
- **Rejected:** register `docs/howto/rust.md`, `docs/howto/c-cpp.md`, `docs/rust-api.md`,
    `docs/architecture.md` (and `docs/api.md`) — they miss 9 to 22 symbols each, so it is a doc
    writing step of its own; next. Part 3 (unchecked counts in specs and per-crate `CLAUDE.md`)
    after.

## Scope

- **Modify**: `scripts/check_tier1_docs.py` (logic + module docstring "Source of truth" paragraph),
    `docs/development.md` (the `check_tier1_docs.py` bullet, line ~163: one sentence that an
    unrecognised crate-root item fails the gate)
- **Tests**: `tests/test_check_tier1_docs.py`
- **Reference**: `crates/iscc-lib/src/lib.rs` (lines 1-60 hold every non-`fn` crate-root item),
    decisions.md 2026-11-20 "The Tier 1 doc gate matches names against a registry…" (settled — do
    not change the registry, matching rules or threshold)

## Not In Scope

- Registering or completing any doc (`rust.md`, `c-cpp.md`, `rust-api.md`, `architecture.md`,
    `api.md`) and the `IsccResult<String>` claim in `rust-api.md` — next step
- Deriving an alias from `pub use x as y` — reject it as unrecognised; a human decides then
- Attribute-on-the-same-line forms (`#[cfg(x)] pub fn`), `pub` text inside macros/strings — rustfmt
    never emits them at column 0
- Editing existing tests' expectations — every current test must pass unchanged (in particular
    `test_floor_rejects_collapsed_parse` and `SYNTHETIC_LIB`)
- Changing `FULL` / `EXACT` / `EXEMPT` / `DISCOVERY_THRESHOLD` or the matching functions

## Implementation Notes

- **Recognise by offset, not by line.** Find every `^pub\b` start (MULTILINE) in `lib.rs`; each
    start must be the start of a derive match or of an exclusion match
    (`re.match(pattern, text, pos)` / `pattern.match(text, pos)`). A line-based classifier would
    reject the multi-line `pub use streaming::{\n    DataHasher,\n};` in `SYNTHETIC_LIB`.
- **Derive forms (unchanged meaning):** `pub fn NAME`, `pub const fn NAME`, `pub const NAME`,
    `pub use path::name;`, `pub use path::{a, b};` (multi-line allowed, trailing comma allowed).
- **Group members must be bare identifiers.** Today `pub use a::{b as c, d};` derives the junk name
    `"b as c"`; `{self, b}` and `{b::c, d}` derive junk likewise. Any member that is not
    `[A-Za-z_]\w*`, and any `self` or `*` member, makes the item unrecognised — same parser, same
    pattern.
- **Explicit exclusions, named items only** (a kind-wide exclusion would let a new
    `pub use codec::*;` add Tier 1 symbols silently): `pub(...)` restricted visibility (blanket —
    not public at the root), `pub mod NAME;` (generic — a module is not a symbol), and exactly
    `pub use types::*;`, `pub enum IsccError`, `pub type IsccResult`. Keep them in one named
    module-level table next to the registry so adding one is a recorded choice. `SYNTHETIC_LIB`
    already uses only these forms.
- **One classifier, one source**: `derive_symbols` must not keep a second, independent regex that
    can disagree with the recognition check. Suggested shape: a scan returning
    `(symbols, unrecognised)`, with `derive_symbols` a thin wrapper (its signature and current
    results unchanged — tests call it).
- **Early return in `run_checks`**, before the floor and before any doc is read: one error per
    unrecognised item, e.g. `f"{LIB_RS}:{line}: unrecognised crate-root item: {first_line}"`, so a
    fire test can assert the complete list.
- Measured at HEAD (read-only probe): all six issue forms plus `pub use codec::*;` derive nothing
    and the gate passes; `pub use a::{b as c, d};` derives `{"b as c", "d"}`.
- Gates: script + `tests/` only → no Rust source changes; trips neither CRAP nor iai.

## Verification

- `uv run pytest tests/test_check_tier1_docs.py` passes, with a parametrised fire test that adds ONE
    form per `run_checks` call at column 0 of a copied `lib.rs` — `pub use codec::x as y;`,
    `pub use a::{b::{c}};`, `pub struct S;`, `pub static X: u8 = 1;`, `pub unsafe fn f() {}`,
    `pub async fn g() {}`, `pub use codec::*;`, `pub use a::{b as c, d};` — and asserts the full
    error list equals that single unrecognised-item error
- A test asserts the real `lib.rs` scan has zero unrecognised items and the excluded forms in
    `SYNTHETIC_LIB` (`pub(crate) mod`, `pub mod`, `pub enum IsccError`, `pub type IsccResult`,
    `pub use types::*;`) are recognised without deriving a symbol
- `uv run scripts/check_tier1_docs.py` exits 0 on the real tree
- `uv run ty check` exits 0 and `mise run check` exits 0

## Done When

All verification criteria pass, and the gate rejects every column-0 `pub` form in `lib.rs` that it
neither derives nor explicitly excludes.
