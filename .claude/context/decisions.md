# Decision Records

Append-only log of judgment calls that a future reader could not reconstruct from the code alone:
deviations from the iscc-core reference or a spec, design trade-offs between real alternatives,
accepted performance regressions, justified suppressions, API-shape choices. Written by the review
agent when approving such a call; human entries welcome. Never rewrite or delete entries — the
history is the point. Distinct from learnings.md: learnings capture operational gotchas, decisions
capture design rationale.

Entry format:

```markdown
## <YYYY-MM-DD> — <title>

**Decision:** <what was decided>
**Why:** <the reasoning, including constraints that forced it>
**Alternatives:** <what was rejected and why>
**Context:** <iteration N / commit sha / discussion reference>
```

<!-- Append entries below this line -->

## 2026-07-24 — Audit role triggered by iteration cadence, not IDLE

**Decision:** The codebase audit role runs after every 10th completed iteration (plus on demand via
`mise run cid:audit`), rather than on IDLE like meta-improve or on a calendar. **Why:** Debt
accumulates fastest during busy development phases — exactly when IDLE never fires. A counter from
iterations.jsonl survives restarts and needs no clock. Findings land in issues.md right before the
next define-next, so the loop consumes them immediately. **Alternatives:** IDLE-coupling (audits
would pause during the phases that create the debt); manual-only (relies on human discipline, which
decays over months); every 5 iterations (more cost, re-finds issues the loop has not yet had time to
fix). **Context:** Interactive session with Titusz, 2026-07-24; implemented alongside this file.

## 2026-07-24 — Audit is find-and-file only, runner-enforced

**Decision:** The audit role may only change issues.md and its own agent memory; the trusted runner
(`enforce_audit_safety` in tools/cid.py) reverts anything else. Fixes flow through the normal
define-next → advance → review cycle. The audit orchestrates a parallel finder/adversarial-verify
workflow rather than using the generic high-stakes harness. **Why:** Free-form "refactoring
sessions" are where agentic projects introduce regressions — refactoring without a gate is churn.
Audit finds, the gated loop fixes. Adversarial verification (2-of-3 skeptics) keeps false positives
from steering iterations. The high-stakes skill defends a single deliverable; the audit needs
multi-dimension discovery — a different harness shape. **Alternatives:** Letting the audit fix
directly (bypasses the review gate); high-stakes skill per audit (rigor lands in the wrong place,
higher cost). **Context:** Interactive session with Titusz, 2026-07-24.

## 2026-07-24 — Scope-cap escape valve: 8 files, only for audit-cited steps

**Decision:** define-next may scope up to 8 non-test, non-doc files (instead of 3) only when the
step cites an `[audit]`-tagged issue; the review agent verifies the citation and fails the step
otherwise. **Why:** The 3-file cap systematically selects against structural fixes — the fix for an
abstraction is always bigger than a point patch, so cross-cutting cleanups (e.g. across the 8
binding crates) were unpickable by design. A hard ceiling keeps refactor steps reviewable in one
sitting. **Alternatives:** No exception (structural debt stays unpickable); uncapped audit steps
(loses the mechanical tripwire that kept 120 iterations reviewable); pre-decomposing every refactor
into 3-file steps (many refactors cannot be split while keeping every intermediate tree green).
**Context:** Interactive session with Titusz, 2026-07-24.

## 2026-07-24 — Tightening decode input-validation is not a SemVer break

**Decision:** Adding the `tail.len() > nbytes` "too long" rejection to the Tier 1,
stability-committed `iscc_decode` (and, iter 120, Go `IsccDecode`) is classified as a robustness bug
fix, not an API break — no `**API-BREAK:**` flag required, and it need not wait for the v1.0.0 cut.
**Why:** The signature and return-tuple shape are unchanged; only the set of *accepted* inputs
shrinks, and every input newly rejected was already malformed (trailing base32 chars that silently
aliased a canonical code, e.g. `ISCC:...AB` == `ISCC:...ABAA`). Narrowing the accepted domain to
reject previously- mis-decoded garbage is a correctness fix, and every conformance vector still
round-trips. **Alternatives:** Treat as breaking and defer to v1.0.0 (would leave a codec-wide
aliasing bug live across all 11 bindings for months, for no user benefit); keep truncating silently
(data-integrity hazard — two distinct strings decode identically). Note for the eventual
`cargo-semver-checks` enforcing gate: input-domain narrowing is invisible to signature-based semver
tooling, so this call is a human judgment, not a tool result. **Context:** iter 121, commit 044d0cc
(Rust); iter 120 (Go).

## 2026-07-25 — `astral-sh/setup-uv` pinned to the exact tag `@v9.0.0`

**Decision:** Every other GitHub Action in `ci.yml` / `docs.yml` is pinned to a floating major
(`@v7`, `@v5`, …), but `astral-sh/setup-uv` is pinned to the exact release tag `@v9.0.0`, with a
two-line YAML comment above each of the two occurrences explaining why. **Why:** upstream stopped
publishing floating major tags after `v7` — `git/matching-refs/tags/v` lists v1–v7 plus exact
v8.x/v9.0.0 tags only, and setup-uv's own README pins exact refs. `@v9` does not resolve; CI proved
it (`Unable to resolve action astral-sh/setup-uv@v9` on commit ebac57f). The accepted cost is that
setup-uv patch releases are no longer picked up automatically, so this pin must be bumped by hand in
each dependency-refresh pass (it recurs in `release.yml`). **Alternatives:** stay on `@v4` (leaves
the action 5 majors and a node runtime behind, defeating the refresh); pin to `@v7` to keep a
floating tag (freezes on an EOL major purely for convention); pin to a commit SHA (inconsistent with
the other 15 refs in these files, and next.md explicitly excluded SHA pinning). **Context:** iter
127, commits ebac57f → 8f76d48; deviation from next.md's `@v9` recommendation.

## 2026-07-25 — JVM manifest refresh accepted despite the raised Kotlin consumer floor

**Decision:** The iter-128 JVM dependency refresh (11 pins across `pom.xml` and `build.gradle.kts`)
was approved and pushed to `develop` even though the `kotlin("jvm")` 2.1.10 → 2.4.10 bump verifiably
breaks consumers compiling with Kotlin < 2.3 (published metadata `mv=[2,4,0]`; 2.1.10 and 2.2.21
fail, 2.3.21 passes). The support-floor question was split out as a `normal` `[review]` issue with
HUMAN REVIEW REQUESTED instead of blocking the slice. **Why:** the break cannot reach a user from
`develop` — `io.iscc:iscc-lib-kotlin` only ships via the human-dispatched `release.yml`, so there is
a natural gate before harm, and pushing lets the 41-job CI validate the other 10 pins (the real gate
for this slice). Choosing the floor is a support-policy decision of the same class as MSRV and
`java-version: '17'`, which the step's own scope explicitly reserved for the owner; a
reviewer-forced revert would have made that choice by default. **Alternatives:** NEEDS_WORK + revert
the Kotlin line to 2.1.x (silently picks "preserve the old floor" without the owner, and denies the
other pins their CI validation); pin `compilerOptions.languageVersion` to 2.1 (insufficient — the
transitive `kotlin-stdlib:2.4.10` in the published POM raises the same error on its own); HUMAN
REVIEW REQUESTED on the handoff to pause the runner (disproportionate — the loop still has
actionable refresh slices, and the issue carries the escalation). **Context:** iter 128, commit
be0a497; found independently by this review and by the Codex second opinion, then confirmed
empirically against consumer projects on Kotlin 2.1.10 / 2.2.21 / 2.3.21.

## 2026-07-25 — Go `x/text` refresh accepted on exhaustive differential evidence; Unicode divergence split out

**Decision:** The iter-129 Go module refresh (`golang.org/x/text` 0.34.0 → 0.40.0) was approved on
the strength of an exhaustive differential — `TextClean`/`TextCollapse` are byte-identical for all
1,112,032 code points under both versions — rather than on the green conformance vectors alone. The
Unicode-version divergence uncovered while establishing that baseline (Rust core keeps the 5,813
code points assigned in Unicode 16/17; Go and `iscc-core` strip them, so Meta/Text codes differ) was
filed as its own `normal` `[review]` issue with HUMAN REVIEW REQUESTED instead of downgrading this
verdict. **Why:** `x/text` ships Unicode tables and feeds NFKC in every `Gen*CodeV0`, so a
vector-green result proves only that the 50 vendored vectors are unaffected — all of them predate
Unicode 16, which is exactly the region where implementations disagree. The differential converts
"no vector regressed" into "no input can regress", which is the claim the refresh actually needs.
The divergence itself is pre-existing, is not reachable from this diff, and picking a Unicode
version is a conformance-policy call with an upstream (ISO 24138 / `iscc-core`) dimension that will
flip direction again when CPython ships Unicode 16 — not something a dependency-refresh slice may
settle. **Alternatives:** accept on vectors alone (cheap, but would have left the divergence
undetected and the bump unjustified in the one dimension that matters); NEEDS_WORK on the slice
until the Unicode question is resolved (punishes a clean, output-neutral refresh for a defect it did
not introduce, and stalls the remaining slices); pin the Rust core to Unicode 15 immediately
(unilaterally sets conformance policy, and re-breaks when the reference moves). **Context:** iter
129, commit 4404fba; divergence reproduced against Go, the Rust core via the Python binding, and
`iscc-core` 1.3.0 side by side.

## 2026-07-25 — ruff 0.16 findings are fixed at the source, never suppressed; stub bodies go docstring-only

**Decision:** The ruff 0.16 adoption resolves every finding by changing the offending code or, where
that is impossible, by an explicit lint-config decision — never by `# noqa`, `per-file-ignores` or
`ignore` entries. Concretely for slice A: the 36 lone `...` placeholders were **deleted** from the
published `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`, making a docstring the entire stub body,
and that becomes the recorded convention for every future Tier 1 symbol. **Why:** `_lowlevel.pyi`
ships in the wheel next to `py.typed`, so it is a consumer-facing artifact — the removal was
therefore validated beyond the repo's own gate (`ty`) against the two dominant third-party checkers,
`mypy 1.18 --strict` and `pyright 1.1.407`, both clean. Suppressing instead would have been
indistinguishable from gate weakening under this project's rules, and would have locked `ruff<0.16`
in place permanently. The same principle pre-decides the hardest remaining finding: the 15 `RUF100`s
sit on the `# noqa: S603/S607` directives that the pre-push `ruff check --select S` hook depends on,
so they must be resolved by widening `[tool.ruff.lint] select` (making `S`/`C901` first-class), not
by deleting the directives that keep the security gate green. **Alternatives:** add
`PIE790`/`PYI048` to `per-file-ignores` for `*.pyi` (one line, but it is a suppression and leaves
the stub in a form ruff will keep flagging); keep the `ruff<0.16` pin indefinitely (freezes the
Python toolchain and defers an ever-growing diff); run `ruff@0.16 check --fix .` wholesale (would
silently delete the security `# noqa`s and red the pre-push `S` gate). **Context:** iter 131, commit
0544797; whole-tree 0.16 findings 104 → 26, pin deliberately retained.
