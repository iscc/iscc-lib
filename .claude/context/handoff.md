# Handoff

## 2026-07-28 — Review of: Bump criterion 0.7 → 0.8 (dev-dependency, bench harness only)

**Verdict:** PASS

**Summary:** A minimal, correctly scoped dependency major: two files (`Cargo.toml` pin +
`Cargo.lock`), no bench-source edit needed, MSRV untouched. Every claim in the advance handoff
re-derived and held, including the load-bearing one — criterion has no non-dev reachability, so its
rustc-1.86 floor cannot reach a consumer.

**Verification:**

- [x] Root `Cargo.toml` pins `version = "0.8"` (line 47); `Cargo.lock` `criterion` → `0.8.2`
- [x] `grep -n 'rust-version = "1.85"' Cargo.toml` → line 17, unchanged
- [x] `cargo bench --no-run` → exit 0 (CI `bench` job command)
- [x] `cargo bench -p iscc-lib --bench benchmarks -- --test` → exit 0, 18 `Testing` / 18 `Success`
- [x] `mise run lint` → exit 0 (clippy `--workspace --all-targets -D warnings`, 187 files formatted)
- [x] `mise run test` → exit 0 (all cargo suites + 441 pytest); `mise run audit` → exit 0
    (`advisories ok, bans ok, licenses ok, sources ok`, cargo-deny 0.19.9, no `deny.toml` edit)
- [x] Beyond next.md — `mise run check` → exit 0, 18 hooks Passed; tree clean
- [x] Beyond next.md — lock delta reaches only criterion: `cargo tree -i` shows `alloca 0.4.0` and
    `page_size 0.6.0` under `criterion 0.8.2` alone, `winapi` only under `page_size` (visible only
    with `--target all`); `cargo tree -i criterion -e no-dev --target all` prints *nothing* — the
    proof that the 1.86 floor is dev-only
- [x] Beyond next.md — `iai_benches.rs` mentions criterion in doc comments only, so the iai gate is
    untouched; leaving `.iai-baseline.json` / `.crap-baseline.json` alone was correct

**Issues found:** (none) — no gate circumvention across the full unpushed range
(`@{upstream}..HEAD`, 5 commits incl. the human `cid(loop)` commit); no lint suppression, skipped
test, threshold change or hook weakening. No API surface touched, no hot-path code moved.

**Codex review:** Clean — no findings. Confirms consistent resolution, compiling tests, all bench
cases executing, advisory check green, and that the 1.86 requirement is confined to dev targets.

**Next:** `uniffi` 0.31 → 0.32 — the last authorized major and the last v0.6.0 release-gating
dependency item. Read the 0.32 changelog first; if `crates/iscc-uniffi/src` needs edits, stop and
re-scope per the issues.md constraint. Regenerate both checked-in bindings (Swift + Kotlin) as a
pure regeneration. **Verify both locally**: Kotlin via Gradle, and Swift via the swift.org Debian 12
6.1.2 tarball recipe in `packages/swift/CLAUDE.md` (~784 MB, no `sudo`) — I corrected the "Swift is
not locally verifiable" claim in issues.md, which was false. CI's `swift` job stays the final
confirmation on the pushed commit.

**Notes:**

- issues.md: the criterion half of the dependency-refresh entry is deleted (resolved); the entry now
    describes only the uniffi remainder.
- The `proc-macro-error2 v2.0.1` future-incompat warning on every `cargo bench`/`cargo test` is
    pre-existing (dev-only, via `iai-callgrind-macros`, no fixed release) — not introduced here.
- After uniffi lands, the v0.6.0 dependency work is done; what remains for the release is the
    human-gated set (semver-checks flip and MSRV job are v1.0.0 prerequisites, not v0.6.0).
