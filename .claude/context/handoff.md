# Handoff

## 2026-07-28 — Review of: Migrate the Ruby binding from magnus 0.7 to magnus 0.8

**Verdict:** PASS

**Summary:** A clean, mechanical major bump: workspace pin `magnus 0.7 → 0.8`, the stale `# held:`
comment removed, and both deprecated `old-api` families migrated to their `Ruby`-handle equivalents
(10 call sites) with no Ruby-visible behaviour change. Every gate from next.md is green, the lock
delta is contained to iscc-rb's subtree, and no consumer floor moved.

**Verification:**

- [x] `grep -n magnus Cargo.toml` → only `magnus = { version = "0.8", features = ["rb-sys"] }`; no
    `held: magnus` match (the criterion/jni/uniffi holds are intact)
- [x] `grep -rn 'magnus::exception::\|RString::from_slice' crates/iscc-rb/src/lib.rs` → no match
- [x] `cargo clippy -p iscc-rb --all-targets -- -D warnings` → exit 0 (compiled magnus 0.8.2)
- [x] `(cd crates/iscc-rb && bundle exec rake compile && bundle exec rake test)` → exit 0, **124
    runs, 314 assertions, 0 failures, 0 errors, 0 skips** — same totals as the handoff's pre-edit
    baseline, and structurally comparable (no test file, `Gemfile*` or fixture moved in the diff, so
    the dynamic case set cannot have shifted)
- [x] `mise run audit` → exit 0 (`advisories ok, bans ok, licenses ok, sources ok`)
- [x] `mise run check` → exit 0, 18 hooks Passed, tracked tree unchanged (only runner-owned
    `iterations.jsonl` dirty)

Extra probes (3 of 3): the tested artifact really is the migrated one —
`strings crates/iscc-rb/lib/iscc_lib/iscc_rb.so` reports `magnus-0.8.2` / `rb-sys-0.9.128`, so the
gitignored `.so` is not a stale 0.7 build. Floor claim re-derived from the crate source: magnus
0.8.2 README says "Ruby versions 3.0–3.4 are fully supported", MSRV 1.65 — both below the gemspec
`>= 3.1.0`, CI `ruby-version: '3.1'`, release cross-gem `3.1, 3.2, 3.3` and workspace
`rust-version = "1.85"`, so **no consumer floor moved**. `cargo tree -i` confirms magnus and the
surprise `rb-sys-env 0.1.2 → 0.2.3` reach only `iscc-rb` (rb-sys-env is magnus's own build-dep).

**Issues found:**

- Style nit, not filed: in the four hasher `update`/`finalize` methods the `Ruby` handle is now
    acquired on every call although `update` uses it only in the never-taken error path.
    `Ruby::get()` is a thread-local `RubyGvlState::cached()` read, so the cost is noise; a future
    cleanup could take the handle inside the `ok_or_else` closure or use the infallible
    `Ruby::get_with(data)`. next.md ruled restructuring out of scope, so the current form is right.
- Gate-integrity scan over all unpushed commits (`@{upstream}..HEAD`): no suppressions, no skips, no
    threshold or hook weakening. The only `noqa` hit is a memory note warning against them.

**Codex review:** Clean — "behaviorally equivalent APIs … dependency lock updated consistently.
Clippy, native-extension compilation, and all 124 Ruby tests pass without errors or warnings." No
actionable findings.

**Next:** `jni` 0.22 (`crates/iscc-jni/src/lib.rs`) is now the **only** CID-schedulable item in the
dependency-refresh issue — the issue body has been rewritten down to that single remaining problem.
It is genuinely bigger than magnus: `JNIEnv → EnvUnowned/Env`, `GlobalRef → Global`,
`AutoLocal → Auto`, closure-based thread attachment and a mandatory per-function `ErrorPolicy`
across ~41 sites, per upstream `docs/0.22-MIGRATION.md`. define-next should spend a scoping pass on
it and consider slicing it (plumbing conversion first, the `ErrorPolicy` decision second) rather
than declaring one 41-site package; if it cannot be sliced honestly, a `**Fan-out:**` declaration is
the right escape valve. Local gates: `cargo clippy -p iscc-jni` plus the Java/Kotlin suites — and
the JNI `.so` is another gitignored artifact that goes stale silently, so rebuild before testing.

**Notes:**

- Quality-gate maintenance: nothing new needed. The Ruby surface is fully covered locally (rake
    compile + rake test + standardrb via prek), which is why this bump was verifiable end to end.
- Watch item for the eventual release, not an issue: the cross-gem Docker build (Ruby 3.1/3.2/3.3,
    `rb_sys` gem pinned `0.9.123`) is the one magnus consumer CI never exercises. magnus 0.8
    supports all three interpreters and needs only `rb-sys >= 0.9.113` (lock has 0.9.128), so risk
    is low, but a v0.6.0 release run is the first real proof.
- learnings.md is at 197/200 after archiving the per-build-system fixture-input detail — the next
    reviewer should archive before appending.
