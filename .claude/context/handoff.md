## 2026-03-04 — Review of: Add RubyGems publish step to release.yml

**Verdict:** PASS

**Summary:** Added `rubygems` workflow dispatch input, cross-platform `build-gem` job (5 platforms
via `oxidize-rb/actions/cross-gem@v1`), and `publish-rubygems` job with version-exists idempotency
check and `GEM_HOST_API_KEY` secret. Clean, well-structured diff that follows the exact pattern of
the 5 existing registry publish jobs. All 7 verification criteria pass.

**Verification:**

- [x] `python -c "import yaml; yaml.safe_load(...)"` exits 0 — valid YAML (also confirmed by
    `check   yaml` pre-commit hook)
- [x] `grep -c 'rubygems'` returns 5 (≥3 required) — input + job name + two conditions + needs
- [x] `grep 'publish-rubygems'` finds the publish job
- [x] `grep 'build-gem'` finds the build job (and its `needs` reference)
- [x] `grep 'GEM_HOST_API_KEY'` finds the secret reference in the env block
- [x] `grep 'cross-gem'` finds `oxidize-rb/actions/cross-gem@v1`
- [x] `grep 'x86_64-linux'` finds the platform in the build matrix
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean (2.84s)
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** No actionable findings. Codex confirmed the jobs follow the existing tag-or-input
gating and version-exists skip pattern with no correctness bugs.

**Next:** The Ruby binding issue has 2 remaining infrastructure items: (1) `docs/howto/ruby.md`
how-to guide + expand `crates/iscc-rb/README.md` from stub to full guide, (2) Ruby section in root
`README.md`. These are documentation tasks that can be grouped into a single step. Standard Ruby
linting (`standard` gem) is also listed in state.md but is lower priority than docs.

**Notes:**

- All cross-compilation runs on `ubuntu-latest` via Docker (rake-compiler-dock). No Windows or macOS
    runners needed for the `build-gem` job.
- RubyGems uses API key authentication (not OIDC). The `GEM_HOST_API_KEY` environment variable is
    the standard mechanism `gem push` reads. Human action still required: RubyGems.org account
    setup, gem name reservation, and API key secret configuration.
- The `publish-rubygems` job's version check (`curl -sf` to RubyGems API) correctly falls through to
    publish when the gem doesn't exist yet (404 → curl fails → grep fails → `skip=false`).
- Source gem is built and pushed alongside precompiled platform gems as a fallback for unsupported
    platforms.
- Release workflow now has 6 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems.
