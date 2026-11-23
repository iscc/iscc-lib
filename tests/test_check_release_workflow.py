"""Tests for the release-workflow static gate in ``scripts/check_release_workflow.py``.

The anchor test runs every check against the real, tracked ``.github/workflows/release.yml``
and asserts zero errors — the gate must pass on the workflow exactly as it stands. The
mutation tests each write a modified copy of that file into ``tmp_path`` (the tracked
workflow is never touched) and assert that the specific check fires: guard wrapper
stripped, ``prepare-release`` wrapped, a registry flag typo'd, an upload renamed away from
its downloads, and a broken ``needs:`` entry.

The gem-series check is tested on a small synthetic workflow whose Ruby series are fixture
values, so that a change of the released series leaves these tests alone. The tests of the
test script and of the publish job mutate the tracked workflow, which also shows that the
check runs as part of the gate.

The action-input compatibility check (``--check-action-inputs``) is exercised through an
injected in-memory fake fetcher — this suite never touches the network; the real fetch
runs only in the dedicated ``release-workflow`` CI job.
"""

import http.client
import importlib.util
import io
from pathlib import Path

# Load scripts/check_release_workflow.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = (
    Path(__file__).resolve().parents[1] / "scripts" / "check_release_workflow.py"
)
_spec = importlib.util.spec_from_file_location("check_release_workflow", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
crw = importlib.util.module_from_spec(_spec)
_loader.exec_module(crw)

WORKFLOW = Path(__file__).resolve().parents[1] / ".github" / "workflows" / "release.yml"


def _mutate(tmp_path, old, new, count=-1):
    """Write a copy of the tracked release workflow with `old` replaced by `new`.

    Asserts the needle is present so a workflow refactor cannot silently turn a
    mutation test into a no-op. Returns the mutated copy's path.
    """
    text = WORKFLOW.read_text(encoding="utf-8")
    assert old in text, f"mutation needle not found in release.yml: {old!r}"
    mutated = tmp_path / "release.yml"
    mutated.write_text(text.replace(old, new, count), encoding="utf-8")
    return mutated


def _run_all(path):
    """Parse a workflow file and return the combined error list of all checks."""
    return crw.run_checks(crw.load_workflow(path))


def test_real_workflow_passes():
    # The gate must pass on the tracked workflow exactly as committed.
    assert _run_all(WORKFLOW) == []


def test_guard_wrapper_stripped_fires(tmp_path):
    # Removing the !cancelled() && !failure() wrapper from one job trips check 1a.
    mutated = _mutate(
        tmp_path,
        "if: ${{ !cancelled() && !failure() && (inputs.version != '' || inputs.crates-io) }}",
        "if: ${{ inputs.version != '' || inputs.crates-io }}",
    )
    errors = _run_all(mutated)
    assert any(e.startswith("guard: job 'publish-crates-io'") for e in errors)


def test_missing_if_fires(tmp_path):
    # Deleting a job's `if` line entirely also trips check 1a.
    mutated = _mutate(
        tmp_path,
        "    if: ${{ !cancelled() && !failure() && (inputs.version != '' || inputs.crates-io) }}\n",
        "",
    )
    errors = _run_all(mutated)
    assert "guard: job 'publish-crates-io' has no `if` condition" in errors


def test_prepare_release_must_stay_unguarded(tmp_path):
    # Wrapping prepare-release in the guard shape must fail: the wrapped form would
    # run the tag-pushing job on registry-only dispatches.
    mutated = _mutate(
        tmp_path,
        "    if: inputs.version != ''\n",
        "    if: ${{ !cancelled() && !failure() && (inputs.version != '') }}\n",
    )
    errors = _run_all(mutated)
    assert any(e.startswith("guard: job 'prepare-release' must keep") for e in errors)


def test_guard_without_version_alternative_fires(tmp_path):
    # A wrapped guard whose inner disjunction drops `inputs.version != ''` trips 1a.
    mutated = _mutate(
        tmp_path,
        "if: ${{ !cancelled() && !failure() && (inputs.version != '' || inputs.crates-io) }}",
        "if: ${{ !cancelled() && !failure() && (inputs.crates-io) }}",
    )
    errors = _run_all(mutated)
    assert any(
        e.startswith("guard: job 'publish-crates-io' guard lacks") for e in errors
    )


def test_renamed_registry_flag_fires_both_directions(tmp_path):
    # Typo'ing a flag in the job `if`s (declaration unchanged) trips check 1b twice:
    # the reference is undeclared AND the declared flag is no longer referenced.
    mutated = _mutate(tmp_path, "inputs.rubygems", "inputs.rubygemz")
    errors = _run_all(mutated)
    assert "inputs: job `if` references undeclared input 'inputs.rubygemz'" in errors
    assert (
        "inputs: declared registry input 'rubygems' is not referenced by any job `if`"
        in errors
    )


def test_renamed_upload_breaks_download_resolution(tmp_path):
    # Renaming an upload `name:` so its downloads no longer resolve trips check 2.
    # count=1 hits the build-wasm upload (first occurrence); the test-wasm and
    # publish-npm-wasm downloads keep referencing `wasm-pkg`.
    mutated = _mutate(tmp_path, "name: wasm-pkg", "name: wasm-package", count=1)
    errors = _run_all(mutated)
    assert (
        "artifact: download 'wasm-pkg' in job 'test-wasm' matches no "
        "upload-artifact name" in errors
    )


def test_broken_needs_entry_fires(tmp_path):
    # A `needs:` entry naming a non-existent job trips check 3.
    mutated = _mutate(
        tmp_path, "needs: [build-wasm]", "needs: [build-wasm-typo]", count=1
    )
    errors = _run_all(mutated)
    assert "needs: job 'test-wasm' needs undeclared job 'build-wasm-typo'" in errors


GEM_LOAD_RULE = (
    "every Ruby series has to load the gem, and a failure has to fail the job"
)
GEM_SCRIPT_ERROR = (
    "gem-series: job 'test-gem': a `run` step must consist of exactly the lines "
    + ", ".join(f"`{line}`" for line in crw.GEM_TEST_SCRIPT)
    + f": {GEM_LOAD_RULE}"
)


def _gem_test_step(wf):
    """Return the position (from 1) and the step of the gem test job that runs the self-test."""
    steps = wf["jobs"]["test-gem"]["steps"]
    return next(
        (position, step)
        for position, step in enumerate(steps, start=1)
        if crw.GEM_SELFTEST_COMMAND in str(step.get("run", ""))
    )


def _gem_wf(built="3.3, 3.4", **matrix):
    """Build a minimal workflow of a cross-gem build job, a gem test and a gem publish job.

    The Ruby series are fixture values, independent of the ones the tracked workflow
    releases. Keyword arguments override or extend the test job's matrix.
    """
    cross_gem = {
        "uses": "oxidize-rb/actions/cross-gem@v1",
        "with": {"ruby-versions": built},
    }
    setup_ruby = {
        "uses": "ruby/setup-ruby@v1",
        "with": {"ruby-version": "${{ matrix.ruby }}"},
    }
    load_gem = {"run": "\n".join(crw.GEM_TEST_SCRIPT) + "\n"}
    return {
        "jobs": {
            "build-gem": {"steps": [cross_gem]},
            "test-gem": {
                "strategy": {"matrix": {"ruby": ["3.3", "3.4"], **matrix}},
                "steps": [setup_ruby, load_gem],
            },
            "publish-rubygems": {"needs": ["build-gem", "test-gem"]},
        }
    }


def test_gem_series_clean():
    # A test matrix equal to the cross-gem list, with the full test script, passes.
    assert crw.check_gem_series(_gem_wf()) == []


def test_gem_series_built_but_not_tested_fires():
    # A Ruby series added to the cross-gem build and not to the test matrix trips check 4.
    expected = (
        "gem-series: job 'test-gem' matrix `ruby` ['3.3', '3.4'] differs from the "
        "cross-gem `ruby-versions` ['3.3', '3.4', '4.0'] of job 'build-gem'"
    )
    assert crw.check_gem_series(_gem_wf(built="3.3, 3.4, 4.0")) == [expected]


def test_gem_series_missing_build_list_fires():
    # Without a cross-gem `ruby-versions` list there is nothing to compare: check 4.
    assert crw.check_gem_series(_gem_wf(built="")) == [
        "gem-series: job 'build-gem' has no cross-gem `ruby-versions` list"
    ]


def test_gem_series_exclude_fires():
    # `exclude` drops a listed series from the jobs that run, whatever the `ruby` axis says.
    expected = (
        "gem-series: job 'test-gem' matrix must not use `exclude`: "
        "every listed Ruby series has to run"
    )
    assert crw.check_gem_series(_gem_wf(exclude=[{"ruby": "3.4"}])) == [expected]


def test_gem_series_conditional_load_step_fires():
    # A condition on the tracked workflow's step that installs and loads the gem would let
    # a series pass the job unloaded.
    wf = crw.load_workflow(WORKFLOW)
    position, step = _gem_test_step(wf)
    step["if"] = "matrix.ruby == 'one-series'"
    expected = (
        f"gem-series: job 'test-gem': step {position} must not carry `if`: "
        + GEM_LOAD_RULE
    )
    assert crw.run_checks(wf) == [expected]


def test_gem_series_step_that_cannot_fail_fires():
    # A step with `continue-on-error` runs on every series and never fails the job.
    wf = _gem_wf()
    wf["jobs"]["test-gem"]["steps"][-1]["continue-on-error"] = True
    expected = (
        "gem-series: job 'test-gem': step 2 must not carry `continue-on-error`: "
        + GEM_LOAD_RULE
    )
    assert crw.check_gem_series(wf) == [expected]


def test_gem_series_job_that_cannot_fail_fires():
    # A job with `continue-on-error` reports success whatever its legs do.
    wf = _gem_wf()
    wf["jobs"]["test-gem"]["continue-on-error"] = True
    expected = (
        "gem-series: job 'test-gem': the job must not carry `continue-on-error`: "
        + GEM_LOAD_RULE
    )
    assert crw.check_gem_series(wf) == [expected]


def test_gem_series_install_without_selftest_fires():
    # The tracked workflow's test step reduced to an install no longer loads the extension.
    wf = crw.load_workflow(WORKFLOW)
    _gem_test_step(wf)[1]["run"] = "gem install gems/*.gem"
    assert crw.run_checks(wf) == [GEM_SCRIPT_ERROR]


def test_gem_series_ruby_check_removed_fires(tmp_path):
    # Without the check of the running Ruby, a later Ruby setup step pinned to one series
    # makes every leg load that series' binary.
    mutated = _mutate(tmp_path, crw.GEM_RUBY_CHECK_COMMAND, "ruby -v")
    assert _run_all(mutated) == [GEM_SCRIPT_ERROR]


def test_gem_series_published_gem_installed_fires(tmp_path):
    # Installing the gem from rubygems.org tests a published version, not the built one.
    mutated = _mutate(tmp_path, crw.GEM_INSTALL_COMMAND, "gem install iscc-lib")
    assert _run_all(mutated) == [GEM_SCRIPT_ERROR]


def test_gem_series_shell_condition_fires():
    # A shell `if` around the self-test line skips it on every other series.
    wf = crw.load_workflow(WORKFLOW)
    step = _gem_test_step(wf)[1]
    command = crw.GEM_SELFTEST_COMMAND
    wrapped = f'if [ "${{{{ matrix.ruby }}}}" = "3.3" ]; then\n  {command}\nfi'
    step["run"] = step["run"].replace(command, wrapped)
    assert command in crw.run_lines(step)
    assert crw.run_checks(wf) == [GEM_SCRIPT_ERROR]


def test_gem_series_commented_out_selftest_fires(tmp_path):
    # A self-test line turned into a shell comment is still in the script and never runs.
    command = crw.GEM_SELFTEST_COMMAND
    assert _run_all(_mutate(tmp_path, command, f"# {command}")) == [GEM_SCRIPT_ERROR]


def test_gem_series_unchecked_selftest_fires(tmp_path):
    # Without the `raise`, a false result leaves the exit status of `ruby -e` at 0.
    checked = "raise 'Conformance failed' unless IsccLib.conformance_selftest"
    mutated = _mutate(tmp_path, checked, "IsccLib.conformance_selftest")
    assert _run_all(mutated) == [GEM_SCRIPT_ERROR]


def test_gem_series_publish_that_does_not_wait_fires():
    # A publish job that does not need the test job can start before a series is loaded.
    wf = crw.load_workflow(WORKFLOW)
    wf["jobs"]["publish-rubygems"]["needs"].remove("test-gem")
    expected = (
        "gem-series: job 'publish-rubygems' must need job 'test-gem': "
        "the publish has to wait for the gem test"
    )
    assert crw.run_checks(wf) == [expected]


def test_gem_series_test_skipped_on_a_gem_dispatch_fires():
    # Without the registry flag the test job is skipped on a `rubygems` dispatch, and the
    # `!failure()` guard of the publish job lets a skipped need pass.
    wf = crw.load_workflow(WORKFLOW)
    wf["jobs"]["test-gem"]["if"] = (
        "${{ !cancelled() && !failure() && (inputs.version != '') }}"
    )
    expected = (
        "gem-series: jobs 'test-gem' and 'publish-rubygems' must carry the same `if`: "
        "a dispatch that publishes the gems has to test them"
    )
    assert crw.run_checks(wf) == [expected]


def test_gem_series_check_runs_on_the_tracked_workflow():
    # The check is wired into run_checks: dropping the last tested series from the parsed
    # tracked workflow, whichever series it lists, is reported.
    wf = crw.load_workflow(WORKFLOW)
    wf["jobs"]["test-gem"]["strategy"]["matrix"]["ruby"].pop()
    assert any(error.startswith("gem-series: ") for error in crw.run_checks(wf))


def test_matrix_include_expansion():
    # `${{ matrix.<k> }}` expands per include entry; unknown spans collapse to `*`.
    job = {
        "strategy": {
            "matrix": {
                "include": [
                    {"os": "ubuntu-latest", "target": "x86_64"},
                    {"os": "macos-14", "target": "universal2"},
                ]
            }
        }
    }
    assert crw.expand_reference(
        "wheels-${{ matrix.os }}-${{ matrix.target }}", job
    ) == [
        "wheels-macos-14-universal2",
        "wheels-ubuntu-latest-x86_64",
    ]
    assert crw.expand_reference("x-${{ github.sha }}", {}) == ["x-*"]


def test_references_match_is_symmetric_over_wildcards():
    # Download glob vs literal upload, literal download vs upload glob, and a
    # non-overlapping pair.
    assert crw.references_match("wheels-*", "wheels-ubuntu-latest-x86_64")
    assert crw.references_match("gem-x86_64-linux", "gem-*")
    assert crw.references_match("nuget-package", "nuget-package")
    assert not crw.references_match("wasm-pkg", "wasm-package")


def test_action_yml_urls_ref_shapes():
    # The four ref shapes present in release.yml: plain tag, sub-path action,
    # slash-containing ref, and branch ref.
    raw = "https://raw.githubusercontent.com"
    assert crw.action_yml_urls("actions/checkout@v7") == [
        f"{raw}/actions/checkout/v7/action.yml",
        f"{raw}/actions/checkout/v7/action.yaml",
    ]
    assert crw.action_yml_urls("oxidize-rb/actions/cross-gem@v1") == [
        f"{raw}/oxidize-rb/actions/v1/cross-gem/action.yml",
        f"{raw}/oxidize-rb/actions/v1/cross-gem/action.yaml",
    ]
    assert crw.action_yml_urls("pypa/gh-action-pypi-publish@release/v1") == [
        f"{raw}/pypa/gh-action-pypi-publish/release/v1/action.yml",
        f"{raw}/pypa/gh-action-pypi-publish/release/v1/action.yaml",
    ]
    assert crw.action_yml_urls("dtolnay/rust-toolchain@stable") == [
        f"{raw}/dtolnay/rust-toolchain/stable/action.yml",
        f"{raw}/dtolnay/rust-toolchain/stable/action.yaml",
    ]


def test_is_repo_action_skips_non_repo_refs():
    # Docker refs, local actions, and refs without an @<gitref> are not fetchable.
    assert crw.is_repo_action("actions/checkout@v7")
    assert not crw.is_repo_action("docker://alpine:3.20")
    assert not crw.is_repo_action("./local-action")
    assert not crw.is_repo_action("actions/checkout")


# In-memory action metadata for check_action_compat tests — the fake fetcher
# resolves refs from this table so the suite never touches the network.
FAKE_ACTIONS = {
    "acme/setup@v1": {
        "inputs": {"version": {}, "cache": {}},
        "outputs": {"path": {}},
    },
    # Required-input shapes: bool `required`, quoted "true", required-with-default.
    "acme/publish@v2": {
        "inputs": {
            "token": {"required": True},
            "tag": {"required": "true"},
            "level": {"required": True, "default": "info"},
            "verbose": {},
        },
    },
    # Docker action: accepts the GitHub-native `args`/`entrypoint` overrides.
    "acme/docker-act@v1": {
        "inputs": {"level": {}},
        "runs": {"using": "docker", "image": "Dockerfile"},
    },
}


def _fake_fetch(ref):
    """Resolve an action ref from FAKE_ACTIONS; unknown refs raise a 404 error."""
    meta = FAKE_ACTIONS.get(ref)
    if meta is None:
        raise crw.ActionNotFoundError(f"no action.yml or action.yaml found for '{ref}'")
    return meta


def _job_wf(steps):
    """Wrap a step list into a minimal single-job workflow mapping."""
    return {"jobs": {"build": {"steps": steps}}}


def test_action_compat_clean():
    # Declared `with:` keys and a declared action output produce no errors.
    wf = _job_wf(
        [
            {"id": "setup", "uses": "acme/setup@v1", "with": {"version": "3"}},
            {"run": "echo ${{ steps.setup.outputs.path }}"},
        ]
    )
    assert crw.check_action_compat(wf, _fake_fetch) == []


def test_action_compat_undeclared_with_key_fires():
    # A `with:` key absent from the action's declared inputs is one error
    # naming both the key and the ref.
    wf = _job_wf([{"uses": "acme/setup@v1", "with": {"version": "3", "cachez": "x"}}])
    errors = crw.check_action_compat(wf, _fake_fetch)
    assert errors == [
        "action: job 'build' passes undeclared input 'cachez' to 'acme/setup@v1'"
    ]


def test_action_compat_undeclared_step_output_fires():
    # Reading an output the action does not declare is one error.
    wf = _job_wf(
        [
            {"id": "setup", "uses": "acme/setup@v1"},
            {"run": "echo ${{ steps.setup.outputs.pathz }}"},
        ]
    )
    errors = crw.check_action_compat(wf, _fake_fetch)
    expected = (
        "action: job 'build' reads undeclared output "
        "'steps.setup.outputs.pathz' from 'acme/setup@v1'"
    )
    assert errors == [expected]


def test_action_compat_missing_action_yml_fires():
    # A ref whose action.yml/action.yaml both 404 is a real defect: one error,
    # reported once despite two steps using the same ref (fetches are cached).
    wf = _job_wf(
        [
            {"uses": "acme/gone@v9", "with": {"anything": "x"}},
            {"uses": "acme/gone@v9"},
        ]
    )
    errors = crw.check_action_compat(wf, _fake_fetch)
    assert errors == ["action: no action.yml or action.yaml found for 'acme/gone@v9'"]


def test_action_compat_skipped_fetch_is_not_an_error():
    # A fetcher returning None (network unavailable) degrades to a skip.
    wf = _job_wf(
        [
            {"id": "setup", "uses": "acme/setup@v1", "with": {"bogus": "x"}},
            {"run": "echo ${{ steps.setup.outputs.bogus }}"},
        ]
    )
    assert crw.check_action_compat(wf, lambda ref: None) == []


def test_action_compat_ignores_local_run_step_outputs():
    # `steps.<id>.outputs.<x>` from a local `run:` step (no `uses:`) has no
    # published metadata and must not error; same for ids not in the job.
    wf = _job_wf(
        [
            {"id": "check", "run": "echo skip=true >> $GITHUB_OUTPUT"},
            {"run": "echo ${{ steps.check.outputs.skip }}"},
            {"run": "echo ${{ steps.elsewhere.outputs.thing }}"},
        ]
    )
    assert crw.check_action_compat(wf, _fake_fetch) == []


def test_action_compat_omitted_required_input_fires():
    # A declared `required: true` input without a default that the step omits
    # is one error; quoted "true" counts, required-with-default does not.
    wf = _job_wf([{"uses": "acme/publish@v2", "with": {"token": "x"}}])
    errors = crw.check_action_compat(wf, _fake_fetch)
    assert errors == [
        "action: job 'build' omits required input 'tag' of 'acme/publish@v2'"
    ]


def test_action_compat_required_inputs_provided_clean():
    # Passing every required-without-default input yields no errors; optional
    # and defaulted inputs may be omitted freely.
    wf = _job_wf([{"uses": "acme/publish@v2", "with": {"token": "x", "tag": "v1"}}])
    assert crw.check_action_compat(wf, _fake_fetch) == []


def test_action_compat_docker_native_overrides_allowed():
    # `args`/`entrypoint` on a docker action are GitHub-native, never declared
    # as inputs, and must not be reported.
    wf = _job_wf(
        [
            {
                "uses": "acme/docker-act@v1",
                "with": {"args": "run", "entrypoint": "/bin/sh", "level": "3"},
            }
        ]
    )
    assert crw.check_action_compat(wf, _fake_fetch) == []


def test_action_compat_docker_overrides_rejected_for_node_action():
    # The same keys on a non-docker action are still undeclared inputs.
    wf = _job_wf([{"uses": "acme/setup@v1", "with": {"args": "x"}}])
    errors = crw.check_action_compat(wf, _fake_fetch)
    assert errors == [
        "action: job 'build' passes undeclared input 'args' to 'acme/setup@v1'"
    ]


def test_action_compat_skips_non_string_with_keys():
    # PyYAML's YAML 1.1 booleans turn a `with:` key literally named `on` into
    # True; boolean keys are skipped rather than reported as undeclared.
    wf = _job_wf([{"uses": "acme/setup@v1", "with": {True: "x", "version": "3"}}])
    assert crw.check_action_compat(wf, _fake_fetch) == []


def test_action_compat_cache_records_resolution():
    # A caller-supplied cache exposes resolution counts after the run: one
    # resolved ref, one 404 recorded as None (main() prints these as summary).
    wf = _job_wf([{"uses": "acme/setup@v1"}, {"uses": "acme/gone@v9"}])
    cache = {}
    errors = crw.check_action_compat(wf, _fake_fetch, cache)
    assert errors == ["action: no action.yml or action.yaml found for 'acme/gone@v9'"]
    assert len(cache) == 2
    assert sum(1 for meta in cache.values() if meta is not None) == 1


def test_fetch_action_incomplete_read_is_skipped(monkeypatch, capsys):
    # A truncated response (http.client.IncompleteRead is an HTTPException,
    # not an OSError) degrades to a warning plus None, never an exception.
    def _raise_incomplete(url, timeout):
        raise http.client.IncompleteRead(b"")

    monkeypatch.setattr(crw.urllib.request, "urlopen", _raise_incomplete)
    assert crw.fetch_action("acme/setup@v1") is None
    assert "warning: skipped acme/setup@v1" in capsys.readouterr().err


def test_fetch_action_unparseable_body_is_skipped(monkeypatch, capsys):
    # A body that is not valid YAML (captive portal / proxy page) raises
    # yaml.YAMLError inside the fetch and degrades to a warning plus None.
    monkeypatch.setattr(
        crw.urllib.request, "urlopen", lambda url, timeout: io.BytesIO(b"a: b\n- c\n")
    )
    assert crw.fetch_action("acme/setup@v1") is None
    assert "warning: skipped acme/setup@v1" in capsys.readouterr().err


def test_declared_inputs_handles_yaml_bool_on_key():
    # YAML 1.1 parses a bare `on:` key as boolean True; both spellings must work.
    inputs = {"version": {}, "pypi": {}}
    assert crw.declared_inputs({True: {"workflow_dispatch": {"inputs": inputs}}}) == {
        "version",
        "pypi",
    }
    assert crw.declared_inputs({"on": {"workflow_dispatch": {"inputs": inputs}}}) == {
        "version",
        "pypi",
    }
