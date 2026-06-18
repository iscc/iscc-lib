# Next Work Package

## Step: Enforce CRAP absolute gate via `--fail-above`

## Goal

Close the regression-only blind spot in the Phase 3 CRAP gate by adding the authorized
`--fail-above` flag to the enforcing CI step, so a brand-new or renamed function whose CRAP score
exceeds the configured threshold (30) fails CI instead of slipping through as `★ N new` and exiting
0\. This implements the first of two AUTHORIZED `[review]` hardening issues ("CRAP gate does not fail
on new high-CRAP functions").

## Scope

- **Modify**: `.github/workflows/ci.yml` — append `--fail-above` to the "CRAP regression gate" step
    (currently line 390) and refresh its preceding comment block (lines 385-388) to describe the
    combined regression + absolute gate.
- **Modify**: `.cargo-crap.toml` — update the header/`threshold` comments (lines 5-9) that currently
    claim "Phase 2 runs report-only: no `fail-above` is set" to reflect that `--fail-above` is now
    wired into the enforcing CI gate keyed off `threshold = 30.0`.
- **Modify**: `.claude/context/specs/ci-cd.md` — flip the "verified when" box at line 445 (CRAP
    `--fail-above`) from `[ ]` to `[x]`.
- **Reference**: `.claude/context/issues.md` (the AUTHORIZED CRAP issue), `.cargo-crap.toml`,
    `.crap-baseline.json` (97 entries, max CRAP 22.27 < 30), `.claude/context/specs/ci-cd.md` §"Rust
    Coverage and CRAP Quality Gate" → Phase 3, `mise.toml` (`[tasks.coverage]`, `[tasks.crap]`).

## Not In Scope

- The `cargo-deny`/`cargo audit` supply-chain gate — that is the SECOND authorized issue and a
    separate future step (do NOT add `deny.toml`, an Audit job, or a `mise run audit` task here).
- Flipping the `Semver (cargo-semver-checks)` job to enforcing (`continue-on-error: true` stays) —
    deliberately held until the v1.0.0 cut.
- Cutting v1.0.0 or bumping the workspace version.
- Changing the local `mise run crap` task — it stays report-only (human format) as a diagnostic; the
    enforcing gate lives only in CI.
- Regenerating `.crap-baseline.json` — the baseline is unchanged (max 22.27 is well below 30).
- Tightening the `threshold` value or editing `.cargo-crap.toml` exclude globs.

## Implementation Notes

- The exact final CI command is:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`.
    `--fail-above` is a boolean flag (NO numeric argument) — it keys off `threshold = 30.0` already
    set in `.cargo-crap.toml`. Verified locally (cargo-crap 0.2.2): `--fail-above` and
    `--fail-regression` are independent booleans that combine cleanly with `--baseline`.
    (`--baseline` is incompatible only with `--format sarif`, which is a separate report-only step —
    no conflict.)
- The two flags are complementary: `--fail-regression` blocks PRs that worsen an existing baselined
    function (tolerating pre-existing debt); `--fail-above` fails ANY function above threshold,
    including new/renamed ones absent from the baseline. Keep both.
- The change is safe: current baseline max CRAP is 22.27, well below 30, so the absolute gate passes
    on the current code and does not break CI.
- Keep YAML 2-space indentation consistent with the surrounding `coverage` job. Run
    `mise run format` before committing so the pre-commit hooks (YAML/TOML validation, whitespace)
    don't rewrite the files after staging.

## Verification

- `grep -- '--fail-above' .github/workflows/ci.yml` shows the flag on the "CRAP regression gate"
    step alongside `--fail-regression --baseline .crap-baseline.json`.
- `.cargo-crap.toml` no longer contains the phrase "no `fail-above` is set" / "report-only" for the
    threshold (grep returns no stale claim).
- `grep -n 'fail-above' .claude/context/specs/ci-cd.md` shows the verified-when line at ~445 marked
    `[x]`.
- Local end-to-end pass: `mise run coverage` (regenerates `lcov.info`) then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 (no regression, max score < 30).
- `uv run prek run --files .github/workflows/ci.yml .cargo-crap.toml` (or `mise run check`) passes —
    YAML/TOML stay valid.

## Done When

The enforcing CRAP CI step runs with `--fail-above` keyed off threshold 30, the
`.cargo-crap.toml`/`ci-cd.md` text reflect the enforcing absolute gate, the local end-to-end CRAP
command exits 0, and edited files pass the pre-commit hooks.
