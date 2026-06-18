# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Wire up `cargo deny`/`cargo audit` supply-chain gate `normal` [review]

`notes/07-security-versioning.md` mandates `cargo deny check` "Run in CI" (licenses + RustSec
advisories + duplicate versions) configured via a workspace-root `deny.toml`, plus `cargo audit` as
a complement. None of this exists: no `deny.toml`, no CI job, no `mise` task, and neither tool is
installed in the devcontainer. This gap surfaced concretely in iteration 105 — the PyO3 0.29 bump
(issue #1) targeted two RustSec advisories but the clearance could only be verified by the
mechanical proxy "lockfile resolves a single `pyo3 0.29.0`, no older entries", not by an actual
advisory scan. Fix: add `deny.toml` at the workspace root, a `Security audit` CI job running
`cargo deny check` (advisories + bans + licenses), and a `mise run audit` task; install via
`taiki-e/install-action` or `cargo binstall -y --force` (note the rust-cache poisoning gotcha — see
learnings "cargo binstall + Swatinem/rust-cache"). Optionally add `npm audit` for the napi package
per the same note.

**Spec:** `.claude/context/specs/ci-cd.md` → "Supply chain — `cargo-deny`" added (2026-06-18) to
mandate the gate. **AUTHORIZED by Titusz (2026-06-18)** — CID may implement autonomously
(review-sourced; the requirement previously lived only in design notes `notes/07`, now also in the
CID spec).

## Release core as v1.0.0 (stability commitment) `low` [human]

Human-driven release: cut **v1.0.0** as the first stability-committed release of the lockstep
workspace (per the 1.0.0 decision). This is the one release allowed to break the 0.4.0 API freely;
afterward 1.x is locked under strict SemVer. Drive via the `/release` skill — do NOT let the CID
loop cut this release autonomously. Ideally land both the `cargo-semver-checks` and `iai-callgrind`
gate issues above first so 1.0.0 ships with enforcement active.

**Status (2026-06-18):** Titusz decided to **hold** the v1.0.0 cut and stay on 0.4.x for now — land
the CRAP `--fail-above` and `cargo deny` hardening gates first, then flip the `cargo-semver-checks`
gate to enforcing as part of the eventual cut. Remains `low` `[human]`; CID must not cut it
autonomously.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
