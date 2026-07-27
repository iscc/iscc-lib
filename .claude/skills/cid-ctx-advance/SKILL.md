---
name: cid-ctx-advance
description: >-
  Context pack for the CID advance role. Invoked by the CID runner (tools/cid.py) as a prompt
  prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(git *)
---

# Context pack — advance

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool** — re-reading returns the same bytes you already have.

The work package is here in full, so start implementing. Beyond this pack, read only what next.md's
`## Reference` section names.

<next-md>
!`cat .claude/context/next.md 2>/dev/null || echo "(missing)"`
</next-md>

<handoff-md>
!`cat .claude/context/handoff.md 2>/dev/null || echo "(missing)"`
</handoff-md>

<learnings-md>
!`cat .claude/context/learnings.md 2>/dev/null || echo "(no learnings)"`
</learnings-md>

<git-status>
!`git status --short 2>/dev/null || echo "(not a git repo)"`
</git-status>

## Task

$ARGUMENTS
