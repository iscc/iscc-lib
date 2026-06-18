# Systems Field Guide

Reference for classifying a system when the type is non-obvious. Read this when the SKILL.md
dispatch table doesn't resolve the call cleanly. The four-type model is the **Cynefin framework**
(Dave Snowden); examples and framing below follow the source talk.

## The four types, in depth

Cause-and-effect distance is the spine of the whole model. As it grows — from *obvious*, to
*knowable*, to *only-in-hindsight*, to *broken* — the right posture flips from "execute" to
"investigate" to "probe" to "stabilize."

### Clear — execute a known process

- **Signature:** cause and effect are directly observable; following the steps predicts the outcome.
- **Tells:** competent people agree on the method; documented best practice exists; the task is
    repeatable.
- **Do:** follow the precise process; use a **checklist** (pilots and surgeons use them not as an
    insult to expertise but as respect for being human). Precision beats cleverness.
- **The "brown M&M" tell:** Van Halen buried a clause in a 400-page contract requiring a bowl of
    M&M's backstage with the brown ones removed. Brown M&M's in the bowl meant the venue hadn't read
    the contract carefully — so the dangerous stage/electrical details were probably skipped too.
    Find the cheap signal that proves the process was actually followed.
- **Examples:** following a recipe; surgical scrub-in protocol (hands, wrists, forearms, 2 minutes
    minimum); standard deployment runbooks.
- **Failure mode:** over-engineering or "improving" a process that just needed to be followed.

### Complicated — investigate, then bring the right expert

- **Signature:** a correct answer exists but is hidden; analysis or expertise uncovers it.
- **Tells:** experts agree the problem is solvable; the symptom is clear but the cause is not; good
    practice (plural) exists rather than one best practice.
- **Do:** slow down and analyze; find the **right specialist**, not just any expert — a cardiac
    surgeon is useless for lung cancer.
- **Examples:** chest pain in an ER (clear symptom, ~20 possible causes, one may kill within the
    hour); choosing a mortgage structure; financial modeling; tax planning; enterprise architecture;
    aircraft repair; acquisition due diligence.
- **Failure mode:** the generalist trap — hiring expertise that's adjacent but wrong, or skipping
    analysis because the symptom looked simple.

### Complex — probe with safe-to-fail experiments

- **Signature:** cause and effect are visible only in **hindsight**; hiring experts doesn't reliably
    help.
- **Tells:** experts disagree; the same input produces different outcomes on different days; humans,
    culture, and adaptation are in the loop; the answer must *emerge*.
- **Do:** run many **small experiments**, amplify what works and dampen what doesn't, course-correct
    continuously. Stay **directionally right, not precisely right**.
- **Examples:**
    - *M&A culture integration* — two companies look complementary on paper; within 60 days the
        cultures (formal/hierarchical/risk-averse vs. informal/fast/"cowboy") prove incompatible;
        leaders leave, products shut down. The greatest opportunity becomes the biggest distraction.
    - *Raising a teenager* — no SOP, no checklist, no hireable expert; body, brain, and priorities are
        all shifting; what worked last week fails this week.
    - *Change management for AI adoption* — see the graduation note below.
- **Failure mode:** demanding a guaranteed plan, or buying a roadmap and assuming execution equals
    adoption.

### Chaotic — act first, stabilize, understand later

- **Signature:** the link between cause and effect is broken; information is incomplete and
    constantly changing.
- **Tells:** novel crisis, safety on the line, no time, the ground is still moving.
- **Do:** act immediately to **stabilize and create safety**; restore enough order that the system
    becomes merely complex, *then* investigate.
- **Example:** the 1982 Tylenol cyanide poisonings — seven dead in Chicago, no one knew which
    bottles were safe or whether more deaths were coming. Johnson & Johnson didn't analyze or
    convene experts; they warned the public and pulled 31 million bottles. Stabilize first, ask why
    later.
- **Failure mode:** **analysis paralysis** — waiting for the full picture. Chaos won't teach you;
    like an earthquake, there's no pattern to respond to in the moment.

## Layered systems — the "graduation" effect

Real problems stack types, and a problem can graduate from one to another mid-stream:

> Rolling out AI across a business: hiring consultants to pick tools, build a roadmap, and execute
> is **complicated** — doable with the right experts. Then you ask people to change how they
> actually work, and you've graduated to **complex** — whether adoption succeeds emerges only over
> time.

Always ask whether the prompt is one system or several layered ones, and whether solving the
complicated layer just exposes a complex layer underneath. Classify and treat each layer on its own
terms.

## Why systems confuse us (the three confusions)

1. **Not knowing which type you're in** — so you bring the wrong protocol to the right system. The
    single most common error; it's why classification comes first.
2. **The incentive problem (cobra effect)** — 1900s Delhi paid a bounty per dead cobra; people bred
    cobras to cash in, and the population *rose*. Attach a reward to a proxy and people optimize
    the proxy while abandoning the goal. Always ask what a system *actually* rewards.
3. **Delayed feedback loops** — you act now but the consequence is invisible for a long time.
    Cigarettes gave relief in seconds and disease in decades, so the feedback never felt urgent.
    When the lag is long, early quiet is not evidence of success.

## Getting on the platform (seeing your own system)

Every system you live inside is quietly training you in some direction — the biggest feedback loop
most people never notice. From inside a train you can't tell whether your train or the one beside it
is moving; the person on the platform has no such confusion. Three ways to reach the platform:

- **Mentors** — someone outside your world with no stake in your story.
- **Data** — numbers don't care about your narrative; they show what the system *does* vs. what you
    *believe* it does.
- **Time** — the truth-teller; compare yourself to a week, month, or year ago.

## False binaries

Most "either/or" choices are limits of **system design**, not reality. Conventional wisdom says
build a Ferrari (high-margin luxury) *or* a Toyota (high-volume everyday). Apple refused the binary
and ships ~350 iPhones a minute — a luxury product at mass-market scale — because they spent two
decades building a system that made both possible. The hardest system to redesign is the story in
your own head about what you can become; like any system, it can be re-imagined.

## Worked example

**Prompt:** "Our two-person startup just signed a huge enterprise client. Should we hire fast to
deliver, or stay lean and risk burning out?"

- **System:** a young company's delivery capacity — the pattern it produces is "commitments outrun
    the people who fulfill them."
- **Parts → connections → pattern:** founders, the new client, future clients, cash runway, hiring
    pipeline; connected by commitments and cash flow; the recurring pattern is over-promising
    against thin capacity.
- **DART:** *Deconstruct* — parts are shifting (team size, client scope, morale). *Analyze* —
    cause→effect is hindsight-only: whether a hire works out shows up months later. *Recognize* —
    classic early-stage scaling pattern. *Test* — bring on one contractor for this contract before
    committing to permanent headcount.
- **Type:** **Complex** (hiring/culture/burnout emerge over time), with a **Complicated** sub-layer
    (structuring the client contract — solvable with the right lawyer).
- **Traps:** *Incentive* — paying recruiters per hire optimizes for speed over fit. *Delayed
    feedback* — a bad culture hire isn't visible for months. *Wrong-approach* — treating this as
    Clear ("just post a job and follow the hiring checklist") ignores the emergent risk.
- **Protocol:** run a safe-to-fail experiment (one contractor or a single senior hire) and watch
    leading signals before scaling; lawyer-up the contract layer.
- **Verify direction:** a founder-mentor who has scaled past the first big client (mentor);
    utilization and cash-runway numbers (data); revisit the call in 30/60/90 days (time).
- **Binary check:** "hire fast vs. stay lean" is a false binary — staged hiring plus contractors
    dissolves it.
