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

A fourth, opt-in check needs the network and therefore runs only in CI (never in prek
or the pytest suite, which stay network-free):

4. **Action-input compatibility** (`--check-action-inputs`) — every `with:` key on a
   repository action must be a declared `inputs` key of that ref's published
   `action.yml`, and every `steps.<id>.outputs.<x>` read from an action step must be a
   declared `outputs` key. A 404 on both `action.yml` and `action.yaml` is an error
   (the ref or sub-path is wrong); any transport failure (offline, timeout, rate
   limit) degrades to a `warning: skipped …` line on stderr, never a red gate.

Requires PyYAML (declared in the `dev` dependency group).

Usage:
    uv run scripts/check_release_workflow.py [--check-action-inputs] [path/to/release.yml]
"""

from __future__ import annotations

import argparse
import functools
import re
import sys
import urllib.error
import urllib.request
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
    other (e.g. `wheels-*` vs `wheels-ubuntu-latest-x86_64`, `gem-x86_64-linux` vs
    `gem-*`).

    This is a deliberate approximation of glob intersection, not the real thing: it
    errs strict, so a pairing where *both* sides wildcard in different positions
    (`gem-*` vs `*-linux`) is reported as an error even though it overlaps. No such
    pairing exists at HEAD; if one is ever added, replace this with a proper
    intersection rather than loosening the check.
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


RAW_HOST = "https://raw.githubusercontent.com"
FETCH_TIMEOUT = 20  # seconds
STEP_OUTPUT_RE = re.compile(r"steps\.([A-Za-z0-9_-]+)\.outputs\.([A-Za-z0-9_-]+)")


class ActionNotFoundError(Exception):
    """Raised when both `action.yml` and `action.yaml` return HTTP 404 for a ref."""


def is_repo_action(uses: str) -> bool:
    """Return True when a step `uses:` ref points to a fetchable repository action.

    Docker refs (`docker://…`) and local actions (`./…`) have no published
    `action.yml` on raw GitHub and are skipped without error.
    """
    if uses.startswith(("docker://", "./")):
        return False
    return "@" in uses and uses.partition("@")[0].count("/") >= 1


def action_yml_urls(ref: str) -> list[str]:
    """Return the candidate raw-GitHub URLs for a repo action ref's metadata file.

    `owner/repo[/subpath]@gitref` maps to
    `https://raw.githubusercontent.com/owner/repo/gitref[/subpath]/action.yml`
    (then `action.yaml`). The ref is split on the first `@`; slashes inside the
    git ref (e.g. `@release/v1`) are kept verbatim, never URL-encoded.
    """
    path, _, git_ref = ref.partition("@")
    segments = path.split("/")
    directory = "/".join([RAW_HOST, segments[0], segments[1], git_ref, *segments[2:]])
    return [f"{directory}/action.yml", f"{directory}/action.yaml"]


def fetch_action(ref: str) -> dict | None:
    """Fetch and parse the published action metadata for a repo action ref.

    Returns the parsed `action.yml` mapping, or `None` (after printing a
    `warning: skipped …` line to stderr) on any transport failure — a degraded
    network must never turn the gate red. Raises `ActionNotFoundError` when
    both candidate URLs return 404: the ref or sub-path is wrong, which is a
    real workflow defect.
    """
    for url in action_yml_urls(ref):
        try:
            # S310 audits urlopen for non-literal URLs; this one is built by
            # action_yml_urls from the RAW_HOST https literal, so the scheme
            # cannot be attacker-controlled.
            with urllib.request.urlopen(url, timeout=FETCH_TIMEOUT) as resp:  # noqa: S310
                data = yaml.safe_load(resp.read().decode("utf-8"))
                return data if isinstance(data, dict) else {}
        except urllib.error.HTTPError as exc:
            if exc.code == 404:
                continue
            print(f"warning: skipped {ref}: HTTP {exc.code} on {url}", file=sys.stderr)
            return None
        except OSError as exc:  # URLError, socket timeout, connection refused
            print(f"warning: skipped {ref}: {exc}", file=sys.stderr)
            return None
    raise ActionNotFoundError(f"no action.yml or action.yaml found for '{ref}'")


def cached_fetch(ref: str, fetch, cache: dict, errors: list[str]) -> dict | None:
    """Fetch action metadata once per distinct ref, recording a 404 as an error."""
    if ref not in cache:
        try:
            cache[ref] = fetch(ref)
        except ActionNotFoundError as exc:
            cache[ref] = None
            errors.append(f"action: {exc}")
    return cache[ref]


def iter_strings(node):
    """Yield every string value found in a nested YAML structure."""
    if isinstance(node, str):
        yield node
    elif isinstance(node, dict):
        for value in node.values():
            yield from iter_strings(value)
    elif isinstance(node, list):
        for item in node:
            yield from iter_strings(item)


def check_with_keys(job_id: str, job: dict, get_meta) -> list[str]:
    """Check 4a: every `with:` key on a repo action step is a declared input."""
    errors: list[str] = []
    for step in job.get("steps") or []:
        uses = str(step.get("uses") or "")
        if not is_repo_action(uses):
            continue
        meta = get_meta(uses)
        if meta is None:  # skipped fetch or already-reported 404
            continue
        declared = set(meta.get("inputs") or {})
        errors.extend(
            f"action: job '{job_id}' passes undeclared input '{key}' to '{uses}'"
            for key in step.get("with") or {}
            if key not in declared
        )
    return errors


def check_step_outputs(job_id: str, job: dict, get_meta) -> list[str]:
    """Check 4b: every `steps.<id>.outputs.<x>` read from an action step resolves.

    References to local `run:` steps (no `uses:`) and to ids that are not steps
    of the same job are ignored — there is no published metadata to check.
    """
    errors: list[str] = []
    steps = {s.get("id"): s for s in job.get("steps") or [] if s.get("id")}
    refs = {m for text in iter_strings(job) for m in STEP_OUTPUT_RE.findall(text)}
    for step_id, output in sorted(refs):
        step = steps.get(step_id)
        if step is None:
            continue
        uses = str(step.get("uses") or "")
        if not is_repo_action(uses):
            continue
        meta = get_meta(uses)
        if meta is None:
            continue
        if output not in set(meta.get("outputs") or {}):
            errors.append(
                f"action: job '{job_id}' reads undeclared output "
                f"'steps.{step_id}.outputs.{output}' from '{uses}'"
            )
    return errors


def check_action_compat(wf: dict, fetch) -> list[str]:
    """Check 4: `with:` keys and step-output reads match published action metadata.

    `fetch` maps an action ref to its parsed `action.yml` mapping, returns
    `None` for a skipped (network-unavailable) fetch, or raises
    `ActionNotFoundError` on a real 404. It is injected so tests supply an
    in-memory fake; production passes `fetch_action`. Fetches are cached per
    distinct ref, so a run costs one request per unique action.
    """
    errors: list[str] = []
    get_meta = functools.partial(cached_fetch, fetch=fetch, cache={}, errors=errors)
    for job_id, job in (wf.get("jobs") or {}).items():
        errors.extend(check_with_keys(job_id, job, get_meta))
        errors.extend(check_step_outputs(job_id, job, get_meta))
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
    parser.add_argument(
        "--check-action-inputs",
        action="store_true",
        help="also validate `with:` keys and step-output reads against each "
        "action's published action.yml (needs network; transport failures "
        "degrade to a stderr warning, not an error)",
    )
    args = parser.parse_args()
    wf = load_workflow(args.workflow)
    errors = run_checks(wf)
    if args.check_action_inputs:
        errors.extend(check_action_compat(wf, fetch_action))
    for error in errors:
        print(error)
    if errors:
        sys.exit(1)
    print(f"OK: {args.workflow} passed release-workflow static checks.")


if __name__ == "__main__":
    main()
