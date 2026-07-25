---
name: feedback-issues-ledger
description: Advance never edits issues.md even when next.md's Implementation Notes instruct a ledger append — put the ledger text in the handoff
metadata:
  type: feedback
---

Advance must NOT modify `.claude/context/issues.md`, even when next.md's Implementation Notes say
"append the result to the issue's progress paragraph" (happened iter 135, ruff slice C).

**Why:** The advance protocol hard-forbids writing issues.md, and the review agent owns issue
resolution/progress. The slice-B review (iter 134) explicitly counted "issues.md untouched by
advance" as *positive* scope discipline — so the protocol rule wins over next.md's instruction.

**How to apply:** When next.md contains a "Ledger" step targeting issues.md, write the exact ledger
paragraph into the handoff's **Notes** section for the review agent to append, and say why it was
not written directly. Do not treat this as a blocker.
