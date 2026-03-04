## 2026-03-04 — Review of: Add Ruby gemspec version to version_sync.py

**Verdict:** PASS

**Summary:** Added `_get_ruby_version` and `_sync_ruby_version` to `scripts/version_sync.py`,
registered the `crates/iscc-rb/lib/iscc_lib/version.rb` target in `TARGETS`, and updated the module
docstring. Clean, minimal diff — follows the exact pattern of existing sync functions. All 4
verification criteria pass, no quality gate circumvention.

**Verification:**

- [x] `uv run scripts/version_sync.py --check` exits 0 — all 10 targets OK including Ruby version.rb
    = 0.1.0
- [x] `grep -c "version.rb" scripts/version_sync.py` returns 2 — target registered (docstring +
    TARGETS entry)
- [x] `uv run scripts/version_sync.py --check 2>&1 | grep "iscc-rb"` shows
    `OK: crates/iscc-rb/lib/iscc_lib/version.rb = 0.1.0`
- [x] `mise run check` passes — all 14 pre-commit hooks clean
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean (2.79s)
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** One P2 finding — Ruby `VERSION` regex is unanchored and could theoretically match
other `VERSION`-containing identifiers in version.rb. Risk is negligible because version.rb is a
controlled single-purpose file with only one `VERSION` constant. The unanchored pattern is
consistent with other sync functions in the same file (`_get_mise_version`,
`_get_test_install_version`). No action needed.

**Next:** The Ruby binding issue (issues.md) has 3 remaining infrastructure items before Ruby is
fully "met": (1) RubyGems publish step in `release.yml`, (2) `docs/howto/ruby.md` how-to guide +
expand `crates/iscc-rb/README.md`, (3) Ruby section in root `README.md`. Of these, the RubyGems
release step is the highest-value next step — it enables publishing the gem. Documentation can
follow after release infrastructure is in place.

**Notes:**

- Version sync item 5 from the Ruby bindings issue is now complete. State.md gap "version_sync.py
    does not sync gemspec version" is resolved.
- The Ruby binding issue should remain open — release, documentation, and README items still
    pending.
- 12 CI jobs all green on the latest run.
