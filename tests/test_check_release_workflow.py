"""Tests for the release-workflow static gate in ``scripts/check_release_workflow.py``.

The anchor test runs every check against the real, tracked ``.github/workflows/release.yml``
and asserts zero errors — the gate must pass on the workflow exactly as it stands. The
mutation tests each write a modified copy of that file into ``tmp_path`` (the tracked
workflow is never touched) and assert that the specific check fires: guard wrapper
stripped, ``prepare-release`` wrapped, a registry flag typo'd, an upload renamed away from
its downloads, and a broken ``needs:`` entry.
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
