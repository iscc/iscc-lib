# Handoff

## 2026-07-25 — Review of: Dependency refresh slice 7 — Ruby manifests (`Gemfile` + `Gemfile.lock`)

**Verdict:** PASS

**Summary:** The advance agent tightened `rb_sys` from `~> 0.9` to an exact `0.9.123` under a
`# held:` comment, documented the `minitest` 5.x hold, and let `bundle update` refresh the remaining
dev dependencies. The diff is 3 files (one generated lockfile, one doc), every next.md verification
criterion reproduces locally, and — unusually for a hold-back step — **both hold-back rationales
check out against upstream registry metadata**, not just against the handoff prose. No lint rule was
silenced, no Ruby source needed touching, and no Rust was affected.

**Verification:**

- [x] `(cd crates/iscc-rb && bundle exec standardrb)` exits 0 — exit 0, zero offenses under standard
    1.56 / rubocop 1.88
- [x] `(cd crates/iscc-rb && bundle exec rake compile)` exits 0 — exit 0 (cosmetic
    `gmake: Clock skew detected` on the bind mount, as reported)
- [x] `rake test` exits 0, `0 failures, 0 errors, 0 skips` over ≥ 111 runs — exit 0,
    `111 runs, 299 assertions, 0 failures, 0 errors, 0 skips`
- [x] `bundle outdated --strict` exits 0 — exit 0, "Bundle up to date!"
- [x] `grep -q 'rb_sys (0\.9\.123)'` and `grep -q 'rake-compiler-dock (1\.10\.0)'` on Gemfile.lock —
    both exit 0
- [x] `grep -c 'held:' crates/iscc-rb/Gemfile` prints `2` — prints `2`
- [x] `grep -q 'tag: 0.9.123' .github/workflows/release.yml` — exit 0 (`release.yml:853`)
- [x] `grep -q 'required_ruby_version = ">= 3.1.0"'` in the gemspec — exit 0, consumer floor
    untouched
- [x] `! grep -qE 'minitest \(6' crates/iscc-rb/Gemfile.lock` — exit 0, minitest stayed 5.27.0
- [x] `git status --porcelain -- crates/iscc-lib .crap-baseline.json` empty — empty
- [x] `git status --porcelain -- crates/iscc-rb/.standard.yml` empty — empty
- [x] `mise run check` exits 0 — all 15 hooks Passed, nothing rewritten (working tree clean apart
    from runner-owned `iterations.jsonl`)

**Independent checks beyond next.md:**

- **Hold-back #1 substantiated**: `gem specification rb_sys -v <v> --remote` → 0.9.123 →
    `rake-compiler-dock = 1.10.0`, 0.9.124 → 1.11.0, 0.9.128 → 1.12.0. The exact pin really does
    guard the `tag: 0.9.123` cross-gem Docker image; `~> 0.9` would have floated to 0.9.128.
- **Hold-back #2 substantiated**: rubygems v1 API `ruby_version` → minitest 6.0.x is `>= 3.2`,
    5.27.0 is `>= 3.1`. Above the gem's declared `required_ruby_version >= 3.1.0` and CI's
    `ruby-version: '3.1'`, so the hold is required, not preference.
- **Every adopted version is genuinely latest**: rake 13.4.2, standard 1.56.0, rubocop 1.88.2,
    rubocop-minitest 0.40.0, rake-compiler 1.3.1 all match the newest published release, and all
    declare a `ruby_version` floor ≤ 3.1.
- **Lock-format change is inert for CI**: tightening the pin rewrote the lock's `DEPENDENCIES` line
    to `rb_sys (= 0.9.123)`. cross-gem's configure step is
    `grep rb_sys Gemfile.lock | head -n 1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+'` — the `GEM specs`
    line sorts first and the regex extracts `0.9.123` from either line. Confirmed against the action
    source.
- **Frozen install proven**: CI's `ruby/setup-ruby` uses `bundler-cache: true` (frozen).
    `BUNDLE_FROZEN=true bundle install --local` → "Bundle complete! 7 Gemfile dependencies, 27
    gems".
- **Gate-circumvention sweep** over the full unpushed range (`@{upstream}..HEAD`): no suppressions,
    no skipped tests, no threshold or hook changes. The only `rubocop:disable`/`exclude` string hits
    are prose in `next.md`/`state.md` forbidding exactly that.
- **Gemspec is a no-op by construction**: it declares no dev dependencies, only the human-owned
    `required_ruby_version` — so "Gemfile + gemspec" is now fully closed.

**Issues found:** (none)

Minor fix applied in this review commit: the `magnus` `# held:` comment in the root `Cargo.toml`
said the migration was "Deferred to the Ruby dependency slice" — that slice just completed without
touching magnus, so the pointer was made stale by this very commit. Reworded to "in its own
dedicated step". Comment-only, zero behavior change.

**Codex review:** Clean — no findings. Verbatim: "The dependency lockfile resolves reproducibly,
updated gems remain compatible with Ruby 3.1, and the rb_sys pin matches the release workflow tag.
Ruby linting, compilation, and all 111 tests pass." This corroborates the independent metadata
checks above.

**Next:** **Adopt ruff 0.16** and retire the `ruff<0.16` hold-back in `pyproject.toml` (filed under
the dependency-refresh issue). It is the last self-contained, fully locally-verifiable slice: run
`uv run ruff check --fix` for the ~60 auto-fixable findings, hand-fix the remainder (mostly
`crates/iscc-py/python/iscc_lib/_lowlevel.pyi` stub-style `PIE790`/`PYI048`/`RUF022`), then drop the
pin and confirm `mise run check` + the pre-push Ruff hooks (`S` security rules, `C901` complexity)
stay green. Baseline to reproduce first: `uvx ruff@0.16.0 check .` reported 104 errors at iter 125
(72 of them in `_lowlevel.pyi`) — re-measure, since the lock has moved since. **Fix the code, never
widen the lint config**: adding `per-file-ignores` or `# noqa` to make 0.16 pass is gate weakening
and will be rejected. Keep it to `pyproject.toml` + the stub + whatever Python files the fixer
touches.

**Notes:**

- The v0.6.0 dependency-refresh issue now has **every locally-verifiable ecosystem closed** (slices
    1-7: Cargo.lock, uv.lock, Rust pins, GHA refs, JVM manifests, Go module, Ruby Gemfile). What
    remains is ruff 0.16, `release.yml` action refs (not exercised by a CID push — static
    verification only, consider bundling with the existing release.yml `if:`-guard issue), and the
    deferred majors (magnus 0.8, jni 0.22, uniffi 0.32, criterion 0.8, Gradle wrapper, JUnit 6.x,
    xunit 3.x / Test.Sdk 18.x). After ruff, the loop is close to running out of autonomous
    dependency work.
- **Two `normal` `[review]` issues remain blocked on Titusz** and were correctly left untouched: the
    Kotlin consumer floor (2.1 → 2.3, must be settled before the next Maven Central publish) and the
    Rust-core Unicode-version divergence (5,813 code points where the core disagrees with
    `iscc-core` and the Go package). Neither is CID-decidable. Not IDLE — ruff 0.16 is actionable.
- `rb_sys` now appears pinned in three places that must move together: `crates/iscc-rb/Gemfile`,
    `crates/iscc-rb/Gemfile.lock`, and `tag:` in `release.yml`. `crates/iscc-rb/CLAUDE.md` states
    this; a future bump should grep all three.
- No perf, API, or conformance surface was touched — no `cargo semver-checks`, benchmark, or
    conformance re-run was owed this iteration.
