## 2026-03-04 — Add Ruby gemspec version to version_sync.py

**Done:** Added Ruby `version.rb` as a sync target in `scripts/version_sync.py`. Two new functions
(`_get_ruby_version`, `_sync_ruby_version`) extract and update the `VERSION = "X.Y.Z"` constant in
`crates/iscc-rb/lib/iscc_lib/version.rb`. The target is registered in the `TARGETS` list and the
module docstring lists it.

**Files changed:**

- `scripts/version_sync.py`: Added `_get_ruby_version` and `_sync_ruby_version` functions, added
    Ruby target to `TARGETS` list, updated module docstring to mention `version.rb`

**Verification:**

- `uv run scripts/version_sync.py --check` exits 0 — all 10 targets OK including Ruby version.rb
- `grep -c "version.rb" scripts/version_sync.py` returns 2 (docstring + TARGETS entry)
- `uv run scripts/version_sync.py --check 2>&1 | grep "iscc-rb"` shows
    `OK: crates/iscc-rb/lib/iscc_lib/version.rb = 0.1.0`
- `mise run check` passes — all 14 pre-commit hooks clean

**Next:** The next logical step for the Ruby binding infrastructure is adding a RubyGems publish job
to `release.yml`, or writing the `docs/howto/ruby.md` how-to guide. The version sync prerequisite
for RubyGems publishing is now met.

**Notes:** Straightforward implementation — no surprises or technical debt. The functions follow the
exact same pattern as `_get_pyproject_version`/`_sync_pyproject`. No tests needed beyond the
script's own `--check` mode, which serves as the integration test.
