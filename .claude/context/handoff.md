# Handoff

## 2026-07-28 — Migrate the Ruby binding from magnus 0.7 to magnus 0.8

**Done:** Bumped the workspace `magnus` pin to 0.8 (removed the 4-line `# held: magnus` comment) and
migrated `crates/iscc-rb/src/lib.rs` off both deprecated `old-api` symbol families:
`magnus::exception::runtime_error()` → `ruby.exception_runtime_error()` (5 sites) and
`RString::from_slice(&b)` → `ruby.str_from_slice(&b)` (5 sites), using the file's existing
`Ruby::get().expect("called from Ruby")` idiom throughout.

**Files changed:**

- `Cargo.toml`: `magnus = { version = "0.8", features = ["rb-sys"] }`; `# held: magnus` block
    deleted (jni/uniffi/criterion holds untouched)
- `Cargo.lock`: magnus 0.7.1 → 0.8.2, magnus-macros 0.6.0 → 0.8.0, rb-sys-env 0.1.2 → 0.2.3
- `crates/iscc-rb/src/lib.rs`: 10 call-site migrations; `to_magnus_err` now obtains a `Ruby` handle;
    in the four hasher `update`/`finalize` methods the handle moved to the top of the method (the
    two `finalize`s reuse it for `hash_new`); `alg_simhash`, `alg_minhash_256`, `soft_hash_video_v0`
    gained a handle before their return-value conversion. All `unsafe { ….as_slice() }` blocks
    untouched (`as_slice` is not deprecated in 0.8)
- `crates/iscc-rb/CLAUDE.md`: line 7 "Magnus 0.7.1" → "Magnus 0.8"

**Verification:**

- `grep -n magnus Cargo.toml` → only line 53 `version = "0.8"`; no `held: magnus` match
- `grep -rn 'magnus::exception::\|RString::from_slice' crates/iscc-rb/src/lib.rs` → no match
- `cargo clippy -p iscc-rb --all-targets -- -D warnings` → exit 0 (compiled against magnus 0.8.2)
- Pre-edit baseline `(cd crates/iscc-rb && bundle exec rake compile && bundle exec rake test)`:
    **124 runs, 314 assertions, 0 failures, 0 errors, 0 skips**
- Post-edit same invocation: **124 runs, 314 assertions, 0 failures, 0 errors, 0 skips** — identical
    totals, six `assert_raises(RuntimeError)` sites still green
- `mise run audit` → exit 0 (`advisories ok, bans ok, licenses ok, sources ok`)
- `mise run check` → exit 0, all hooks Passed (incl. "Ruby linting (fix)" = standardrb); tracked
    tree unchanged except the intended files + runner-owned `iterations.jsonl`

**Next:** `jni` 0.22 is the last CID-schedulable Rust major (`crates/iscc-jni/src/lib.rs`, ~41
`JNIEnv|GlobalRef|AutoLocal` sites per upstream `docs/0.22-MIGRATION.md`) — needs its own scoping
pass with an `ErrorPolicy` decision. Beyond that, the refresh remainder is human/release-gated
(`release.yml` action bumps, uniffi 0.32, criterion 0.8 MSRV).

**Notes:**

- Baseline test totals for the ledger: **124 runs / 314 assertions before AND after**, zero skips
    (next.md asked for this line; both runs are on the same tree modulo my edits, so the equality is
    the meaningful check — the suite grew from 111 at iter 130 because the unicode-boundary suite
    landed in between).
- Lock delta deviation from next.md's prediction: `rb-sys-env` 0.1.2 → 0.2.3 came along with magnus
    \+ magnus-macros (a build-dependency of magnus/rb-sys; `cargo update -p magnus` pulled it as part
    of the minimal update). No other lock movement.
- No consumer floor moved: gemspec `required_ruby_version`, CI `ruby-version: '3.1'`, and workspace
    `rust-version` are all untouched, matching next.md's measurement (magnus 0.8 MSRV 1.65, Ruby
    3.0-3.4).
- The `rb_sys` gem pin (`Gemfile` 0.9.123) and the crate-side `rb-sys` 0.9.128 in Cargo.lock are
    distinct artifacts; neither moved (magnus 0.8 requires rb-sys >= 0.9.113, already satisfied).
- Ruby-visible behaviour unchanged by construction: `exception_runtime_error()` returns the same
    `RuntimeError` class the old free function did.
