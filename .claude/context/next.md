# Next Work Package

## Step: Give the Ruby source gem the workspace release profile

## Goal

Pick up the `normal` issue "The Ruby source gem is built without the workspace release profile": the
staged standalone `Cargo.toml` carries the root `[profile.release]` table, derived from the root
manifest and never retyped. A source install then builds stripped, with LTO and `panic = "abort"`,
the same as the platform gems. This clears one of the two remaining v0.8.0 blockers.

## Alternatives Considered

- **Chosen:** the source-gem release profile. It is small and self-contained, the review handoff
    suggests it, and the issue names a mechanical check for it
- **Rejected:** making the Tier 1 doc gate fail closed on crate-root forms it does not parse (part
    of the derived-fact drift issue). It is equally valuable but touches a different subsystem.
    Finishing the Ruby source-gem work while its context is fresh is cheaper. The drift issue is
    next

## Scope

- **Modify**: `scripts/build_ruby_source_gem.py` (a helper that extracts the table,
    `render_manifest`, `stage_gem_tree`, module docstring), `crates/iscc-rb/CLAUDE.md` → Publishing
    (one sentence: a source install uses the workspace `[profile.release]`, copied by the script)
- **Tests**: `tests/test_build_ruby_source_gem.py`
- **Reference**: root `Cargo.toml` lines 58–72 (`[profile.release]`, then `[profile.bench]`),
    `scripts/check_docs_nav.py` line 39 (why there is no `tomllib`)

## Not In Scope

- The issue's second branch (a decisions.md entry that accepts a different profile). The parity
    branch matches root `CLAUDE.md` → Design Constraints, and advance cannot write decisions.md. If
    the install time looks alarming, report it in the handoff and do not switch branches
- Copying `[profile.bench]`, `[patch]` or workspace lints into the staged manifest
- Changing the gemspec, `extconf.rb`, the `ruby-source-gem` CI job or `release.yml`
- Re-opening the staged-gem design (carried core, shipped `Cargo.lock`, conditional `rb_sys`;
    decisions.md 2026-11-20)

## Implementation Notes

- **No `tomllib`.** The tests import the script in-process, and the `python-test` CI matrix runs
    Python 3.10. Take a verbatim text slice of the `[profile.release]` table from
    `Path(metadata["workspace_root"]) / "Cargo.toml"`: the lines after the header, up to the next
    `[`-header line. Copy the key lines as they are instead of re-rendering values. That way the
    table is derived, and a key added later in the root comes along with nothing else to change
- **Fail closed** (`SystemExit`, like the rest of the script) when:
    - the root has no `[profile.release]` table,
    - the table has no key lines,
    - the root has any `[profile.release.<x>]` sub-table, such as a per-package override. The slice
        would drop it silently
- **Boundary trap:** `[profile.bench]` comes right after the table, with `strip = false` and
    `debug = true`, and a comment block sits between the two. The slice must end at the bench header
    and must not pick up its keys. Comments and blank lines may be copied or dropped, as long as the
    result is valid TOML
- Pass the table text into `render_manifest`. A new parameter is fine. Put the table before or after
    `[workspace]`. **Expected collateral:** `test_render_manifest_is_standalone` asserts
    `manifest.rstrip().endswith("[workspace]")`, and every test calls
    `render_manifest(rb_package())`. Update those tests rather than bending the layout or the
    signature to keep them green
- Tests to add (simple functions, real data where possible):
    1. Run the helper on the real root `Cargo.toml`. Each `key = value` line of the root table, read
        by the test's own independent line scan, appears in the staged manifest. The scan needs a
        non-empty floor. Do not hard-code a key count
    2. Neither `strip = false` nor `debug = true` appears in the staged manifest (bench boundary)
    3. Fixture text without the table, and fixture text with a `[profile.release.package.x]`
        sub-table, each raise `SystemExit`
    4. Fixture text with a key removed: that key is absent from the output. This proves the table is
        derived, not retyped
- rb_sys's mkmf builds with cargo's `release` profile by default, so the staged table should take
    effect. The behavioural check below proves it. Do not set `RB_SYS_CARGO_PROFILE`
- **Report only, not a criterion:** time `gem install` on Ruby 3.4 with the profile and say how long
    it took in the handoff. iteration 240 had no timing for the version without the profile, so
    report the number on its own. Do not stash or reset to get a "before" number
- Ruby 3.4.13 is installed under mise (`~/.local/share/mise/installs/ruby/3.4.13`). Use
    `mise exec ruby@3.4.13 -- …`, or put its `bin/` first on `PATH`. Export `LANG=C.UTF-8`
- No Rust source changes, so the CRAP, iai and semver gates are unaffected

## Verification

- `uv run pytest tests/test_build_ruby_source_gem.py tests/test_ruby_gem_docs.py` passes, including
    the 4 new tests above
- `uv run --script scripts/build_ruby_source_gem.py --out /tmp/sg` exits 0. Unpack the gem
    (`gem unpack` or `tar -xOf …gem data.tar.gz | tar -xzO ./Cargo.toml`): its `Cargo.toml` has
    `lto = true`, `codegen-units = 1`, `strip = true` and `panic = "abort"` under a
    `[profile.release]` header, and has no `debug = true`
- On Ruby 3.4.13, from `/tmp` with an empty `GEM_HOME`/`GEM_PATH`:
    `gem install /tmp/sg/iscc-lib-*.gem --no-document` exits 0, and
    `ruby -e 'require "iscc_lib"; exit(IsccLib.conformance_selftest ? 0 : 1)'` exits 0
- `file` on the installed `iscc_rb.so` (`find "$GEM_HOME" -name 'iscc_rb.so'`) does not contain
    `not stripped`. Check the absence; a grep for `stripped` matches both outcomes
- `mise run check` exits 0

## Done When

The staged source gem's manifest carries the root `[profile.release]` table, derived from the root
`Cargo.toml`, the gem installed from it on Ruby 3.4 holds a stripped extension that passes the
selftest, and all verification commands pass.
