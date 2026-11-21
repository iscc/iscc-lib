## 2026-11-21 — Review of: Make the Tier 1 doc gate fail closed on crate-root items it does not parse

**Verdict:** PASS

**Summary:** `scripts/check_tier1_docs.py` now classifies every column-0 `pub` item of `lib.rs`
through one scanner (`scan_lib`): it derives symbols, matches a named `EXCLUDED_ITEMS` entry, or
fails the gate with one `lib.rs:<line>: unrecognised crate-root item` error before the floor and
before any doc is read. The diff is the script, its tests and one sentence in `docs/development.md`;
no existing test changed. The second part of the `normal` issue "Derived facts…" is closed and the
entry now describes the two parts that are left.

**Verification:**

- [x] `uv run pytest tests/test_check_tier1_docs.py` passes with a parametrised fire test, one form
    per `run_checks` call, full error list asserted — 36 passed; all 8 forms from next.md are in
    `UNRECOGNISED_FORMS` (plus 5 more)
- [x] A test asserts the real `lib.rs` scan has zero unrecognised items, and each excluded form of
    `SYNTHETIC_LIB` is recognised without deriving a symbol — `test_real_lib_rs_has_no_…`,
    `test_excluded_form_is_recognised_without_symbol` (each form scanned alone, 5 params)
- [x] `uv run scripts/check_tier1_docs.py` exits 0 — `OK: 16 full and 2 exact Tier 1 docs match …`
- [x] `uv run ty check` exits 0 and `mise run check` exits 0 (ruff `S`/`C901` on both files clean)

Reviewer probes (3): the derived set on the real `lib.rs` equals the previous script's (35 symbols,
43 column-0 `pub` starts, 0 unrecognised); a sweep of 35 further forms (`pub trait`, `pub union`,
`pub extern "C" fn`, inline `pub mod x {`, `pub use foo;`, `pub use ::a::b;`, a comment inside a
group, `pub enum IsccErrorKind`, `pub use crate::types::*;`, CRLF) found no fail-open case; five
mutants in a `git archive` copy (accept `self`, drop the `:` / `;` terminators, drop the early
return, kind-wide glob exclusion) were each killed by exactly the expected tests.

**Issues found:**

- (none filed). Nits, not worth an entry: `IDENT_RE` and `TOKEN_RE` are the same regex;
    `pub const _: () = …;` derives the symbol `_` (as before this step; the `EXACT` docs then fail,
    so it is fail-closed); a `#[macro_export] macro_rules!` is public at the crate root without
    `pub` and is outside what the gate scans (the crate has none)

**Codex review:** no actionable findings ("The fail-closed parser matches the intended scope").

**Next:** Part 1 of the same `normal` issue: bring `docs/howto/rust.md`, `docs/howto/c-cpp.md`,
`docs/rust-api.md`, `docs/architecture.md` → "Tier 1" and `docs/api.md` under the gate — complete
each and add it to `FULL`, or add it to `EXEMPT` with a reason — and correct the
`IsccResult<String>` claim in `docs/rust-api.md`. If five docs are too much for one step, split by
doc, not by symbol. Part 3 (unchecked counts, file lists and line numbers in the specs and the
per-crate `CLAUDE.md` files) follows.

**Notes:**

- The gate checks NAMES only. A return type or signature a doc adds in the next step must be run or
    read from `lib.rs` / the binding source, not copied from a sibling doc (iteration 190 fabricated
    a per-surface return type that way; 233 review took ~25 min for the same reason)
- `EXEMPT` is for a doc that enumerates the API partially on purpose; a doc that is merely
    incomplete gets completed and goes to `FULL`
- A new crate-root form in `lib.rs` now needs a derive rule or a named `EXCLUDED_ITEMS` entry in the
    same step, or the prek hook and the pytest anchor both fail
- No Rust source changed in this step: CRAP, iai and semver gates are unaffected
