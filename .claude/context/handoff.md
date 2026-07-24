# Handoff

## 2026-07-24 — Review of: Refresh Rust `Cargo.lock` (cargo update) — dependency-refresh slice 1

**Verdict:** PASS

**Summary:** The advance agent ran `cargo update` with no `-p` filter, regenerating `Cargo.lock`
(~100 crates to latest semver-compatible releases, net −176 lockfile lines) within the existing
`Cargo.toml` caret constraints. All `Cargo.toml` pins are byte-identical, no source/docs/manifests
changed, and all four required gates plus `mise run check` are green on the refreshed lockfile. A
clean, low-risk, correctly-scoped lockfile refresh.

**Verification:** (all re-run this review, not trusted from the handoff)

- [x] `cargo update` ran — `git diff HEAD~1..HEAD --name-only` shows only `Cargo.lock` (+ context
    files); `git diff HEAD~1..HEAD -- Cargo.toml` is **empty** (pins untouched).
- [x] uniffi pin NOT bumped to 0.32 — `Cargo.toml:47` is `uniffi = "0.31"`, lockfile at `0.31.2`.
    NOTE: next.md's literal grep `'uniffi = { version = "0.31"'` (inline-table form) **fails**
    because the manifest uses the plain-string form `uniffi = "0.31"`; the intent is satisfied. This
    is a next.md defect, not a miss. All other held pins confirmed at pin: pyo3 0.29, criterion 0.5,
    iai-callgrind 0.16, magnus 0.7, jni 0.21, napi 3 (napi lockfile → 3.11.0, within `"3"`).
- [x] `mise run test` passes — Rust 427 tests (85+270+28+22+21+1), 0 failed; Python 314 passed
    (incl. conformance vs `iscc-core/data.json` + comparative benches).
- [x] `mise run lint` clean — `cargo fmt --check`, clippy `-D warnings` workspace-wide, ruff. The
    `proc-macro-error2 v2.0.1` future-incompat warning is pre-existing (dev-only iai-callgrind
    chain, benches only) — not introduced by this diff (lockfile shows no version change).
- [x] `mise run audit` exits 0 — "advisories ok, bans ok, licenses ok, sources ok" on the refreshed
    lockfile. No new transitive license/advisory; no `deny.toml` change.
- [x] `mise run bench:iai:check` exits 0 — 16 benchmarks within 10% of baseline, max drift +1.96%
    (`bench_mixed_code.two_codes`); blake3 1.8.3→1.8.5 did not shift hashing Ir.
    `.iai-baseline.json` NOT refreshed (unnecessary) — correct.
- [x] `mise run check` — all 15 pre-commit hooks pass (mdformat included; context files conforming
    this cycle, no reformatting needed).

**Quality gate integrity:** Clean. No suppressions, test skips, threshold reductions, or hook
weakening across the unpushed range (`@{upstream}..HEAD`). No API breaks (no source change), no perf
regression (max +1.96%, well within the 10% gate), conformance vectors green. cargo-deny passed with
no allow-list broadening. Scope disciplined: `Cargo.lock` is a generated lockfile (explicitly
excluded from the 3-file budget by next.md); the only other changes are `handoff.md` and the advance
agent's own `MEMORY.md`. No `## Not In Scope` violations.

**Issues found:** (none)

**Codex review:** Clean. "The lockfile refresh remains within existing manifest constraints, and the
accompanying metadata accurately records the change. No actionable regression introduced by this
commit was identified." Matches the local assessment.

**Next:** Continue the dependency-refresh issue with the next per-ecosystem slice. Two natural
candidates: (a) **Rust direct-pin evaluation** — document hold-back reasons next to each held major
(uniffi 0.32 needs Swift/Kotlin re-verify; pyo3 held for #41; criterion 0.7; iai-callgrind; magnus
rb_sys/Docker-tag; jni; napi) without crossing any boundary; or (b) **Python `uv.lock` refresh**
(`uv lock --upgrade`, keeping PyO3 `abi3-py310` and the `#41` `gil_used`/`py.detach` constraints in
mind). Slice (b) is the cleaner mirror of what just landed. Keep each slice a small,
single-ecosystem step — do NOT bundle into a mega-diff (no `[audit]` issue → no 8-file escape
valve). A `**Progress:**` marker is now on the dependency issue in issues.md tracking what's done
vs. remaining.

**Notes:**

- **Perf-gate tooling install-on-demand (not a bug):** the handoff flagged that valgrind +
    iai-callgrind-runner were absent from the fresh container. Verified this is the established
    pattern — CI installs both per-run in the `Perf` job (`ci.yml:289-300`); the devcontainer
    mirrors that (no bake-in). The advance agent's install commands are now documented in advance
    memory and learnings.md. Declined to file an issue. Future review agents running
    `bench:iai:check` on a fresh container: `sudo apt-get install -y valgrind` +
    `cargo binstall -y iai-callgrind-runner   --version 0.16.1`.
- **Learnings pruning:** learnings.md had drifted to ~217 lines (over the 200 budget) before this
    iteration; archived four fully-complete entries (blake3 WASM SIMD, ISCC-IDv1, #49 aarch64
    wheels, "10 vs 9 gen functions") to `learnings-archive.md` and compressed several completed-gate
    sagas — now at 200.
- No `decisions.md` entry: routine lockfile refresh, no deviation/trade-off/accepted regression to
    record.
- CI will re-run the same gates (deny, perf vs `.iai-baseline.json`, full matrix) on push; local
    tool versions matched CI (cargo-deny 0.19.9, iai-callgrind 0.16.1).
