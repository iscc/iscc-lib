## 2026-06-18 — Enforce CRAP absolute gate via `--fail-above`

**Done:** Wired the authorized `--fail-above` boolean onto the enforcing "CRAP regression gate" CI
step so any function whose CRAP score exceeds the configured threshold (30) now fails CI — including
brand-new or renamed functions absent from the baseline that regression mode alone reported as
`★ N new` and exited 0. Updated the `.cargo-crap.toml` header/threshold comments and the ci-cd.md
verified-when checkbox to match. Closes the first of the two authorized `[review]` hardening issues.

**Files changed:**

- `.github/workflows/ci.yml`: appended `--fail-above` to the Phase 3 CRAP gate command (final form
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`) and
    rewrote the preceding comment block to describe the combined regression + absolute gate. yamlfix
    folded the long `run:` scalar onto two lines (semantically unchanged — folded newline = space).
- `.cargo-crap.toml`: replaced the stale "Phase 2 runs report-only: no `fail-above` is set" /
    "report-only — no build failure" comments with text describing `--fail-above` as the enforcing
    Phase 3 gate keyed off `threshold = 30.0`.
- `.claude/context/specs/ci-cd.md`: flipped the `--fail-above` verified-when box (line 445) from
    `[ ]` to `[x]`. The §"Phased rollout" prose (lines 90-102) already described the enforcing gate,
    so it was already consistent — no change needed there.

**Verification:**

- `grep -- '--fail-above' .github/workflows/ci.yml` → present on the gate step (lines 387, 393).
- `.cargo-crap.toml` grep for "no `fail-above` is set" / "report-only — no build failure" → no
    matches (stale claims removed).
- `grep -n 'fail-above' .claude/context/specs/ci-cd.md` → line 445 now `[x]`.
- Local end-to-end: `mise run coverage` regenerated `lcov.info`, then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exited **0** (`0 regressed, 0 new, 97 unchanged`; max CRAP ~22.3 < 30 threshold).
- `uv run prek run --files .github/workflows/ci.yml .cargo-crap.toml .claude/context/specs/ci-cd.md`
    passes (YAML/TOML/markdown valid; yamlfix applied its formatting then is idempotent on re-run).

**Next:** The SECOND authorized `[review]` issue — the `cargo-deny`/`cargo audit` supply-chain gate
(add a root `deny.toml`, an Audit CI job running `cargo deny check`, and a `mise run audit` task).
Explicitly out of scope here. v1.0.0 cut stays on hold (`low` `[human]`), and the
`Semver (cargo-semver-checks)` job stays `continue-on-error` until that cut.

**Notes:** Pure CI/config + doc change — no Rust source, no public API, no hot path touched, so no
benches run. The `--fail-above` flag takes no numeric argument; it keys off the `.cargo-crap.toml`
`threshold` (30) and combines cleanly with `--fail-regression --baseline` (verified locally on
cargo-crap 0.2.2). `.crap-baseline.json` is unchanged (not in scope; max 22.27 well under 30). The
local `mise run crap` task is deliberately left report-only — only CI enforces. `iterations.jsonl`
shows modified in git status (runner-managed) and was intentionally NOT staged.
