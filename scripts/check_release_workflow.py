"""Static checks for the release workflow (`.github/workflows/release.yml`).

The release workflow is `workflow_dispatch`-only: no CI run and no CID push ever executes
it, so a structural mistake surfaces only on release day, with publish authority. This
gate verifies four pure-local invariants on every edit of the file (via a prek hook
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
4. **Gem smoke-test series** — the `test-gem` matrix `ruby` list must equal the
   `ruby-versions` list that the cross-gem step of `build-gem` builds, and the matrix
   must not drop a listed series through `exclude`. One `run` step must consist of
   exactly three lines: a check that the running Ruby is the leg's matrix series, the
   install of the downloaded gem and the self-test command. No step may carry `if` or
   `continue-on-error`, and the job may not carry
   `continue-on-error`. `publish-rubygems` must need `test-gem` and carry the same `if`,
   so no dispatch publishes gems without testing them. A precompiled gem holds one
   binary per Ruby series, so a series that is built and not loaded would be published
   untested.

A fifth, opt-in check needs the network and therefore runs only in CI (never in prek
or the pytest suite, which stay network-free):

5. **Action-input compatibility** (`--check-action-inputs`) — every `with:` key on a
   repository action must be a declared `inputs` key of that ref's published
   `action.yml` (docker actions additionally accept the GitHub-native `args` and
   `entrypoint` overrides), every declared input with `required: true` and no
   `default:` must be passed by the step, and every `steps.<id>.outputs.<x>` read
   from an action step must be a declared `outputs` key. A 404 on both `action.yml`
   and `action.yaml` is an error (the ref or sub-path is wrong); any transport or
   parse failure (offline, timeout, rate limit, truncated read, captive-portal
   HTML) degrades to a `warning: skipped …` line on stderr, never a red gate. The
   check prints an `action-inputs: resolved <R> of <T> action refs` summary line
   so an all-skipped run is visible in the job log rather than passing silently.

Requires PyYAML (declared in the `dev` dependency group).

Usage:
    uv run scripts/check_release_workflow.py [--check-action-inputs] [path/to/release.yml]
"""

from __future__ import annotations

import argparse
import functools
import http.client
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


def job_needs(job: dict) -> list[str]:
    """Return the `needs:` entries of a job as a list."""
    needs = job.get("needs") or []
    return [needs] if isinstance(needs, str) else list(needs)


def check_needs(wf: dict) -> list[str]:
    """Check 3: every `needs:` entry names a declared job id."""
    jobs = wf.get("jobs") or {}
    return [
        f"needs: job '{job_id}' needs undeclared job '{dep}'"
        for job_id, job in jobs.items()
        for dep in job_needs(job)
        if dep not in jobs
    ]


GEM_BUILD_JOB = "build-gem"
GEM_TEST_JOB = "test-gem"
GEM_PUBLISH_JOB = "publish-rubygems"
CROSS_GEM_ACTION = "oxidize-rb/actions/cross-gem@"
# Step keys that let a matrix leg succeed without the step running or counting.
GEM_TEST_STEP_BYPASSES = ("if", "continue-on-error")
# The script that checks the running Ruby against the leg's series (which Ruby setup
# steps ran before cannot be read from the YAML), installs the gem this run built and
# runs the conformance vectors through its native extension. `ruby -e` exits 0 whatever
# its last expression returns, so each `raise` is what fails the step. Any further line
# could skip the test on a series or install another gem, so the script is compared as a
# whole.
GEM_RUBY_CHECK_COMMAND = (
    "ruby -e \"raise 'Ruby ' + RUBY_VERSION + ' is not ${{ matrix.ruby }}' "
    "unless RUBY_VERSION.start_with?('${{ matrix.ruby }}.')\""
)
GEM_INSTALL_COMMAND = "gem install gems/*.gem"
GEM_SELFTEST_COMMAND = (
    "ruby -e \"require 'iscc_lib'; "
    "raise 'Conformance failed' unless IsccLib.conformance_selftest\""
)
GEM_TEST_SCRIPT = [GEM_RUBY_CHECK_COMMAND, GEM_INSTALL_COMMAND, GEM_SELFTEST_COMMAND]


def step_with(job: dict, action: str) -> dict:
    """Return the `with:` mapping of the first step of a job that uses `action`."""
    for step in job.get("steps") or []:
        if str(step.get("uses", "")).startswith(action):
            return step.get("with") or {}
    return {}


def cross_gem_series(job: dict) -> list[str]:
    """Return the Ruby series that the cross-gem step of a job builds."""
    versions = str(step_with(job, CROSS_GEM_ACTION).get("ruby-versions", ""))
    return [series.strip() for series in versions.split(",") if series.strip()]


def run_lines(step: dict) -> list[str]:
    """Return the stripped, non-blank lines of the `run` script of a step."""
    lines = str(step.get("run", "")).splitlines()
    return [line.strip() for line in lines if line.strip()]


def gem_test_bypasses(job: dict) -> list[str]:
    """Return what lets a leg of the gem test job succeed without loading the gem."""
    steps = job.get("steps") or []
    found = [
        f"step {position} must not carry `{key}`"
        for position, step in enumerate(steps, start=1)
        for key in GEM_TEST_STEP_BYPASSES
        if key in step
    ]
    if "continue-on-error" in job:
        found.append("the job must not carry `continue-on-error`")
    if not any(run_lines(step) == GEM_TEST_SCRIPT for step in steps):
        found.append(
            "a `run` step must consist of exactly the lines "
            + ", ".join(f"`{line}`" for line in GEM_TEST_SCRIPT)
        )
    return found


def gem_publish_gaps(jobs: dict) -> list[str]:
    """Return what lets the gem publish job run without a gem test that passed."""
    test_job = jobs.get(GEM_TEST_JOB) or {}
    publish_job = jobs.get(GEM_PUBLISH_JOB) or {}
    found = []
    if GEM_TEST_JOB not in job_needs(publish_job):
        found.append(
            f"gem-series: job '{GEM_PUBLISH_JOB}' must need job '{GEM_TEST_JOB}': "
            "the publish has to wait for the gem test"
        )
    if test_job.get("if") != publish_job.get("if"):
        found.append(
            f"gem-series: jobs '{GEM_TEST_JOB}' and '{GEM_PUBLISH_JOB}' must carry the "
            "same `if`: a dispatch that publishes the gems has to test them"
        )
    return found


def check_gem_series(wf: dict) -> list[str]:
    """Check 4: every Ruby series `build-gem` builds is loaded before it is published."""
    jobs = wf.get("jobs") or {}
    built = cross_gem_series(jobs.get(GEM_BUILD_JOB) or {})
    if not built:
        return [
            f"gem-series: job '{GEM_BUILD_JOB}' has no cross-gem `ruby-versions` list"
        ]
    test_job = jobs.get(GEM_TEST_JOB) or {}
    matrix = (test_job.get("strategy") or {}).get("matrix") or {}
    tested = [str(series) for series in matrix.get("ruby") or []]
    errors: list[str] = []
    if sorted(tested) != sorted(built):
        errors.append(
            f"gem-series: job '{GEM_TEST_JOB}' matrix `ruby` {tested} differs from "
            f"the cross-gem `ruby-versions` {built} of job '{GEM_BUILD_JOB}'"
        )
    if "exclude" in matrix:
        errors.append(
            f"gem-series: job '{GEM_TEST_JOB}' matrix must not use `exclude`: "
            "every listed Ruby series has to run"
        )
    errors.extend(
        f"gem-series: job '{GEM_TEST_JOB}': {problem}: every Ruby series has to load "
        "the gem, and a failure has to fail the job"
        for problem in gem_test_bypasses(test_job)
    )
    return errors + gem_publish_gaps(jobs)


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
    `warning: skipped …` line to stderr) on any transport or parse failure — a
    degraded network must never turn the gate red. Raises `ActionNotFoundError`
    when both candidate URLs return 404: the ref or sub-path is wrong, which is
    a real workflow defect.
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
            # Must stay first: HTTPError is an OSError subclass.
            if exc.code == 404:
                continue
            print(f"warning: skipped {ref}: HTTP {exc.code} on {url}", file=sys.stderr)
            return None
        except (OSError, http.client.HTTPException, yaml.YAMLError) as exc:
            # URLError/timeouts, truncated reads (IncompleteRead), and bodies
            # that are not valid YAML (captive portal or proxy HTML).
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


# GitHub-native `with:` overrides accepted by docker actions without declaration.
DOCKER_NATIVE_KEYS = {"args", "entrypoint"}


def declared_input_keys(meta: dict) -> set[str]:
    """Return an action's declared input names, plus docker-native overrides.

    Docker actions (`runs.using: docker`) accept the GitHub-native `args` and
    `entrypoint` `with:` overrides, which are never declared as `inputs`.
    """
    declared = set(meta.get("inputs") or {})
    if (meta.get("runs") or {}).get("using") == "docker":
        declared |= DOCKER_NATIVE_KEYS
    return declared


def missing_required_inputs(meta: dict, with_keys: set) -> list[str]:
    """Return declared required inputs without a default that `with_keys` omits.

    A `required` value of the string "true" counts as required — action
    authors quote it. Non-string input names (YAML 1.1 booleans) are skipped.
    """
    missing = []
    for name, spec in (meta.get("inputs") or {}).items():
        spec = spec if isinstance(spec, dict) else {}
        if (
            isinstance(name, str)
            and spec.get("required") in (True, "true")
            and "default" not in spec
            and name not in with_keys
        ):
            missing.append(name)
    return sorted(missing)


def check_with_keys(job_id: str, job: dict, get_meta) -> list[str]:
    """Check 5a: `with:` keys and required inputs of each repo action step agree.

    Every string `with:` key must be a declared input (or a docker-native
    override), and every declared `required: true` input without a `default:`
    must be passed. Non-string `with:` keys are skipped: PyYAML's YAML 1.1
    booleans turn a key literally named `on`/`off`/`yes`/`no` into
    `True`/`False`, so the two sides cannot be compared reliably.
    """
    errors: list[str] = []
    for step in job.get("steps") or []:
        uses = str(step.get("uses") or "")
        if not is_repo_action(uses):
            continue
        meta = get_meta(uses)
        if meta is None:  # skipped fetch or already-reported 404
            continue
        with_keys = {key for key in step.get("with") or {} if isinstance(key, str)}
        errors.extend(
            f"action: job '{job_id}' passes undeclared input '{key}' to '{uses}'"
            for key in sorted(with_keys - declared_input_keys(meta))
        )
        errors.extend(
            f"action: job '{job_id}' omits required input '{name}' of '{uses}'"
            for name in missing_required_inputs(meta, with_keys)
        )
    return errors


def check_step_outputs(job_id: str, job: dict, get_meta) -> list[str]:
    """Check 5b: every `steps.<id>.outputs.<x>` read from an action step resolves.

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


def check_action_compat(wf: dict, fetch, cache: dict | None = None) -> list[str]:
    """Check 5: `with:` keys and step-output reads match published action metadata.

    `fetch` maps an action ref to its parsed `action.yml` mapping, returns
    `None` for a skipped (network-unavailable) fetch, or raises
    `ActionNotFoundError` on a real 404. It is injected so tests supply an
    in-memory fake; production passes `fetch_action`. Fetches are cached per
    distinct ref, so a run costs one request per unique action. Pass `cache`
    to inspect resolution counts after the run — `main()` uses this for its
    summary line, so an all-skipped run is visible rather than silently green.
    """
    errors: list[str] = []
    get_meta = functools.partial(
        cached_fetch, fetch=fetch, cache={} if cache is None else cache, errors=errors
    )
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
        + check_gem_series(wf)
    )


def main() -> None:
    """Entry point: print one error per line and exit 1 if any check fails."""
    parser = argparse.ArgumentParser(
        description="Static checks for the release workflow (guards, artifacts, needs, gem series)."
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
        cache: dict = {}
        errors.extend(check_action_compat(wf, fetch_action, cache))
        resolved = sum(1 for meta in cache.values() if meta is not None)
        print(
            f"action-inputs: resolved {resolved} of {len(cache)} action refs "
            f"({len(cache) - resolved} skipped)"
        )
    for error in errors:
        print(error)
    if errors:
        sys.exit(1)
    print(f"OK: {args.workflow} passed release-workflow static checks.")


if __name__ == "__main__":
    main()
