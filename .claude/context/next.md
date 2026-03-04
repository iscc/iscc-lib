# Next Work Package

## Step: Add RubyGems publish step to release.yml

## Goal

Add a `rubygems` checkbox and cross-platform gem build/publish jobs to the release workflow,
enabling the `iscc-lib` Ruby gem to be published to RubyGems.org alongside the existing registry
targets. This is the critical path for users to `gem install iscc-lib` with precompiled native
extensions.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**:
    - `.github/workflows/release.yml` (existing publish job patterns — crates.io, PyPI, npm, Maven,
        FFI)
    - `crates/iscc-rb/iscc-lib.gemspec` (gem metadata, file list, extension config)
    - `crates/iscc-rb/Rakefile` (rb_sys extension task, compile target)
    - `crates/iscc-rb/Gemfile` (build dependencies: rake-compiler, rb_sys)
    - `crates/iscc-rb/Cargo.toml` (cdylib crate config)
    - `.claude/context/specs/ruby-bindings.md` (spec for release approach)

## Not In Scope

- Standard Ruby linting (`standard` gem, `.standard.yml`) — separate future step
- Documentation (`docs/howto/ruby.md`, expanded README, root README Ruby section) — separate step
- RubyGems.org account setup, gem name reservation, API key configuration — manual human action
- Testing the workflow end-to-end (requires credentials and manual dispatch)
- Adding `rake-compiler-dock` or other cross-compilation infrastructure to the devcontainer

## Implementation Notes

### Pattern to follow

Follow the existing release.yml structure exactly. Each registry has: (1) an `inputs.*` boolean
checkbox, (2) a build job with cross-platform matrix, (3) a publish job with version-exists check
and idempotent skip.

### Workflow dispatch input

Add a `rubygems` boolean input (default: false) after the `ffi` input:

```yaml
rubygems:
  description: Publish iscc-lib to RubyGems
  type: boolean
  default: false
```

### Build job: `build-gem`

Use `oxidize-rb/actions/cross-gem@v1` — the official rb-sys GitHub Action for building precompiled
native gems from Magnus/rb_sys crates. This handles `rake-compiler-dock` internally.

**Platform matrix** — build for 5 Ruby platforms (matching the project's standard 5-platform set):

| Ruby platform    | Equivalent Rust target      |
| ---------------- | --------------------------- |
| `x86_64-linux`   | `x86_64-unknown-linux-gnu`  |
| `aarch64-linux`  | `aarch64-unknown-linux-gnu` |
| `x86_64-darwin`  | `x86_64-apple-darwin`       |
| `arm64-darwin`   | `aarch64-apple-darwin`      |
| `x64-mingw-ucrt` | `x86_64-pc-windows-msvc`    |

The `cross-gem` action takes `platform` and `ruby-versions` inputs. Target Ruby versions:
`3.1, 3.2, 3.3` (gem supports `>= 3.1.0` per gemspec).

Upload each platform gem as an artifact (pattern: `gem-<platform>`).

**Working directory**: `crates/iscc-rb` — the gemspec, Rakefile, and Cargo.toml are all here.

### Publish job: `publish-rubygems`

Depends on `build-gem`. Steps:

1. **Get workspace version** — same `grep` pattern as other jobs
2. **Check version on RubyGems** — `curl -sf "https://rubygems.org/api/v1/versions/iscc-lib.json"`
    and check if the version already exists. If yes, skip
3. **Download gem artifacts** — `actions/download-artifact@v4` with `pattern: gem-*`
4. **Publish each gem** — `gem push *.gem` with `GEM_HOST_API_KEY` secret. Push all platform gems

The publish job should also build and push a **source gem** (fallback for platforms without
precompiled gems). Use `gem build iscc-lib.gemspec` in the publish job after checkout.

### Trigger condition

Same pattern as all other jobs: `startsWith(github.ref, 'refs/tags/v') || inputs.rubygems`

### Permissions

The publish job only needs `contents: read`. RubyGems uses an API key secret, not OIDC (unlike
crates.io/PyPI). The secret name should be `GEM_HOST_API_KEY` (the standard env var that `gem push`
reads).

### Important: shell: bash on Windows

If any step runs on Windows runners, ensure `shell: bash` is set (per learnings). The build matrix
includes `x64-mingw-ucrt` which may run on `windows-latest`. Check whether
`oxidize-rb/actions/cross-gem` runs on ubuntu with Docker (likely — rake-compiler-dock is
Docker-based) or requires platform-native runners. If all cross-compilation happens on ubuntu via
Docker, no Windows runner is needed.

## Verification

- `python -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -c 'rubygems' .github/workflows/release.yml` returns at least 3 (input + job name +
    condition)
- `grep 'publish-rubygems' .github/workflows/release.yml` finds the publish job
- `grep 'build-gem' .github/workflows/release.yml` finds the build job
- `grep 'GEM_HOST_API_KEY' .github/workflows/release.yml` finds the secret reference
- `grep 'cross-gem' .github/workflows/release.yml` finds the cross-compilation action reference
- `grep 'x86_64-linux' .github/workflows/release.yml` finds a platform in the build matrix

## Done When

All verification criteria pass — the release.yml contains a complete `rubygems` input, a
cross-platform `build-gem` job using `oxidize-rb/actions/cross-gem`, and a `publish-rubygems` job
with version-exists idempotency check and `GEM_HOST_API_KEY` secret.
