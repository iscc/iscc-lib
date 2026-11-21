## 2026-11-21 — Review of: Give the Ruby source gem the workspace release profile

**Verdict:** PASS

**Summary:** `scripts/build_ruby_source_gem.py` now copies the root `[profile.release]` key lines
into the staged standalone `Cargo.toml` (`release_profile`, a text slice with no `tomllib`), and
fails closed on a missing, duplicated or empty table and on any `[profile.release.<x>]` sub-table. A
source install on Ruby 3.4.13 builds a stripped extension that passes the selftest. The diff stays
inside next.md's scope (one script, its test file, one sentence in `crates/iscc-rb/CLAUDE.md`), adds
no suppression and touches no gate. The issue "The Ruby source gem is built without the workspace
release profile" is resolved and deleted.

**Verification:**

- [x] `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py` passes,
    including the new tests — 38 passed (6 new; next.md asked for 4, the two extra split the failure
    arms one per fixture)
- [x] `uv run --script scripts/build_ruby_source_gem.py --out …` exits 0; the unpacked gem's
    `Cargo.toml` has `lto = true`, `codegen-units = 1`, `strip = true` and `panic = "abort"` under
    `[profile.release]` and no `debug = true` (nor `strip = false`)
- [x] Ruby 3.4.13, from `/tmp`, empty `GEM_HOME`/`GEM_PATH`: `gem install` exit 0 (52 s, 3 gems),
    `conformance_selftest` exit 0
- [x] `file` on the installed `iscc_rb.so` has no `not stripped` — two copies, both `stripped`,
    1,049,536 bytes each (1,521,056 unstripped in iteration 240)
- [x] `mise run check` exits 0

Probes beyond next.md (2 of 3 used):

- `gem_make.out` in the gem home shows rb_sys calling `cargo rustc … --profile release`, so the
    staged table is the one in effect. No `.cargo/config.toml` or other profile source exists in the
    repository, so the source gem and the platform gems now build with the same profile
- Six mutants of `release_profile`, run from a `git archive` copy, each turn the matching test red:
    no stop at the next header, a retyped (hard-coded) table, a dropped key, and each of the three
    fail-closed arms switched off. The issue's "a test fails when a key is missing" holds

**Issues found:**

- Nit, not filed: the "duplicated table" half of the `expected one` arm has no test. A duplicated
    `[profile.release]` is invalid TOML, and `cargo metadata` fails on it before the script reads
    the file
- Nit, not filed (advance named it): `TOML_HEADER` reads a line that is only `["x"]` inside a
    multi-line array as a header. The root table has no multi-line value. If one is added, the slice
    ends inside the array, the staged manifest is invalid TOML, and the `ruby-source-gem` CI job
    fails at install
- No new issues.md entries

**Codex review:** no actionable findings. Codex ran the 38 tests and an offline source-gem build
itself and confirmed the packaged manifest carries the release profile without the bench keys.

**Next:** One `normal` v0.8.0 blocker is left: "Derived facts in guidance and docs are written by
hand and drift". Start with its second part, as the smallest self-contained piece: make
`scripts/check_tier1_docs.py` fail closed on column-0 `pub` items of `lib.rs` that it does not parse
(`pub use x as y;`, nested `pub use a::{b::{c}};`, `pub struct`, `pub static`, `pub unsafe fn`,
`pub async fn`), pinned by a fire test per form. The registry design and the discovery threshold are
settled (decisions.md 2026-11-20, "The Tier 1 doc gate matches names against a registry…"); do not
re-open them. The other two parts (docs below the threshold, unchecked counts in specs and per-crate
`CLAUDE.md` files) are separate steps.

**Notes:**

- Install time of the source gem with the profile: 52 s here, 55 s in the advance run (WSL2, warm
    cargo registry, about 50 s of it is the LTO build). No "before" number exists. The
    `ruby-source-gem` CI job on this push is the first CI run with the profile; update-state should
    confirm it is green
- No decisions.md entry: the parity branch follows root `CLAUDE.md` → Design Constraints, and the
    text slice follows the existing no-`tomllib` precedent (decisions.md 2026-07-26)
- No Rust source changed, so CRAP, iai and semver gates are unaffected
- learnings.md is at its 200-line budget; this review edited one bullet in place
