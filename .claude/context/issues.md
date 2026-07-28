# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`, `[audit]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external
repo). The review agent deletes resolved issues after verification (history in git). `[audit]`
entries carry a `**Scope estimate:**` — a step citing one may modify up to 8 non-test files.

**This is a defect tracker, not a work queue.** An entry states the remaining problem and how to
tell it is fixed, in **40 lines or fewer**, and is deleted once nothing is left to fix. Never record
progress inside an entry — no per-iteration checklists, no "✅ slice N done", no plan of remaining
sub-steps. Git history records what was done; a step queue here removes the gap analysis that
define-next exists to perform. If only part of a problem remains, rewrite the entry to describe that
smaller problem. Design rationale belongs in `decisions.md`, operational gotchas in `learnings.md`,
and user-facing behaviour in `docs/`.

<!-- Add issues below this line -->

## Dependency review and refresh across the project `normal` [human]

Planned for the **v0.6.0** release. No automated dependency updates are configured (no
Dependabot/Renovate), so manifests drift between releases.

Two items remain. Both were **authorized by Titusz on 2026-07-28** and are schedulable as separate
steps:

- **`criterion` 0.7 → 0.8 — take it and keep `rust-version = "1.85"`.** This is not an MSRV policy
    call: `criterion` is a dev-dependency of `crates/iscc-lib` only, consumed by the two `[[bench]]`
    targets, and a dev-dependency is never built by downstream consumers, so its rustc-1.86 floor
    cannot raise a consumer's MSRV. It raises only the contributor/bench toolchain floor, and every
    Rust CI job runs `dtolnay/rust-toolchain@stable`. Do **not** edit `rust-version` in this step.
- **`uniffi` 0.31 → 0.32 — regenerate, do not hand-edit.** Read the 0.32 changelog first: if the
    proc-macro surface changed such that `crates/iscc-uniffi/src` needs edits, stop and re-scope —
    that is a different step. Otherwise regenerate both checked-in bindings
    (`packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`); the diff must be a pure
    regeneration. Kotlin is verifiable locally (Gradle/JVM). **Swift is not** — the Linux
    devcontainer has no Swift toolchain, so the `swift` CI job is the verification, and the step is
    not done until CI is green on the pushed commit. uniffi's runtime checksum check fails loudly on
    a stale binding, so a missed regeneration reds the suites rather than passing silently.

GitHub Action refs are **current** — re-probed 2026-07-28 across all 25 distinct `uses:` refs in
`.github/workflows/*.yml`: `git/matching-refs/tags/v<N+1>` is empty for all 23 versioned refs, and
both exact pins equal their publisher's `releases/latest`. Re-probe before assuming drift; do not
scope an action bump on a release note alone.

**Constraints that still bind:** wheels stay `abi3-py310`; PyO3 bumps only together with
re-verifying `gil_used = true` semantics and the `py.detach` call sites; `rb_sys` in `Gemfile.lock`
must match the `oxidize-rb/actions/cross-gem` Docker image tag (exact-pinned `0.9.123`);
quality-gate CI tool pins bump together with their baselines. Never run `ruff@0.16 check --fix .` —
it deletes load-bearing `# noqa` directives. All quality gates and conformance vectors must pass on
the refreshed set.

**Known upstream wart (not actionable):** the `proc-macro-error2 v2.0.1` future-incompat warning on
every `cargo test` / `cargo bench` comes from `iai-callgrind-macros`, is dev-only, and has no fixed
release. Re-check when bumping `iai-callgrind` (which must stay in lockstep with the CI-installed
`iai-callgrind-runner`).

**Spec:** `.claude/context/specs/ci-cd.md` → "Dependency Freshness"

## go1.27 bump reds the Go boundary suite unless the freeze table lands with it `normal` [review]

**Trigger-on-bump record — do not scope a step until go1.27 is available (~Aug 2026).**

Bumping `packages/go/go.mod` past `go 1.26.1` **must** land the 731-range freeze table in
`packages/go/utils.go` in the *same* step. Go currently passes the two post-16.0 boundary vectors
for the wrong reason: its Unicode 15.0 tables classify `U+20C1` and `U+A7F1` as `Cn`, so the
category-`C` filter drops them, coinciding with the freeze rule's output by accident. Under go1.27
both become assigned (`x/text`'s `tables17.0.0.go` is `//go:build go1.27`; the stdlib `unicode`
tables move too), so 5 currently-green cases flip red — `text_clean` + `text_collapse` for `U+20C1`
and `U+A7F1`, plus `text_clean/…_seq_ua7f1_no_decomposition_leak` — while the 3 documented skips
become unnecessary.

Do **not** version-gate the skip map to hide this; the red is the intended signal.
`go-version-file: packages/go/go.mod` pins CI to the go.mod directive, so nothing flips without a
deliberate bump.

## Update the upstream iscc-core thread with the sentinel mechanism `low` [human]

**Human-only — CID must not act on this.** The Unicode data version work is complete on this side:
the sentinel freeze rule, the fail-closed differential sweep gate, boundary vectors on all 11 native
surfaces plus pure-Go, and user documentation in `docs/unicode.md` all landed.

<https://github.com/iscc/iscc-core/issues/137> still describes the earlier framings. The thread
needs updating to propose the **sentinel** mechanism —
`unicodedata2==16.0.0; python_version < '3.14'` + import shim, map unassigned code points to
`U+FFFF` before `unicodedata.normalize` using the same 731 vendored ranges, leave the category-`C`
filter alone, and add boundary vectors to `data.json`. Both the sequence-delta framing and the
pre-normalization-removal framing should be withdrawn.

Worth stating explicitly upstream: `iscc-core` output has never been deterministic across its
declared `>=3.9,<4.0` range (Unicode 13.0/14.0/15.0/15.1/16.0 by Python version), so this is a
determinism fix and the behavioural break it implies is accepted deliberately. Per ISO 24138 Annex D
the reference implementation is normative, so the `iscc-core` release adopting this settles the
standard's answer.

Close this issue once the thread is updated.

**Upstream:** iscc/iscc-core **Spec:** `.claude/context/specs/rust-core.md` → "Unicode data version
is part of the conformance contract"

## Gate-script remainders deliberately deferred at iteration 146 `low` [review]

Three items left open when the gate blind-spot issues closed, each contingent on something that has
not happened yet — CID must not act on them unprompted:

1. **A zero-resolved run still exits 0.** The condition is *visible*
    (`action-inputs: resolved <R> of <T> action refs`) but not fatal. Flipping the exit code is a
    policy change against `decisions.md` 2026-07-26, which accepted all-skipped-as-green by design.
    **Trigger:** the first `release-workflow` CI run whose log shows a non-zero skip count. **Needs
    Titusz** — it turns third-party availability into a merge blocker.
2. **Job-level `uses:` (reusable-workflow calls) is still unscanned** — only `job.steps[*].uses`.
    `release.yml` has none at HEAD; the `owner/repo/.github/workflows/x.yml@ref` form also needs a
    different URL shape than `action_yml_urls` builds. **Trigger:** a reusable workflow is added to
    any workflow file.
3. **The YAML 1.1 boolean fix is one-sided.** Non-string `with:` keys are now skipped, but the
    *metadata* side is not normalized: an action declaring an input literally named `on`/`off`/
    `yes`/`no` parses as `True`/`False`, so a workflow passing the **quoted** `"on"` would still be
    reported undeclared. No such action exists among the 18 refs. **Trigger:** the false positive
    is actually observed.

## Release core as v1.0.0 (stability commitment) `low` [human]

Human-driven release: cut **v1.0.0** as the first stability-committed release of the lockstep
workspace (per the 1.0.0 decision). This is the one release allowed to break the 0.4.0 API freely;
afterward 1.x is locked under strict SemVer. Drive via the `/release` skill — do NOT let the CID
loop cut this release autonomously.

**Status (2026-07-28):** Titusz reaffirmed **not yet** — the project is working toward **v0.6.0**,
and v1.0.0 comes after it. The two hardening gates the 2026-06-18 hold waited on have both landed
and are enforcing (`ci.yml` → `cargo crap … --fail-regression --fail-above` with a committed
`.crap-baseline.json`, and `Audit (cargo-deny)`), so they are no longer the blocker. What remains
for the cut itself: flip `cargo-semver-checks` from `continue-on-error: true` to enforcing, and
settle the MSRV question (see the MSRV issue below). Do not re-derive this hold from the gate status
— it is a release-sequencing decision, not a technical one.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## The declared MSRV is asserted but never verified `low` [human]

**Human-gated — a v1.0.0 prerequisite, not v0.6.0 work. CID must not act on this unprompted.**

The workspace declares `rust-version = "1.85"` (root `Cargo.toml`, inherited by all 8 crates), but
nothing tests that the crate actually builds on 1.85: every Rust job in `ci.yml` uses
`dtolnay/rust-toolchain@stable`, there is no `rust-toolchain.toml`, and `resolver = "2"` is not
MSRV-aware. The claim is therefore unfalsifiable today — tolerable at 0.5.0, not at v1.0.0, where
MSRV becomes part of the stability commitment.

**Scope when it lands:** add a CI job on `dtolnay/rust-toolchain@1.85` running
`cargo check -p iscc-lib`. It must skip dev-dependencies (`cargo check`, not `cargo test`) — once
`criterion` 0.8 is in, the dev-dependency graph needs rustc 1.86 by design, and that is not an MSRV
violation.

Resolved when either the job exists and passes, or `rust-version` is dropped/raised deliberately.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Migrate npm publishing to OIDC Trusted Publishing `low` [human]

**DEFERRED by Titusz 2026-07-26 — not for v0.6.0.** The v0.6.0 release stays on `NPM_TOKEN`, which
was rotated (`iscc-lib-ci-2026`, granular `@iscc` scope) and is valid until 2026-09-16. Revisit
before the release after v0.6.0, or sooner if the token expiry becomes a problem again. CID must
**not** prepare the YAML diff — an unused OIDC-shaped diff sitting in the tree would be its own
hazard.

The v0.5.0 release failed both npm publishes because `NPM_TOKEN` had expired (npm caps write-token
expiry at 90 days). npm now supports OIDC Trusted Publishing — the same keyless mechanism already
used for crates.io, PyPI and RubyGems.

**Scope when it lands:** update the two npm publish jobs in `.github/workflows/release.yml` to
publish via OIDC (drop `NODE_AUTH_TOKEN`/`NPM_TOKEN`, rely on the existing `id-token: write` +
`npm publish --provenance`). **Human-gated:** a Trusted Publisher must be configured for each
package on npmjs.com before the token can be removed.

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
