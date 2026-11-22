"""Tests for ``scripts/ffi_release_assets.py`` and its wiring in the release workflow.

The script tests write fake archives and a ``gh release view --json assets`` fixture in
``tmp_path`` — the asset shape is copied from the real v0.7.0 release. The wiring tests
read the real ``.github/workflows/release.yml``: the workflow is
``workflow_dispatch``-only, so these tests are the gate that keeps ``publish-ffi``
pruning ``starter`` assets before the upload and verifying the result after it.
"""

import importlib.util
import json
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
VERSION = "9.8.7"
ARCHIVES = [
    f"iscc-ffi-v{VERSION}-x86_64-unknown-linux-gnu.tar.gz",
    f"iscc-ffi-v{VERSION}-x86_64-pc-windows-msvc.zip",
]
XCFRAMEWORK = "IsccLib.xcframework.zip"

# Load scripts/ffi_release_assets.py by path — it is a repo script, not a package.
_SCRIPT_PATH = ROOT / "scripts" / "ffi_release_assets.py"
_spec = importlib.util.spec_from_file_location("ffi_release_assets", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
fra = importlib.util.module_from_spec(_spec)
_loader.exec_module(fra)


def _asset(name, state, asset_id):
    """Return one asset in the shape of ``gh release view --json assets``."""
    return {
        "apiUrl": f"https://api.github.com/repos/iscc/iscc-lib/releases/assets/{asset_id}",
        "contentType": "application/gzip",
        "createdAt": "2026-10-17T19:19:50Z",
        "digest": "sha256:4b3dcf5e6eb93bbb5aeba08a53f1bfbdb8205b2b0e2c47eabcd4fc4405f243e1",
        "downloadCount": 3,
        "id": f"RA_kwDORWP6a84m0w{asset_id}",
        "label": "",
        "name": name,
        "size": 0 if state == "starter" else 2137326,
        "state": state,
        "updatedAt": "2026-10-17T19:19:50Z",
        "url": f"https://github.com/iscc/iscc-lib/releases/download/v{VERSION}/{name}",
    }


def _setup(tmp_path, assets, staged=ARCHIVES):
    """Stage fake archives and write the assets JSON; return ``(staging, json)``."""
    staging = tmp_path / "ffi-staging"
    staging.mkdir()
    for name in staged:
        (staging / name).write_bytes(f"fake archive {name}".encode())
    assets_json = tmp_path / "assets.json"
    assets_json.write_text(json.dumps({"assets": assets}), encoding="utf-8")
    return staging, assets_json


def _run(mode, staging, assets_json):
    """Run the script's ``main`` and return its exit code."""
    return fra.main([mode, str(staging), str(assets_json)])


def test_stale_lists_starter_asset(tmp_path, capsys):
    # A staged archive whose asset is `starter` is printed by its apiUrl, by itself.
    starter = _asset(ARCHIVES[0], "starter", 101)
    staging, assets_json = _setup(
        tmp_path, [starter, _asset(ARCHIVES[1], "uploaded", 102)]
    )
    assert _run("stale", staging, assets_json) == 0
    assert capsys.readouterr().out.splitlines() == [starter["apiUrl"]]


def test_stale_skips_uploaded_asset(tmp_path, capsys):
    # Uploaded assets are kept: the portfile pins their hashes.
    assets = [_asset(name, "uploaded", 200 + i) for i, name in enumerate(ARCHIVES)]
    staging, assets_json = _setup(tmp_path, assets)
    assert _run("stale", staging, assets_json) == 0
    assert capsys.readouterr().out == ""


def test_stale_ignores_unstaged_starter_asset(tmp_path, capsys):
    # A `starter` asset that is not a staged archive is none of the job's business.
    assets = [
        _asset(XCFRAMEWORK, "starter", 300),
        _asset(f"iscc-ffi-v{VERSION}-aarch64-apple-darwin.tar.gz", "starter", 301),
    ]
    staging, assets_json = _setup(tmp_path, assets)
    assert _run("stale", staging, assets_json) == 0
    assert capsys.readouterr().out == ""


def test_verify_fails_on_missing_archive(tmp_path, capsys):
    # One staged archive absent from the release fails the check and is named.
    staging, assets_json = _setup(tmp_path, [_asset(ARCHIVES[0], "uploaded", 400)])
    assert _run("verify", staging, assets_json) == 1
    err = capsys.readouterr().err
    assert ARCHIVES[1] in err
    assert ARCHIVES[0] not in err


def test_verify_fails_on_starter_archive(tmp_path, capsys):
    # One staged archive left in state `starter` fails the check and is named.
    assets = [
        _asset(ARCHIVES[0], "uploaded", 500),
        _asset(ARCHIVES[1], "starter", 501),
    ]
    staging, assets_json = _setup(tmp_path, assets)
    assert _run("verify", staging, assets_json) == 1
    err = capsys.readouterr().err
    assert ARCHIVES[1] in err
    assert "starter" in err
    assert ARCHIVES[0] not in err


def test_verify_passes_when_all_uploaded(tmp_path, capsys):
    # Every staged archive uploaded passes; unstaged assets do not matter.
    assets = [_asset(name, "uploaded", 600 + i) for i, name in enumerate(ARCHIVES)]
    assets.append(_asset(XCFRAMEWORK, "starter", 610))
    staging, assets_json = _setup(tmp_path, assets)
    assert _run("verify", staging, assets_json) == 0
    assert capsys.readouterr().err == ""


def test_empty_staging_dir_fails(tmp_path):
    # Both modes refuse an empty staging dir: an empty set check proves nothing.
    staging, assets_json = _setup(tmp_path, [], staged=[])
    assert _run("verify", staging, assets_json) == 1
    assert _run("stale", staging, assets_json) == 1


def test_staged_names_use_upload_glob(tmp_path):
    # Files outside the upload step's glob are not staged archives.
    staging, _ = _setup(tmp_path, [], staged=[*ARCHIVES, "notes.txt"])
    assert fra.staged_names(staging) == sorted(ARCHIVES)


def _ffi_steps():
    """Return the steps of the real ``publish-ffi`` job."""
    workflow = yaml.safe_load(WORKFLOW.read_text(encoding="utf-8"))
    return workflow["jobs"]["publish-ffi"]["steps"]


def _step_index(steps, predicate, what):
    """Return the index of the one step matching ``predicate``."""
    hits = [i for i, step in enumerate(steps) if predicate(step)]
    assert len(hits) == 1, f"expected exactly one {what} step, found {len(hits)}"
    return hits[0]


def _script_step(steps, mode):
    """Return the index of the one step running the script in ``mode``."""
    return _step_index(
        steps,
        lambda s: f"ffi_release_assets.py {mode}" in s.get("run", ""),
        f"ffi_release_assets.py {mode}",
    )


def _upload_step(steps):
    """Return the index of the one softprops upload step."""
    return _step_index(
        steps,
        lambda s: s.get("uses", "").startswith("softprops/action-gh-release@"),
        "gh-release",
    )


def test_publish_ffi_prunes_before_upload_and_verifies_after():
    # prune < upload < verify: the rerun re-uploads `starter` assets, then checks them.
    steps = _ffi_steps()
    assert _script_step(steps, "stale") < _upload_step(steps)
    assert _upload_step(steps) < _script_step(steps, "verify")


def test_publish_ffi_script_steps_read_ffi_staging():
    # Both script steps read the staging dir the upload step uploads from.
    steps = _ffi_steps()
    for mode in ("stale", "verify"):
        run = steps[_script_step(steps, mode)]["run"]
        assert f"ffi_release_assets.py {mode} ffi-staging " in run
    assert steps[_upload_step(steps)]["with"]["files"].startswith("ffi-staging/")


def test_publish_ffi_script_steps_take_tag_from_env():
    # The tag reaches `run` through env, never inlined; gh gets the job token.
    steps = _ffi_steps()
    for mode in ("stale", "verify"):
        step = steps[_script_step(steps, mode)]
        assert "${{" not in step["run"]
        assert step["env"]["TAG"] == "v${{ steps.version.outputs.version }}"
        assert "github.token" in step["env"]["GH_TOKEN"]


def test_publish_ffi_keeps_uploaded_archives():
    # Pruning only touches unfinished assets; uploaded ones are never replaced.
    steps = _ffi_steps()
    assert steps[_upload_step(steps)]["with"]["overwrite_files"] is False
