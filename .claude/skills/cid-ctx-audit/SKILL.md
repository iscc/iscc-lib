---
name: cid-ctx-audit
description: >-
  Context pack for the CID audit role. Invoked by the CID runner (tools/cid.py) as a prompt
  prefix, never by hand.
disable-model-invocation: true
allowed-tools:
  - Bash(cat *)
  - Bash(tail *)
  - Bash(git *)
---

# Context pack — audit

Everything your protocol reads unconditionally is inlined below, current as of this moment. **Do not
read these files again with the Read tool**, and do not re-run `tools/metrics.py` — the runner
already appended the latest snapshot and the trend is here.

<metrics-trend>
!`tail -n 6 .claude/context/metrics.jsonl 2>/dev/null || echo "(no metrics yet)"`
</metrics-trend>

<issues-md>
!`cat .claude/context/issues.md 2>/dev/null || echo "(no issues)"`
</issues-md>

<learnings-md>
!`cat .claude/context/learnings.md 2>/dev/null || echo "(no learnings)"`
</learnings-md>

<recent-audits>
!`git log --oneline -300 2>/dev/null | grep "cid(audit)" | head -5 || echo "(no prior audits)"`
</recent-audits>

<git-log>
!`git log --oneline -15 2>/dev/null || echo "(no commits yet)"`
</git-log>

## Task

$ARGUMENTS
