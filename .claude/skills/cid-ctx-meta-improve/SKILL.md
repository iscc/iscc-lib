---
name: cid-ctx-meta-improve
description: >-
  Context pack for the CID meta-improve role. Invoked by the CID runner (tools/cid.py) as a
  prompt prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(tail *)
  - Bash(git *)
---

# Context pack — meta-improve

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool** — re-reading returns the same bytes you already have.

<handoff-md>
!`cat .claude/context/handoff.md 2>/dev/null || echo "(missing)"`
</handoff-md>

<issues-md>
!`cat .claude/context/issues.md 2>/dev/null || echo "(no issues)"`
</issues-md>

<learnings-md>
!`cat .claude/context/learnings.md 2>/dev/null || echo "(no learnings)"`
</learnings-md>

<prior-auto-changes>
!`tail -n 40 .claude/context/meta-log.jsonl 2>/dev/null || echo "(no meta-log yet)"`
</prior-auto-changes>

<git-log>
!`git log --oneline -25 2>/dev/null || echo "(no commits yet)"`
</git-log>

## Task

$ARGUMENTS
