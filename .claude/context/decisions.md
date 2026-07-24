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
