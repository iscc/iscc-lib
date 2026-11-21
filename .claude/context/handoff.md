## 2026-11-21 — Review of: Register the Rust docs and the Python API page with the Tier 1 doc gate

**Verdict:** PASS

**Summary:** `docs/rust-api.md`, `docs/howto/rust.md` and `docs/architecture.md` → "Tier 1" are
complete and registered `FULL`; `docs/api.md` is `EXEMPT`. The false claims are gone: no page says a
gen function returns a string, and `api.md` no longer says Python returns JSON. Every claim the
handoff made held up when re-measured. Diff touches the registry tuple and four docs only.

**Verification:**

- [x] `uv run scripts/check_tier1_docs.py` exits 0 — `OK: 19 full and 2 exact Tier 1 docs match`
- [x] Registry assertion one-liner exits 0
- [x] `uv run pytest tests/test_check_tier1_docs.py` passes — 36 passed
- [x] `grep -n "json.loads" docs/api.md` prints nothing; the only `IsccResult<String>` hits are the
    `json_to_data_url` (rust-api:497) and `encode_component` (rust-api:559) signatures;
    `architecture.md` has none
- [x] Every return type in a `rust-api.md` signature equals the source — own script: all 39 printed
    `pub fn` / `pub const` items occur in `crates/iscc-lib/src/*.rs` (whitespace-insensitive; the
    one difference is `finalize(self` for the source's `finalize(mut self`, as rustdoc renders it)
- [x] `uv run zensical build` exits 0 and `mise run check` exits 0

Probes beyond next.md (3):

- All 51 Rust example blocks of `rust-api.md` + `howto/rust.md` compiled and run in a `/tmp` cargo
    project with a path dependency: 48 pass, 3 fail only on the placeholder files (`document.pdf`,
    `large_file.bin`), as the handoff said
- The 9 Rust examples of `README.md`, `crates/iscc-lib/README.md`, `docs/index.md` and the tutorial
    compile too (one placeholder-file failure): no sibling page still treats a result as a string
- Python: every `gen_*` function of `iscc_lib` returns a `dict` subclass with `r["iscc"] == r.iscc`

**Reviewer fixes (step 9):**

- `tests/test_rust_api_signatures.py` (5 tests): pins every item printed in `docs/rust-api.md` to
    the crate source and requires a printed signature for each crate-root `gen_*` function. Proven
    against the page's previous version (10 of 13 items stale). `ty` and ruff `S`/`C901` green
- `docs/rust-api.md`: "The three `META_TRIM_*` constants" → no count (next.md banned new counts)
- `docs/architecture.md`: "Tier 1 is everything `lib.rs` makes public at the crate root" → "the
    functions, constants and streaming types that `lib.rs` exports": `pub mod codec` is public at
    the crate root as well, and the next subsection calls it Tier 2

**Issues found:**

- Filed `normal` `[review]`: "The minimal-build snippet pins `iscc-lib` to 0.4" —
    `docs/howto/rust.md` and `crates/iscc-lib/README.md` show `version = "0.4"`, which resolves to
    the published 0.4.0; that release lacks `encode_seq` and `gen_iscc_id_v1` (measured). Not
    touched by this step
- Rewrote the "Derived facts" issue: of its docs part only `docs/howto/c-cpp.md` is left (measured:
    it is the one how-to / API reference outside the registry)
- Not filed: `rust-api.md` → "Types" says the `codec` module provides the types, but `IsccError`
    lives in `lib.rs`; the new sections have no `---` separators while the gen sections do;
    `howto/rust.md` keeps its old "All 10 `gen_*_v0` functions"

**Codex review:** No actionable findings. It ran the gate, the 36 tests and the 51 Rust examples
(with fixture files supplied for the file-reading ones) and all passed.

**Next:** `docs/howto/c-cpp.md`: complete it and register it `FULL`, or `EXEMPT` with a reason
(sources `iscc.h` / `iscc.hpp`; which one depends on `c-ffi-dx.md` / `cpp-bindings.md`). The
two-line 0.4 snippet fix is small enough to weigh alongside it. After that, part 3 of the "Derived
facts" issue (counts, file lists and line numbers in specs and per-crate `CLAUDE.md`).

**Notes:**

- The new signature test covers `docs/rust-api.md` only. `howto/rust.md` prints a few signatures
    inside bold bullets ("Algorithm primitives"); nothing checks those
- A `zensical build` run while Codex was building printed "page does not exist" for the new
    `rust-api.md` link; a sequential re-run is clean and the built link is `../rust-api/`. It was a
    race on `site/`, not a broken link
- No Rust source changed: CRAP, iai and semver gates are unaffected
