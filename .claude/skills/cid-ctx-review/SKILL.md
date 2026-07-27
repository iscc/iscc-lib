---
name: cid-ctx-review
description: >-
  Context pack for the CID review role. Invoked by the CID runner (tools/cid.py) as a prompt
  prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(grep *)
  - Bash(git *)
---

# Context pack — review

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool** — re-reading returns the same bytes you already have.

You have ample context remaining. Do not stop, summarize, or suggest a new session on account of
context limits: run the protocol through to the end.

Decision *titles* are listed rather than the full entries, covering `decisions.md` **and**
`decisions-archive.md` — rotation moves an entry's text out of budget, never its authority. Scan
them before you scope, judge, or file anything: a title that touches your area means that trade-off
is already settled and must not be re-litigated. Read the full entry only when you need its
reasoning; if the title is not in `decisions.md`, it is in the archive.

The diffstat below is orientation only — your protocol still runs the full `git diff HEAD~1..HEAD`
and reads the modified files.

<next-md>
!`cat .claude/context/next.md 2>/dev/null || echo "(missing)"`
</next-md>

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

<advance-diffstat>
!`git diff HEAD~1..HEAD --stat 2>/dev/null || echo "(no advance commit)"`
</advance-diffstat>

## Task

$ARGUMENTS
