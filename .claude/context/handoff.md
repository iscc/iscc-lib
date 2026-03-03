## 2026-03-03 — Review of: Add Ruby CI job to ci.yml

**Verdict:** PASS

**Summary:** Added a dedicated `ruby` CI job to `.github/workflows/ci.yml` with all 8 required steps
(checkout, rust-toolchain+clippy, rust-cache, libclang-dev, ruby/setup-ruby, clippy, compile, test).
Clean implementation matching next.md exactly — no out-of-scope changes, no quality gate
circumvention. All 7 verification criteria pass.

**Verification:**

- [x] `grep -c 'name: Ruby' .github/workflows/ci.yml` returns 1 — confirmed
- [x] `grep 'cargo clippy -p iscc-rb'` finds the clippy step — confirmed
- [x] `grep 'bundle exec rake test'` finds the test step — confirmed
- [x] `grep 'libclang-dev'` finds the dependency install step — confirmed
- [x] `grep -c 'exclude iscc-rb'` returns 2 — Rust job keeps its excludes, as required
- [x] YAML is valid — `yaml.safe_load` exits 0
- [x] `mise run check` passes — all 14 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Codex review:** No functional, security, or maintainability bugs found. Clean pass.

**Next:** Ruby binding infrastructure is progressing. The next logical step is
`scripts/version_sync.py` gemspec integration (adding `crates/iscc-rb/lib/iscc_lib/version.rb` to
sync targets), the `release.yml` RubyGems publish job, or `docs/howto/ruby.md` documentation. The
version_sync integration is a small, well-scoped step that enables correct version management before
the release workflow is added.

**Notes:**

- The `--exclude iscc-rb` in the Rust CI job is permanent — the Rust job lacks Ruby headers and
    libclang-dev. The dedicated Ruby job handles iscc-rb clippy/compile/test instead.
- CI now has 12 jobs (version-check, rust, python-test, python, nodejs, wasm, c-ffi, java, go, ruby,
    bench) — wait for the next CI run to confirm the Ruby job passes in GitHub Actions.
- The Ruby issues.md entry (item 3: CI) is partially addressed by this iteration. Remaining items:
    release.yml, version_sync, documentation, account setup.
