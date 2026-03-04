## 2026-03-04 — Review of: Configure Standard Ruby linting

**Verdict:** PASS

**Summary:** Standard Ruby linting fully configured with `.standard.yml`, `rubocop-minitest` plugin,
CI enforcement, and both pre-commit (auto-fix) and pre-push (check) hooks. Eight mechanical lint
fixes across test files are correct and semantically equivalent. All verification criteria pass —
`standardrb` exits 0, 111 tests / 299 assertions pass, `mise run check` shows 15 hooks green, clippy
clean.

**Verification:**

- [x] `bundle exec standardrb` exits 0 — all Ruby files pass linting
- [x] `bundle exec rake test` — 111 runs, 299 assertions, 0 failures, 0 errors, 0 skips
- [x] `grep 'standard' crates/iscc-rb/Gemfile` — finds `gem "standard", "~> 1.0"`
- [x] `test -f crates/iscc-rb/.standard.yml` — config file exists (3 lines: plugins + ignore)
- [x] `grep 'standardrb' .pre-commit-config.yaml` — finds both `standardrb-fix` and `standardrb`
- [x] `grep 'standardrb' .github/workflows/ci.yml` — finds `Run standardrb` step
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean
- [x] `mise run check` — all 15 pre-commit hooks pass (including new Ruby linting hook)
- [x] No quality gate circumvention in diff — hooks added, not weakened

**Issues found:**

- (none)

**Codex review:** Codex flagged that the `files: ^crates/iscc-rb/.*\.rb$` pattern won't trigger
hooks for `Gemfile`/`Rakefile` (no `.rb` suffix). Technically correct: if ONLY `Gemfile` is edited,
the hook won't fire. However, the hook uses `pass_filenames: false` so when it does run (any `.rb`
change in the same commit), it checks all files including `Gemfile`/`Rakefile`. CI runs `standardrb`
unconditionally as the ultimate backstop. Low-impact — `Gemfile`/`Rakefile` changes are rare and
typically paired with `.rb` changes.

**Next:** The Ruby bindings are now substantially complete with linting. Remaining Ruby items: (1)
`docs/ruby-api.md` API reference page (spec line 310), (2) RubyGems account setup (human action).
Non-Ruby: the root README Go quickstart example is stale (uses old WASM-bridge API).

**Notes:**

- Pre-commit hook count is now 15 (was 14) — update any documentation referencing the count.
- next.md predicted 295 assertions but actual count is 299. The difference comes from Minitest
    counting `refute_includes` / `refute_empty` differently than the manual `refute x.include?` /
    `refute x.empty?` they replaced.
- The Ruby linting item from state.md "Missing" list is now addressed. After `docs/ruby-api.md` is
    created, Ruby can move from "partially met" to "met" (assuming linting is also reflected in
    state.md).
