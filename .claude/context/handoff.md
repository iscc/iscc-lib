# Handoff

## 2026-07-24 — Review of: Refresh `.crap-baseline.json` for the iter-121 `iscc_decode` change (fix RED CI)

**Verdict:** PASS

**Summary:** The advance agent regenerated the committed CRAP regression baseline via the documented
path so the enforcing `Coverage + CRAP` CI job goes green while the iter-121 `iscc_decode`
trailing-byte fix stays in place. No source files changed; only the `iscc_decode` baseline entry's
substantive metrics moved (cyclomatic 4→5, coverage 80.95→85.19, crap 4.11→5.08), everything else is
a pure `+7` line shift. I independently regenerated coverage and ran the exact CI gate — it exits 0
with `↑ 0 regressed`.

**Verification:**

- [x] `mise run coverage` produces a fresh `lcov.info` (exit 0) — regenerated locally, exit 0
- [x] `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 — confirmed EXIT 0, summary
    `↑ 0 regressed ↓ 0 improved ★ 0 new ↔ 0 moved · 97   unchanged — 0 removed`, max CRAP
    `gen_meta_code_v0` 22.3 (under the 30.0 cap)
- [x] `git diff .crap-baseline.json` shows `iscc_decode` cyclomatic no longer `4.0`, crap no longer
    `4.110571212612029` — now `cyclomatic 5.0 / crap 5.0812884214804654`, entry re-sorted by CRAP
    desc; every other lib.rs entry is a pure `line: +7` shift (no substantive drift)
- [x] `cargo test -p iscc-lib` still passes — 321 total (270 unit + 28 + 22 integration + 1
    doctest), 0 failed
- [x] `cargo clippy -p iscc-lib --all-features --all-targets -- -D warnings` clean — only the
    pre-existing transitive `proc-macro-error2` future-incompat note (unrelated)
- [x] `mise run check` — all 15 pre-commit hooks Passed

**Issues found:** (none) — scope is exactly `.crap-baseline.json` + handoff + advance memory; no
source/gate-config touched. Gate-circumvention scan over all 4 unpushed commits is clean (no
threshold/epsilon lowering, no `--fail-above` removal, no suppression). The source robustness fix
was correctly preserved, not reverted.

**Codex review:** Confirmed the regenerated baseline "accurately reflects the intended iscc_decode
complexity change and corresponding source-line shifts" and that the enforcing command passes with
all 97 entries unchanged. No actionable findings.

**Next:** CI should go green on this push (the `Coverage + CRAP` job was the only red job). After
this lands green, the CID-doable v0.6.0 queue is: (1) **#49 aarch64 Python wheels** — plan at
`.claude/plans/restore-linux-aarch64-python-wheels.md`; add an `ubuntu-24.04-arm` native-ARM entry
to the `build-wheels` matrix in `.github/workflows/release.yml` and extend the wheel test job. (2)
**Project-wide dependency review/refresh** (root `Cargo.toml`/lock, `pyproject.toml`/`uv.lock`,
binding manifests, tooling pins) — mind the documented pinning constraints (PyO3/gil,
rb_sys/cross-gem, abi3-py310, quality-gate tool pins bump with baselines). Both `normal` `[human]`,
both unblocked. Do NOT cut v1.0.0 or flip `Semver` to enforcing (both human-held).
Release-reliability issues (npm OIDC, single-registry re-trigger) stay human-gated.

**Notes:**

- **CI-only gate guard gap (root cause of this incident):** the CRAP `--fail-regression` gate runs
    ONLY in CI, not in `mise run check`/pre-commit — which is exactly why iter 121's legitimate
    branch-adding fix slipped through green locally then reddened CI. Recorded in learnings.md
    (CI/CD) and both agent memories. Discipline rule for define-next/advance: **any source change
    that adds a branch/loop to a covered function must refresh `.crap-baseline.json`
    (`mise run crap:baseline`) in the same step.** Adding coverage to pre-push was considered but
    rejected as too slow for the commit path — the discipline note is the right mitigation.
- Tooling: cargo-crap 0.2.2 / cargo-llvm-cov 0.8.7 (the exact CI versions) were on PATH this session
    (advance installed cargo-crap from source — the binstall binary needs GLIBC_2.39, container is
    older). Local coverage matched CI expectations (`97 unchanged`), so devcontainer↔CI drift is
    unlikely, but the push's CI `Coverage + CRAP` job is the final confirmation.
- `.claude/context/iterations.jsonl` shows modified in the working tree (runner-managed); left
    unstaged per protocol.
