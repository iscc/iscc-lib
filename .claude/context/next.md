# Next Work Package

## Step: Migrate the Ruby binding from magnus 0.7 to magnus 0.8

## Goal

Close the `magnus` 0.8 major of the authorized dependency refresh (issues.md → "Dependency review
and refresh across the project"): bump the workspace pin and migrate the two `old-api` symbol
families that 0.8 deprecates, leaving `jni` 0.22 as the last CID-schedulable major.

## Alternatives Considered

- **Chosen:** magnus 0.8 — one source file, two mechanical symbol families (10 call sites), fully
    verifiable by the local Ruby gates, and it retires one of the two remaining `# held:` majors.
- **Rejected:** `jni` 0.22 (the handoff's first suggestion) — a wholesale rework (778-line upstream
    migration doc, 41 `JNIEnv` sites, closure-based `with_env` plus a per-function `ErrorPolicy`
    choice across ~40 exported natives). It carries real design decisions and deserves its own
    scoping pass; taking the small major first keeps each red attributable to one ecosystem.

## Scope

- **Modify**: `Cargo.toml` (the `magnus` pin + delete the 4-line `# held: magnus` comment above it),
    `crates/iscc-rb/src/lib.rs`, `crates/iscc-rb/CLAUDE.md` (line 7 says "Magnus 0.7.1"),
    `Cargo.lock` (generated)
- **Reference**: `crates/iscc-rb/Gemfile` (the `rb_sys` lockstep comment),
    `.github/workflows/ci.yml` → the `ruby:` job (lines 187-212 — copy the gate invocations from
    there)

## Not In Scope

- `jni` 0.22, `uniffi` 0.32, `criterion` 0.8 — one major per step, no bundling.
- Re-enabling magnus's `old-api` feature: that silences the deprecations instead of migrating.
- The `rb_sys` **gem** pin (`Gemfile`/`Gemfile.lock` at `0.9.123`) and the `oxidize-rb/cross-gem`
    `tag:` in `release.yml` — they move together or not at all, and magnus 0.8 needs neither.
- Restructuring the binding (e.g. giving the module functions a `ruby: &Ruby` first parameter) —
    keep the file's existing `Ruby::get()` idiom.
- Editing `issues.md` (review owns issue resolution).

## Implementation Notes

Measured read-only from the `magnus-0.8.2` crate source while scoping:

- Pin `magnus = { version = "0.8", features = ["rb-sys"] }`. MSRV is 1.65 and Ruby 3.0-3.4 stay
    supported, so **no consumer floor moves** (gemspec `required_ruby_version`, CI
    `ruby-version: '3.1'`, workspace `rust-version` all unchanged). magnus 0.8 requires
    `rb-sys >= 0.9.113` and `Cargo.lock` already holds 0.9.128, so the lock delta should be magnus +
    magnus-macros only.
- 0.8 drops `old-api` from the default features, which turns two families into `#[deprecated]` (i.e.
    `clippy -D warnings` failures); each has a replacement on the `Ruby` handle:
    - `magnus::exception::runtime_error()` → `ruby.exception_runtime_error()` — 5 sites
    - `RString::from_slice(&b)` → `ruby.str_from_slice(&b)` — 5 sites
- Obtain the handle with the file's existing idiom, `Ruby::get().expect("called from Ruby")`.
    `to_magnus_err` (`src/lib.rs:25`) has no handle in scope — add one there.
- `RString::as_slice` is **not** deprecated: leave the `unsafe { … .as_slice() }` blocks alone. 0.8
    removes `FString` and `RString::as_interned_str`; this crate uses neither.
- Clippy is the authority on the deprecation set — if it flags a symbol not listed above, fix it the
    same way (grab a `Ruby` handle), never by re-enabling `old-api`.
- Ruby-visible behaviour must not change: the suites assert `assert_raises(RuntimeError)` at six
    places and `exception_runtime_error()` is that same class.
- The compiled `.so` is gitignored and goes stale silently — `bundle exec rake compile` before
    `rake test`. Bundler has no `-C`; use a subshell, `(cd crates/iscc-rb && bundle exec …)`.
- `test_conformance.rb` defines its cases dynamically, so the suite total is not greppable: run
    `rake test` on the unmodified tree first, and put its "N runs, M assertions" line in the handoff
    Notes so review can check the totals did not move.

## Verification

- `grep -n magnus Cargo.toml` shows `version = "0.8"` and no `# held: magnus` block remains
- `grep -rn 'magnus::exception::\|RString::from_slice' crates/iscc-rb/src/lib.rs` → no match
- `cargo clippy -p iscc-rb --all-targets -- -D warnings` exits 0
- `(cd crates/iscc-rb && bundle exec rake compile && bundle exec rake test)` exits 0 reporting
    `0 failures, 0 errors, 0 skips` at the same runs/assertions totals as the pre-edit run
- `mise run audit` exits 0 (Cargo.lock moved)
- `mise run check` exits 0 and leaves no tracked file modified

## Done When

magnus 0.8 is the workspace pin, `crates/iscc-rb` compiles deprecation-free against it, and every
Ruby gate is green at unchanged test totals.
