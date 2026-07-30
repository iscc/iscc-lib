---
name: cid-ctx-update-state
description: >-
  Context pack for the CID update-state role. Invoked by the CID runner (tools/cid.py) as a
  prompt prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(grep *)
  - Bash(git *)
---

# Context pack — update-state

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool** — re-reading returns the same bytes you already have.

Open-issue *titles* are listed rather than the full `issues.md`, because your only use for them is
the DONE rule (any open issue blocks DONE). Read the full file only if a title is ambiguous.

<target-md>
!`cat .claude/context/target.md 2>/dev/null || echo "(missing)"`
</target-md>

<current-state-md>
!`cat .claude/context/state.md 2>/dev/null || echo "(missing — do a full review)"`
</current-state-md>

<handoff-md>
!`cat .claude/context/handoff.md 2>/dev/null || echo "(missing)"`
</handoff-md>

<open-issue-titles>
!`grep '^## ' .claude/context/issues.md 2>/dev/null || echo "(no open issues)"`
</open-issue-titles>

<git-log>
!`git log --oneline -20 2>/dev/null || echo "(no commits yet)"`
</git-log>

## Task

$ARGUMENTS
