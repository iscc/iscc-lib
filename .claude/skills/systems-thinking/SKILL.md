---
name: systems-thinking
description: >-
  Apply systems thinking to a problem, decision, or situation. Classifies the
  problem as a clear, complicated, complex, or chaotic system, diagnoses it with
  the DART framework (Deconstruct, Analyze, Recognize, Test), surfaces hidden
  parts/connections/patterns and traps (incentive/cobra effects, delayed feedback
  loops), and recommends the action protocol that fits the system type. Use when
  asked to "apply systems thinking", "think in systems", "what kind of problem is
  this", "how should I approach this", "analyze this decision/situation", "should
  I X or Y", or when facing a messy call where the right approach is unclear. Do
  NOT use for routine tasks that already have an obvious, agreed solution.
user-invocable: true
argument-hint: <problem, decision, or situation to analyze>
---

# Systems Thinking

Read a situation before acting on it. A **system** is a set of connected parts that keep producing a
pattern — a company, a career, a marriage, a coffee shop. Most expensive mistakes come from acting
on the visible parts while holding the wrong model of the pattern. This skill runs a given prompt
through a diagnostic that names the system, classifies its type, and matches the right protocol —
because bringing the right approach to the wrong kind of system is itself the error.

> The four-type model below is the **Cynefin framework** (Dave Snowden). The DART diagnostic and the
> "platform" tools are the framing from the source talk.

## What this skill produces

Given a problem/decision/situation, output a structured diagnosis: the system and its pattern → DART
analysis → system type → traps present → the matched action protocol → how to verify you read it
right. Fill the [output template](#output-template).

For deep type cards, signals, and a worked example, read `references/systems-field-guide.md` when a
classification is non-obvious.

## Step 1 — Frame the system

State the system in one line, then move **parts → connections → pattern**. Probe what is *not*
visible with three questions:

- **Hidden parts** — which actors, incentives, constraints, or resources aren't named in the prompt?
- **Connections** — what flows between the parts (information, money, approval, trust,
    order-of-operations)?
- **Pattern** — what outcome does this configuration keep producing, regardless of intent?

## Step 2 — Diagnose with DART

| Step                | Question                                                                               | What it tells you                                                         |
| ------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| **D — Deconstruct** | Break the problem into sub-parts. Are the parts **stable** or **constantly shifting**? | Stable → leans clear/complicated. Shifting → leans complex/chaotic.       |
| **A — Analyze**     | *(decisive)* What is the relationship between **cause and effect**?                    | Maps directly to the system type (Step 3).                                |
| **R — Recognize**   | Have I seen this pattern before — here or in a structurally similar system?            | Borrows a tested response; guards against treating the familiar as novel. |
| **T — Test**        | What is the **smallest reversible test** before committing to a full response?         | De-risks the move. *Exception: chaotic systems leave no time to test.*    |

## Step 3 — Classify the system type

The **Analyze** answer picks the type. Each type has one correct posture; using another type's
protocol is the classic failure.

| System          | Cause ↔ effect                                   | Tell                                                                                          | Protocol                                                                                                                                                  |
| --------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Clear**       | Obvious, directly observable                     | Competent people agree on the steps; best practice exists; repeatable                         | Follow the stable process. Use a **checklist**. Don't improvise. Watch for your "brown M&M" — the small tell that the process was skipped.                |
| **Complicated** | Exists but hidden; needs analysis                | A knowable right answer exists but isn't obvious; solvable by the right pro                   | **Slow down, analyze, hire the *right* specialist** — not just any expert (a cardiac surgeon won't treat lung cancer).                                    |
| **Complex**     | Visible only in **hindsight**                    | Experts disagree; the same action gives different results; people/culture/adaptation involved | Run **small, safe-to-fail experiments**; amplify what works, dampen what doesn't; course-correct. Aim to be **directionally right, not precisely right**. |
| **Chaotic**     | Link is **broken**; info incomplete and shifting | Novel crisis, safety at stake, no time, ground still moving                                   | **Act first — stabilize, create safety**, then understand. Avoid analysis paralysis. Once stable it usually becomes complex/complicated.                  |

**Layered systems are common.** Many real problems stack types — e.g., *building* an AI rollout is
complicated (hire experts, build a roadmap), but getting people to *adopt* it is complex (outcome
emerges over time). Classify each layer and apply its own protocol.

## Step 4 — Check the three traps

These distort the diagnosis or sabotage the action. Flag any that are present:

- **Wrong-type approach** — applying one type's protocol to another (a checklist for a teenager;
    running experiments during an earthquake). State the protocol the prompt is *implicitly*
    assuming and whether it fits.
- **Incentive / cobra effect** — a reward attached to a proxy makes people optimize the proxy and
    abandon the goal (a bounty on dead cobras bred more cobras). Ask: *what does this system
    actually reward, and how will people game it?*
- **Delayed feedback** — consequences surface long after the action (cigarettes: relief in seconds,
    damage in decades). Ask: *what is the lag between action and signal?* Don't read early quiet as
    success.

## Step 5 — Apply the matched protocol

Translate the Step 3 protocol into concrete next actions **for this specific prompt** — name the
checklist, the specialist to call, the experiment to run, or the stabilizing move. Be specific, not
generic.

## Step 6 — Verify direction (get on the platform)

From inside a system you can't tell which way it's carrying you (the moving-train illusion).
Recommend at least one outside vantage point to confirm the read:

- **Mentor** — someone outside the situation with no stake in the story.
- **Data** — what the system actually does vs. what you believe it does.
- **Time** — compare against a week / month / year ago.

## Step 7 — Binary check (when the prompt is "X or Y")

Most either/or framings are limits of the current **system design**, not limits of reality (Ferrari
*vs.* Toyota → Apple ships a luxury product at mass-market scale). If the prompt forces a binary,
test whether a redesign dissolves it — including the story the asker tells about what they're
capable of.

## Output template

```markdown
## System
<one-line name> — the pattern it keeps producing: <pattern>

## Parts → Connections → Pattern
- Parts: ...
- Connections: ...
- Hidden / easily missed: ...

## DART
- Deconstruct: parts are <stable | shifting>
- Analyze: cause→effect is <obvious | discoverable | hindsight-only | broken>
- Recognize: closest prior pattern = ...
- Test: smallest safe test = ... (or "n/a — chaotic")

## System type: <Clear | Complicated | Complex | Chaotic>
<rationale; note any layered sub-systems and their types>

## Traps present
- Incentives / cobra effect: ...
- Delayed feedback (lag): ...
- Wrong-approach risk: ...

## Recommended protocol
<concrete next actions matched to the type(s)>

## Verify direction
- Mentor / Data / Time signal to watch: ...

## Binary check
<is the either/or real, or a design limit?>   (omit if no binary in the prompt)
```

## Notes

- If the prompt is genuinely chaotic, lead with the stabilizing action — do not make the user wait
    through a full analysis.
- Keep the diagnosis proportional to the stakes. A small decision needs a few lines, not the whole
    template.
- If essential parts are unknown, say what you'd need to observe rather than guessing the pattern.
