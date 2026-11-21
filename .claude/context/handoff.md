## 2026-11-21 — Give the Ruby source gem the workspace release profile

**Done:** `scripts/build_ruby_source_gem.py` now copies the root `[profile.release]` table into the
staged standalone `Cargo.toml`. It takes the key lines verbatim from the root `Cargo.toml`, so
nothing is retyped (no `tomllib`). The helper `release_profile(text)` fails closed on a missing or
duplicated table, a table with no keys, and any `[profile.release.<x>]` sub-table anywhere in the
file. A source install on Ruby 3.4 now builds a stripped extension that passes the selftest.

**Files changed:**

- `scripts/build_ruby_source_gem.py`: `TOML_HEADER` regex plus the helpers `header_name`,
    `is_key_line` and `release_profile`.
    `render_manifest(rb_package, release_profile_keys, core_dir)` writes the `[profile.release]`
    table before `[workspace]`. `stage_gem_tree` reads the root `Cargo.toml` from
    `metadata["workspace_root"]`. Module docstring updated
- `tests/test_build_ruby_source_gem.py`: a `render()` helper (fixture profile) for the existing
    tests, which now need the new required parameter. 6 new tests: the real root table is copied
    (checked by the test's own line scan, with a non-empty floor); the bench keys stay out
    (`strip = false`, `debug = true`); missing table, sub-table and empty table each raise
    `SystemExit`, one failure arm per fixture with its own `match=`; derivation (a key removed from
    the real root text is absent from the rendered table while the rest remain)
- `crates/iscc-rb/CLAUDE.md` → Publishing: one sentence saying a source install uses the workspace
    `[profile.release]`, copied by the script

**Verification:**

- `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py`: 38 passed (32
    before, plus 6 new)
- `uv run --script scripts/build_ruby_source_gem.py --out /tmp/sg241` exit 0. I read `Cargo.toml`
    out of the gem's `data.tar.gz`. Under the `[profile.release]` header it has `lto = true`,
    `codegen-units = 1`, `strip = true` and `panic = "abort"`. It has no `debug` line and no
    `strip = false`. The comment block between the root release and bench tables is dropped, because
    that comment contains the text `debug = true`
- Ruby 3.4.13, from `/tmp`, `LANG=C.UTF-8`, with a fresh `mktemp -d` used as both `GEM_HOME` and
    `GEM_PATH`: `gem install /tmp/sg241/iscc-lib-0.7.0.gem --no-document` rc=0 (3 gems: rb_sys
    0.9.130 and rake-compiler-dock 1.12.0 came in as dependencies). The selftest
    `ruby -e 'require "iscc_lib"; exit(IsccLib.conformance_selftest ? 0 : 1)'` rc=0
- `find` found two copies of `iscc_rb.so` (under `gems/…/lib/iscc_lib/` and
    `extensions/x86_64-linux/3.4.0-static/…`). `file` reports both as `…, stripped`, and
    `grep -c 'not stripped'` = 0. Size 1,049,536 bytes; the review measured 1,521,056 unstripped in
    iteration 240
- **Install time (report only):** 55 s wall clock for the whole `gem install`, including the
    dependency gems. Warm `~/.cargo` registry cache, fresh build; this devcontainer is on WSL2. I
    have no "before" number (none was recorded in 240, and next.md said not to reset)
- `uv run prek run --hook-stage pre-push --files <script> <test>`: all passed (ruff S/C901, `ty`,
    pytest). `mise run check` rc=0

**Next:** The remaining `normal` v0.8.0 blocker is "Derived facts in guidance and docs are written
by hand and drift". Its first part is to make the Tier 1 doc gate fail closed on crate-root forms it
does not parse.

**Notes:**

- I placed the table before `[workspace]`, so `test_render_manifest_is_standalone` (which checks the
    manifest ends with `[workspace]`) passes unchanged
- `release_profile` matches headers on the whole line (`TOML_HEADER`, spaces removed from the table
    name). A line inside a multi-line array that consists of `["x"]` alone would read as a header.
    The root release table has no such value, and the fail-closed checks catch a slice that
    collapses to nothing
- No Rust source changed, so the CRAP, iai and semver gates are unaffected
- The `ruby-source-gem` CI job will run on this push and build the gem with the profile. It should
    stay well inside its timeout (55 s locally)
