# Next Work Package

## Step: Add Ruby gemspec version to version_sync.py

## Goal

Integrate `crates/iscc-rb/lib/iscc_lib/version.rb` into the version sync system so that Ruby gem
versions stay in lockstep with the workspace version from root `Cargo.toml`. This is a prerequisite
for the RubyGems release workflow — without version sync, the gem could ship with a stale version.

## Scope

- **Create**: (none)
- **Modify**: `scripts/version_sync.py`
- **Reference**: `crates/iscc-rb/lib/iscc_lib/version.rb`, `crates/iscc-rb/iscc-lib.gemspec`

## Not In Scope

- Adding a `rubygems` publish step to `release.yml` — that's a separate step after version sync
- Standard Ruby linting (`standard` gem, `.standard.yml`) — independent concern
- Documentation (`docs/howto/ruby.md`, expanding `crates/iscc-rb/README.md`) — separate step
- Adding Ruby install/quickstart to the root `README.md` — separate step
- Modifying the version.rb file itself (it already has the correct format and current version)

## Implementation Notes

The version.rb file has this format:

```ruby
# frozen_string_literal: true

# Version constant for the iscc-lib gem (synced from root Cargo.toml).
module IsccLib
  VERSION = "0.1.0"
end
```

Add two functions following the existing pattern in version_sync.py:

1. `_get_ruby_version(text)` — extract version from `VERSION = "X.Y.Z"` using regex like
    `r'VERSION\s*=\s*"(\d+\.\d+\.\d+)"'`
2. `_sync_ruby_version(text, version)` — replace the version string in the `VERSION = "..."` line

Add the target tuple to the `TARGETS` list:

```python
(("crates/iscc-rb/lib/iscc_lib/version.rb", _get_ruby_version, _sync_ruby_version),)
```

Also update the module docstring at the top of `version_sync.py` to mention the Ruby version.rb
target in the "Synced targets" list.

Pattern to follow: the pyproject.toml sync functions are the closest analog — simple regex
get/replace on a `version = "X.Y.Z"` pattern. The Ruby version uses `VERSION = "X.Y.Z"` (with
uppercase and space around `=`).

## Verification

- `uv run scripts/version_sync.py --check` exits 0 (Ruby version.rb included in output, shows "OK")
- `grep -c "version.rb" scripts/version_sync.py` returns at least 1 (target registered)
- `uv run scripts/version_sync.py --check 2>&1 | grep "iscc-rb"` shows the Ruby target checked
- `mise run lint` passes (ruff + clippy clean)

## Done When

All verification criteria pass — `version_sync.py --check` validates the Ruby version.rb target
alongside all existing targets, and linting is clean.
