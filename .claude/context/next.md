# Next Work Package

## Step: Add Ruby CI job to ci.yml

## Goal

Add a dedicated Ruby CI job to `.github/workflows/ci.yml` that compiles and tests the `iscc-rb`
crate on every push, and runs clippy on the Ruby bridge crate. This enables automated verification
of all 111 Ruby tests (including 50 conformance vectors) on every push — currently Ruby is excluded
from CI entirely.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml`
- **Reference**:
    - `crates/iscc-rb/Rakefile` — build/test task definitions
    - `crates/iscc-rb/Gemfile` — Ruby dependencies
    - `crates/iscc-rb/extconf.rb` — extension config (uses `rb_sys`)
    - `crates/iscc-rb/iscc-lib.gemspec` — `required_ruby_version: ">= 3.1.0"`
    - `.github/workflows/ci.yml` — existing CI structure (follow patterns from java/nodejs jobs)

## Not In Scope

- **Do NOT remove `--exclude iscc-rb` from the Rust job** — the Rust job lacks Ruby headers and
    libclang-dev. Clippy for iscc-rb runs in the new Ruby job instead. This mirrors how the Java job
    doesn't rely on the Rust job for JNI-specific checks.
- Do NOT add Standard Ruby linting (`standard` gem) — that's a separate step
- Do NOT modify `release.yml` — RubyGems publish is a separate step
- Do NOT modify `version_sync.py` — gemspec sync is a separate step
- Do NOT write `docs/howto/ruby.md` — documentation is a separate step

## Implementation Notes

Add a new `ruby` job to `ci.yml` following the pattern of existing jobs (checkout → toolchain →
cache → build → test). Place it after the `go` job and before the `bench` job. The job structure:

1. **`actions/checkout@v4`**
2. **`dtolnay/rust-toolchain@stable`** with `components: clippy` — needed for both Rust compilation
    and clippy linting
3. **`Swatinem/rust-cache@v2`** — cache Rust compilation artifacts
4. **Install `libclang-dev`** — required by rb-sys/bindgen at compile time. Use
    `sudo apt-get update && sudo apt-get install -y libclang-dev`.
5. **`ruby/setup-ruby@v1`** with `ruby-version: '3.1'` and `working-directory: crates/iscc-rb`. Use
    `bundler-cache: true` to auto-install gems from the Gemfile.
6. **Clippy**: `cargo clippy -p iscc-rb -- -D warnings` — lint the bridge crate
7. **Compile**: `bundle exec rake compile` with `working-directory: crates/iscc-rb`
8. **Test**: `bundle exec rake test` with `working-directory: crates/iscc-rb`

Name the job key `ruby` with display name `Ruby (magnus build, test)`.

**`ruby/setup-ruby` working-directory**: The action supports a `working-directory` input parameter
for bundler operations (finding Gemfile). Set it as an action `with:` parameter, not a step-level
`working-directory`.

**Clippy step placement**: Run clippy BEFORE `rake compile` because clippy catches issues faster
than a full build+test cycle. Clippy will compile the crate as a side effect (populating the cargo
cache), which `rake compile` then benefits from.

**No `shell: bash` needed**: This job only runs on `ubuntu-latest` (no Windows matrix), so default
shell is fine.

## Verification

- `grep -c 'name: Ruby' .github/workflows/ci.yml` returns 1 (new job exists)
- `grep 'cargo clippy -p iscc-rb' .github/workflows/ci.yml` finds the clippy step
- `grep 'bundle exec rake test' .github/workflows/ci.yml` finds the test step
- `grep 'libclang-dev' .github/workflows/ci.yml` finds the dependency install step
- `grep 'exclude iscc-rb' .github/workflows/ci.yml` still finds 2 occurrences (Rust job keeps its
    excludes — this is intentional)
- YAML is valid: `python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0
- `mise run check` passes (pre-commit hooks validate YAML/formatting)

## Done When

All verification criteria pass — the Ruby CI job is defined in ci.yml with clippy, compile, and test
steps, and the file passes YAML validation and formatting checks.
