## 2026-03-04 — Add RubyGems publish step to release.yml

**Done:** Added `rubygems` workflow dispatch input, `build-gem` cross-platform build job using
`oxidize-rb/actions/cross-gem@v1` (5 platforms: x86_64-linux, aarch64-linux, x86_64-darwin,
arm64-darwin, x64-mingw-ucrt), and `publish-rubygems` publish job with RubyGems version-exists
idempotency check and `GEM_HOST_API_KEY` secret. Source gem is built and published alongside
precompiled platform gems as a fallback.

**Files changed:**

- `.github/workflows/release.yml`: Added `rubygems` boolean input, `build-gem` job (5-platform
    matrix, Docker-based cross-compilation on ubuntu-latest), and `publish-rubygems` job (version
    check via RubyGems API, artifact download, source gem build, gem push with API key)

**Verification:** All 7 criteria pass — valid YAML, `rubygems` appears 5 times (≥3 required),
`publish-rubygems` job present, `build-gem` job present, `GEM_HOST_API_KEY` secret referenced,
`oxidize-rb/actions/cross-gem@v1` action used, `x86_64-linux` platform in matrix. All pre-commit
hooks pass (`mise run check` clean after yamlfix auto-formatting).

**Next:** The Ruby binding issue has 2 remaining infrastructure items: (1) `docs/howto/ruby.md`
how-to guide + expand `crates/iscc-rb/README.md`, (2) Ruby section in root `README.md`. These are
documentation tasks that can be grouped into a single step.

**Notes:**

- `oxidize-rb/cross-gem-action` (standalone repo) is archived since May 2023. Used the maintained
    `oxidize-rb/actions/cross-gem@v1` (monorepo action) instead, as recommended by the maintainers.
- All cross-compilation runs on `ubuntu-latest` via Docker (rake-compiler-dock). No Windows or macOS
    runners needed for the build-gem job — Docker handles all cross-compilation internally.
- `ruby-versions: 3.1, 3.2, 3.3` — yamlfix strips quotes from this value (changes `'3.1, 3.2, 3.3'`
    to `3.1, 3.2, 3.3`). This is fine — the action accepts comma-separated strings and YAML will
    treat the unquoted value as a string due to commas.
- The `cross-gem` action outputs `gem-path` pointing to the compiled gem. The path pattern used for
    artifact upload is `crates/iscc-rb/pkg/*-${{ matrix.platform }}.gem` which matches rb-sys-dock's
    output convention.
- `fail-fast: false` on the build matrix ensures all platform builds complete even if one fails
    (matching the pattern of providing maximum artifacts from a single workflow run).
- RubyGems API key authentication (not OIDC) is used per next.md spec. The `GEM_HOST_API_KEY`
    environment variable is the standard mechanism that `gem push` reads automatically.
- Human action still required: RubyGems.org account setup, gem name reservation, and API key
    configuration as a GitHub repository secret.
