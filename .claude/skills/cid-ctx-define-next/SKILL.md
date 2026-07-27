---
name: cid-ctx-define-next
description: >-
  Context pack for the CID define-next role. Invoked by the CID runner (tools/cid.py) as a
  prompt prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(git *)
---

# Context pack — define-next

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool** — re-reading returns the same bytes you already have.

You have the target and the state together here, which is the whole input to your gap analysis.
There is no excuse for scoping from the handoff suggestion alone.

<target-md>
!`cat .claude/context/target.md 2>/dev/null || echo "(missing)"`
</target-md>

<state-md>
!`cat .claude/context/state.md 2>/dev/null || echo "(missing)"`
</state-md>

<handoff-md>
!`cat .claude/context/handoff.md 2>/dev/null || echo "(missing)"`
</handoff-md>

<issues-md>
!`cat .claude/context/issues.md 2>/dev/null || echo "(no issues)"`
</issues-md>

<learnings-md>
!`cat .claude/context/learnings.md 2>/dev/null || echo "(no learnings)"`
</learnings-md>

<decision-titles>
!`grep -h '^## 2' .claude/context/decisions-archive.md .claude/context/decisions.md 2>/dev/null || echo "(no decisions)"`
</decision-titles>

<git-log>
!`git log --oneline -10 2>/dev/null || echo "(no commits yet)"`
</git-log>

## Task

$ARGUMENTS
