## 2026-06-18 — Review of: Enforce CRAP absolute gate via `--fail-above`

**Verdict:** PASS

**Summary:** The advance agent wired the authorized `--fail-above` boolean onto the enforcing Phase
3 CRAP CI gate, closing the regression-only blind spot where a brand-new/renamed high-CRAP function
reported `★ N new` and exited 0. The change is gate *strengthening* (not circumvention), tightly
scoped to `ci.yml` + `.cargo-crap.toml` comments + the `ci-cd.md` checkbox, and verified clean
end-to-end. Closes the first of the two authorized `[review]` hardening issues.

**Verification:**

- [x] `grep -- '--fail-above' .github/workflows/ci.yml` — present on the "CRAP regression gate" step
    (line 393); `yaml.safe_load` confirms the folded scalar resolves to
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
- [x] `.cargo-crap.toml` no longer contains "no `fail-above` is set" / "report-only — no build
    failure" — grep returns no stale claims; TOML still valid
- [x] `grep -n 'fail-above' .claude/context/specs/ci-cd.md` — verified-when box at line 445 now
    `[x]`
- [x] Local end-to-end:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression --fail-above`
    exits **0** (`0 regressed · 0 new · 97 unchanged`; max CRAP ~22.3 < 30 threshold) against the
    freshly regenerated `lcov.info`
- [x] `mise run check` — all 15 pre-commit hooks pass (YAML/TOML/markdown valid, formatting
    idempotent)
- [x] Scope discipline — diff touches only the 3 in-scope files (+ handoff/advance-memory); no
    `deny.toml`, no Audit job, no semver flip, no version bump, `.crap-baseline.json` unchanged
- [x] Gate-circumvention scan across all 7 unpushed commits (`@{upstream}..HEAD`) — clean; only
    CI/config change is the `--fail-above` addition (strengthening)

**Issues found:**

- (none)

**Codex review:** No blocking issues. Codex confirmed the CI YAML parses to the intended cargo-crap
command, that cargo-crap 0.2.2 supports `--fail-above` using the configured threshold, and that the
remaining comment/doc updates are consistent.

**Next:** Implement the SECOND authorized `[review]` issue — the `cargo-deny`/`cargo audit`
supply-chain gate. Add a workspace-root `deny.toml`, a `Security audit` CI job running
`cargo deny check` (advisories + bans + licenses), and a `mise run audit` task. Install the tool in
CI via `taiki-e/install-action` or `cargo binstall -y --force` — mind the `Swatinem/rust-cache`
poisoning gotcha (`--force` is load-bearing). Optionally complement with `cargo audit` / `npm audit`
(napi). Flip the corresponding `ci-cd.md` "verified when" box only once CI confirms the new job is
green (its enforcement is CI-only — `cargo-deny`/`cargo-audit` are NOT installed in the
devcontainer, so local verification is limited to `deny.toml` parse + `mise run audit` task wiring;
treat the green CI job as the real confirmation, like the post-push step for other CI-only gates).

**Notes:**

- Pure CI/config + doc change — no Rust source, no public API, no hot path touched, so benches were
    correctly skipped.
- `--fail-above` is a boolean with NO numeric argument; it keys off `.cargo-crap.toml`
    `threshold = 30.0` and combines cleanly with `--fail-regression --baseline` (verified on
    cargo-crap 0.2.2). Current max CRAP ~22.3 is well under 30, so the absolute gate passes on the
    current code without breaking CI.
- yamlfix folds the long `run:` scalar onto two physical lines — the folded newline is a space, so
    the effective command is single-line. Always confirm via `yaml.safe_load`, not raw grep.
- Remaining issues after this iteration: 1 `normal` `[review]` (cargo-deny gate, AUTHORIZED,
    actionable) + 2 `low` `[human]` (v1.0.0 cut held; docs logos cosmetic). The loop is NOT idle —
    there is one autonomous work package left before the human-gated v1.0.0 milestone.
- Do NOT cut v1.0.0 or flip the `Semver (cargo-semver-checks)` gate to enforcing — both are
    deliberately held by Titusz until after the cargo-deny gate lands.
