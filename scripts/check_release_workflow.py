"""Static checks for the release workflow (`.github/workflows/release.yml`).

The release workflow is `workflow_dispatch`-only: no CI run and no CID push ever executes
it, so a structural mistake surfaces only on release day, with publish authority. This
gate verifies three pure-local invariants on every edit of the file (via a prek hook
scoped to `release.yml`) and in CI (via `tests/test_check_release_workflow.py`):

1. **Registry-guard shape** — `prepare-release` is the one deliberately unguarded job;
   every other job must be wrapped in `${{ !cancelled() && !failure() && (...) }}` with
   `inputs.version != ''` as an alternative. Every `inputs.<name>` referenced in a job
   `if` must be a declared `workflow_dispatch` input, and every declared registry input
   must be referenced by at least one job — catching both a typo'd flag and a silently
   dropped one. All shapes are derived structurally from the parsed document; nothing
   (job counts, flag histograms) is hardcoded.
2. **Artifact wiring** — every `download-artifact` `name:`/`pattern:` must resolve to at
   least one `upload-artifact` `name:`, after expanding `${{ matrix.<k> }}` spans against
   the job's `strategy.matrix.include` entries. An upload nobody downloads is not an
   error.
3. **Job graph** — every `needs:` entry must be a declared job id.

Requires PyYAML (declared in the `dev` dependency group).

Usage:
    uv run scripts/check_release_workflow.py [path/to/release.yml]
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"

# `prepare-release` is the one deliberately unguarded job: it pushes the release tag and
# the GitHub Release, so it must run ONLY on full releases (`inputs.version` provided).
# Wrapping it in the `!cancelled() && !failure()` guard shape would make it run on
# registry-only re-trigger dispatches too, pushing a tag for an unreleased version.
UNGUARDED_JOB = "prepare-release"
VERSION_GUARD = "inputs.version != ''"

# Guard wrapper required on every job except UNGUARDED_JOB. The relaxed `success()` ->
# `!cancelled() && !failure()` shape lets publish jobs run when sibling registries were
# skipped, but is only safe in exactly this form (see decisions.md 2026-07-25).
GUARD_RE = re.compile(
    r"\$\{\{ !cancelled\(\) && !failure\(\) && \((?P<inner>.+)\) \}\}"
)
EXPR_RE = re.compile(r"\$\{\{\s*(.*?)\s*\}\}")
INPUT_RE = re.compile(r"inputs\.([A-Za-z0-9_-]+)")


def load_workflow(path: Path) -> dict:
    """Parse a workflow YAML file into a mapping, exiting on unreadable input."""
    if not path.is_file():
        sys.exit(f"error: workflow file not found: {path}")
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        sys.exit(f"error: {path} did not parse to a YAML mapping")
    return data


def declared_inputs(wf: dict) -> set[str]:
    """Return the declared `workflow_dispatch` input names.

    YAML 1.1 parses the top-level `on:` key as the boolean `True`, so the trigger
    block is read from either key.
    """
    on = wf.get("on", wf.get(True)) or {}
    return set((on.get("workflow_dispatch") or {}).get("inputs") or {})


def check_guards(wf: dict) -> list[str]:
    """Check 1a: every job carries the required registry-guard `if` shape."""
    errors: list[str] = []
    jobs = wf.get("jobs") or {}
    if UNGUARDED_JOB not in jobs:
        errors.append(f"guard: job '{UNGUARDED_JOB}' not found in workflow")
    for job_id, job in jobs.items():
        cond = job.get("if")
        if not isinstance(cond, str):
            errors.append(f"guard: job '{job_id}' has no `if` condition")
            continue
        cond = cond.strip()
        if job_id == UNGUARDED_JOB:
            if cond != VERSION_GUARD:
                errors.append(
                    f"guard: job '{UNGUARDED_JOB}' must keep the bare "
                    f"`if: {VERSION_GUARD}`, got: {cond}"
                )
            continue
        match = GUARD_RE.fullmatch(cond)
        if match is None:
            errors.append(
                f"guard: job '{job_id}' `if` does not match the required "
                "'${{ !cancelled() && !failure() && (...) }}' wrapper: " + cond
            )
            continue
        alternatives = [alt.strip() for alt in match.group("inner").split("||")]
        if VERSION_GUARD not in alternatives:
            errors.append(
                f"guard: job '{job_id}' guard lacks `{VERSION_GUARD}` as an "
                f"alternative: {cond}"
            )
    return errors


def check_input_wiring(wf: dict) -> list[str]:
    """Check 1b: job `if` input references and declared workflow inputs agree."""
    declared = declared_inputs(wf)
    if not declared:
        return ["inputs: no `workflow_dispatch` inputs declared"]
    referenced: set[str] = set()
    for job in (wf.get("jobs") or {}).values():
        cond = job.get("if")
        if isinstance(cond, str):
            referenced.update(INPUT_RE.findall(cond))
    errors = [
        f"inputs: job `if` references undeclared input 'inputs.{name}'"
        for name in sorted(referenced - declared)
    ]
    errors.extend(
        f"inputs: declared registry input '{name}' is not referenced by any job `if`"
        for name in sorted(declared - {"version"} - referenced)
    )
    return errors


def matrix_include(job: dict) -> list[dict]:
    """Return the job's `strategy.matrix.include` entries, or an empty list."""
    matrix = (job.get("strategy") or {}).get("matrix") or {}
    return [entry for entry in matrix.get("include") or [] if isinstance(entry, dict)]


def substitute_entry(value: str, entry: dict) -> str:
    """Replace each `${{ … }}` span in `value` using one matrix include entry.

    A `matrix.<k>` span whose key exists in the entry becomes that literal value;
    any other span becomes a `*` wildcard.
    """
    result = value
    for match in reversed(list(EXPR_RE.finditer(value))):
        expr = match.group(1)
        replacement = "*"
        if expr.startswith("matrix."):
            key = expr[len("matrix.") :]
            if key in entry:
                replacement = str(entry[key])
        result = result[: match.start()] + replacement + result[match.end() :]
    return result


def expand_reference(value: str, job: dict) -> list[str]:
    """Expand an artifact name/pattern against the job's matrix include entries.

    With include entries present, each entry yields one expansion (deduplicated);
    without them, every `${{ … }}` span collapses to a `*` wildcard.
    """
    include = matrix_include(job)
    if include and EXPR_RE.search(value):
        return sorted({substitute_entry(value, entry) for entry in include})
    return [EXPR_RE.sub("*", value)]


def collect_artifacts(wf: dict) -> tuple[list[str], list[tuple[str, str]]]:
    """Collect expanded upload names and `(job_id, reference)` download references."""
    uploads: list[str] = []
    downloads: list[tuple[str, str]] = []
    for job_id, job in (wf.get("jobs") or {}).items():
        for step in job.get("steps") or []:
            uses = str(step.get("uses") or "")
            with_block = step.get("with") or {}
            if uses.startswith("actions/upload-artifact"):
                name = with_block.get("name")
                if name is not None:
                    uploads.extend(expand_reference(str(name), job))
            elif uses.startswith("actions/download-artifact"):
                ref = with_block.get("name", with_block.get("pattern"))
                if ref is not None:
                    downloads.extend(
                        (job_id, expanded)
                        for expanded in expand_reference(str(ref), job)
                    )
    return uploads, downloads


def glob_to_re(value: str) -> re.Pattern[str]:
    """Compile an artifact glob (literal text with `*` wildcards) to a regex."""
    return re.compile(".*".join(re.escape(part) for part in value.split("*")))


def references_match(download: str, upload: str) -> bool:
    """Return True if a download reference can resolve the given upload name.

    Both sides may carry `*` wildcards — download references natively (`pattern:
    jni-*`), upload names from unexpandable `${{ … }}` spans. A reference resolves
    when either glob, with the other side's wildcards collapsed, fully matches the
    other: this accepts every pairing whose literal parts line up (e.g. `wheels-*`
    vs `wheels-ubuntu-latest-x86_64`, `gem-x86_64-linux` vs `gem-*`) and rejects a
    pairing only when no overlap is possible.
    """
    return bool(
        glob_to_re(upload).fullmatch(download.replace("*", ""))
        or glob_to_re(download).fullmatch(upload.replace("*", ""))
    )


def check_artifact_wiring(wf: dict) -> list[str]:
    """Check 2: every download-artifact reference resolves to an upload name."""
    uploads, downloads = collect_artifacts(wf)
    return [
        f"artifact: download '{ref}' in job '{job_id}' matches no upload-artifact name"
        for job_id, ref in downloads
        if not any(references_match(ref, upload) for upload in uploads)
    ]


def check_needs(wf: dict) -> list[str]:
    """Check 3: every `needs:` entry names a declared job id."""
    errors: list[str] = []
    jobs = wf.get("jobs") or {}
    for job_id, job in jobs.items():
        needs = job.get("needs") or []
        if isinstance(needs, str):
            needs = [needs]
        errors.extend(
            f"needs: job '{job_id}' needs undeclared job '{dep}'"
            for dep in needs
            if dep not in jobs
        )
    return errors


def run_checks(wf: dict) -> list[str]:
    """Run all static checks against a parsed workflow; return all error strings."""
    return (
        check_guards(wf)
        + check_input_wiring(wf)
        + check_artifact_wiring(wf)
        + check_needs(wf)
    )


def main() -> None:
    """Entry point: print one error per line and exit 1 if any check fails."""
    parser = argparse.ArgumentParser(
        description="Static checks for the release workflow (guards, artifacts, needs)."
    )
    parser.add_argument(
        "workflow",
        nargs="?",
        type=Path,
        default=DEFAULT_WORKFLOW,
        help=f"workflow file to check (default: {DEFAULT_WORKFLOW})",
    )
    args = parser.parse_args()
    errors = run_checks(load_workflow(args.workflow))
    for error in errors:
        print(error)
    if errors:
        sys.exit(1)
    print(f"OK: {args.workflow} passed release-workflow static checks.")


if __name__ == "__main__":
    main()
