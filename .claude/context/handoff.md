## 2026-11-21 — Make the Tier 1 doc gate fail closed on crate-root items it does not parse

**Done:** `scripts/check_tier1_docs.py` now classifies every column-0 `pub` start in `lib.rs` (by
offset, `pattern.match(text, pos)`): it either derives symbols (`pub fn`, `pub const fn`,
`pub const`, `pub use path::name;`, `pub use path::{bare, idents,}`) or matches the named
`EXCLUDED_ITEMS` table (`pub(`, `pub mod NAME;`, `pub use types::*;`, `pub enum IsccError`,
`pub type IsccResult`). Anything else returns one `lib.rs:<line>: unrecognised crate-root item:`
error per item from `run_checks`, before the floor and before any doc is read.

**Files changed:**

- `scripts/check_tier1_docs.py`: `scan_lib` → `(symbols, unrecognised)` is the single classifier;
    `derive_symbols` is a thin wrapper (signature unchanged). Helpers `item_symbols`,
    `group_members`, `is_excluded`. Derive regexes end at a terminator (`[<(]` after a fn name,
    `\s*:` after a const name, `;` after a use). `EXCLUDED_ITEMS` dict (pattern → reason) sits next
    to the registry. Module docstring "Source of truth" paragraph rewritten.
- `tests/test_check_tier1_docs.py`: parametrised fire test (13 forms, one per `run_checks` call,
    full error list asserted incl. line number); real `lib.rs` has zero unrecognised; each
    `SYNTHETIC_LIB` excluded form scanned ALONE → `(set(), [])` (no vacuous pass); `SYNTHETIC_LIB`
    has no unrecognised; `group_members` rejects `b as c`, `self`, `b::c`, `*`, empty, `a,,b`.
- `docs/development.md`: one sentence in the `check_tier1_docs.py` bullet (mdformat re-wrapped it).

**Verification:**

- `uv run pytest tests/test_check_tier1_docs.py -q` → 36 passed (15 existing unchanged + 21 new)
- `uv run scripts/check_tier1_docs.py` → `OK: 16 full and 2 exact Tier 1 docs match …`
- Same test file against HEAD's script (`git archive` copy in /tmp): all 13 fire params red, i.e.
    the HEAD gate passed every form
- Mutant: `EXCLUDED_ITEMS` `pub enum IsccError\b` → kind-wide `pub enum` → exactly
    `[pub enum Other {}]` turns red
- Probe at HEAD: `derive_symbols("pub const unsafe fn h() {}")` returned `{"unsafe"}` (junk name)
- `uv run ty check` → All checks passed; `mise run check` → exit 0 (second run; the first one only
    re-wrapped `docs/development.md` via mdformat); pre-push hooks on the three files → all Passed
    (clippy, cargo test, ty, ruff S, C901, pytest)

**Next:** Part 1 of the same issue: register (and complete) `docs/howto/rust.md`,
`docs/howto/c-cpp.md`, `docs/rust-api.md`, `docs/architecture.md`, `docs/api.md`, plus the
`IsccResult<String>` claim in `rust-api.md`. Part 3 (counts in specs / per-crate `CLAUDE.md`) after.

**Notes:**

- Beyond next.md's 8 fire forms I added 5: `pub use a::{self, b};`, `pub use a::{b::c, d};`,
    `pub const unsafe fn h() {}` (the junk-name case above), `pub enum Other {}` and
    `pub type Other = u8;` (the last two pin "named, not kind-wide" exclusions)
- One trailing empty group member is allowed (`{b,}`, rustfmt multi-line form); `{}` and `{a,,b}`
    are unrecognised
- `is_excluded` compiles the five patterns per call; trivial at ~40 `pub` items
- No Rust source changed: CRAP, iai, semver unaffected
