"""Tests for the release-workflow static gate in ``scripts/check_release_workflow.py``.

The anchor test runs every check against the real, tracked ``.github/workflows/release.yml``
and asserts zero errors — the gate must pass on the workflow exactly as it stands. The
mutation tests each write a modified copy of that file into ``tmp_path`` (the tracked
workflow is never touched) and assert that the specific check fires: guard wrapper
stripped, ``prepare-release`` wrapped, a registry flag typo'd, an upload renamed away from
its downloads, and a broken ``needs:`` entry.

The action-input compatibility check (``--check-action-inputs``) is exercised through an
injected in-memory fake fetcher — this suite never touches the network; the real fetch
runs only in the dedicated ``release-workflow`` CI job.
"""

import importlib.util
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
