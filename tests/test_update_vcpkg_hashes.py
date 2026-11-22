"""Tests for ``scripts/update_vcpkg_hashes.py`` and its wiring in the release workflow.

The script tests run against a copy of the real portfile and a ``vcpkg.json`` written in
``tmp_path``, with small fake archives — no network, and the tracked files are never
touched. The wiring tests read the real ``.github/workflows/release.yml``: the workflow is
``workflow_dispatch``-only, so these tests are the gate that keeps ``prepare-release``
writing the hashes from the ``build-ffi`` archives into the commit it tags.
"""

import hashlib
import importlib.util
import json
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
PORTFILE = ROOT / "packages" / "cpp" / "portfile.cmake"
WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"
VERSION = "9.8.7"

# Load scripts/update_vcpkg_hashes.py by path — it is a repo script, not a package.
_SCRIPT_PATH = ROOT / "scripts" / "update_vcpkg_hashes.py"
_spec = importlib.util.spec_from_file_location("update_vcpkg_hashes", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
uvh = importlib.util.module_from_spec(_spec)
_loader.exec_module(uvh)


def _targets():
    """Return the target branches parsed from the real portfile."""
    return uvh.parse_targets(PORTFILE.read_bytes().decode("utf-8"))


def _setup(tmp_path, version=VERSION):
    """Copy the portfile, write a manifest and one fake archive per target.

    Archives are named for ``version``; the manifest always says ``VERSION``.
    Returns ``(portfile, manifest, archive_dir, {target: archive_path})``.
    """
    portfile = tmp_path / "portfile.cmake"
    portfile.write_bytes(PORTFILE.read_bytes())
    manifest = tmp_path / "vcpkg.json"
    manifest.write_text(json.dumps({"name": "iscc", "version": VERSION}))
    archive_dir = tmp_path / "archives"
    archive_dir.mkdir()
    archives = {}
    for target in _targets():
        path = archive_dir / f"iscc-ffi-v{version}-{target.name}{target.ext}"
        path.write_bytes(f"fake archive for {target.name}".encode())
        archives[target.name] = path
    return portfile, manifest, archive_dir, archives


def _run(portfile, manifest, archive_dir, *extra):
    """Run the script's ``main`` on the given files and return its exit code."""
    argv = [str(archive_dir), "--portfile", str(portfile), "--manifest", str(manifest)]
    return uvh.main([*argv, *extra])


def _mask_hashes(text):
    """Return the portfile text with every ISCC_SHA512 value blanked out."""
    return uvh.SHA_RE.sub('set(ISCC_SHA512 "")', text)


def test_portfile_has_targets():
    # Guard against an empty target set making every other test vacuous.
    assert len(_targets()) >= 5


def test_rewrite_sets_each_hash(tmp_path):
    # Each ISCC_SHA512 becomes the sha512 of its archive; nothing else changes.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    text = portfile.read_bytes().decode("utf-8")
    for target in uvh.parse_targets(text):
        expected = hashlib.sha512(archives[target.name].read_bytes()).hexdigest()
        assert target.sha512 == expected
    original = PORTFILE.read_bytes().decode("utf-8")
    assert _mask_hashes(text) == _mask_hashes(original)
    assert text != original


def test_missing_archive_writes_nothing(tmp_path):
    # One missing archive fails the run and leaves the portfile byte-identical.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    archives[_targets()[-1].name].unlink()
    assert _run(portfile, manifest, archive_dir) == 1
    assert portfile.read_bytes() == PORTFILE.read_bytes()


def test_wrong_version_archive_counts_as_missing(tmp_path):
    # Archives named for another version do not satisfy the manifest version.
    portfile, manifest, archive_dir, _ = _setup(tmp_path, version="1.2.3")
    assert _run(portfile, manifest, archive_dir) == 1
    assert portfile.read_bytes() == PORTFILE.read_bytes()


def test_missing_archive_is_named(tmp_path, capsys):
    # The error output names the missing archive file.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    missing = archives[_targets()[0].name]
    missing.unlink()
    assert _run(portfile, manifest, archive_dir) == 1
    assert missing.name in capsys.readouterr().err


def test_check_passes_after_rewrite(tmp_path):
    # --check exits 0 once the hashes are written, and writes nothing.
    portfile, manifest, archive_dir, _ = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    written = portfile.read_bytes()
    assert _run(portfile, manifest, archive_dir, "--check") == 0
    assert portfile.read_bytes() == written


def test_check_fails_on_stale_hash(tmp_path, capsys):
    # --check exits 1 and names the target whose archive changed; writes nothing.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    written = portfile.read_bytes()
    stale = _targets()[1].name
    archives[stale].write_bytes(b"rebuilt archive")
    assert _run(portfile, manifest, archive_dir, "--check") == 1
    assert stale in capsys.readouterr().err
    assert portfile.read_bytes() == written


def _jobs():
    """Return the parsed jobs mapping of the real release workflow."""
    return yaml.safe_load(WORKFLOW.read_text(encoding="utf-8"))["jobs"]


def _step_index(steps, predicate, what):
    """Return the index of the one step matching ``predicate``."""
    hits = [i for i, step in enumerate(steps) if predicate(step)]
    assert len(hits) == 1, f"expected exactly one {what} step, found {len(hits)}"
    return hits[0]


def test_portfile_targets_match_build_ffi_matrix():
    # The portfile's targets are exactly the archives build-ffi produces.
    matrix = _jobs()["build-ffi"]["strategy"]["matrix"]["include"]
    assert {t.name for t in _targets()} == {entry["target"] for entry in matrix}


def test_prepare_release_needs_build_ffi():
    # prepare-release waits for the FFI archives; build-ffi no longer waits for it.
    jobs = _jobs()
    assert "build-ffi" in jobs["prepare-release"]["needs"]
    assert "prepare-release" not in (jobs["build-ffi"].get("needs") or [])


def test_hashes_written_before_commit_and_tag():
    # The hash update runs before the commit that stages the portfile, before tagging.
    steps = _jobs()["prepare-release"]["steps"]
    update = _step_index(
        steps, lambda s: "update_vcpkg_hashes.py" in s.get("run", ""), "hash update"
    )
    commit = _step_index(
        steps,
        lambda s: (
            "git add" in s.get("run", "")
            and "packages/cpp/portfile.cmake" in s.get("run", "")
        ),
        "portfile commit",
    )
    tag = _step_index(steps, lambda s: s.get("name") == "Create tags", "tag")
    assert update < commit < tag


def test_prepare_release_downloads_ffi_artifacts():
    # The hash update reads the merged build-ffi artifacts from ffi-staging.
    steps = _jobs()["prepare-release"]["steps"]
    download = _step_index(
        steps,
        lambda s: (
            s.get("uses", "").startswith("actions/download-artifact@")
            and (s.get("with") or {}).get("pattern") == "ffi-*"
        ),
        "ffi download",
    )
    update = _step_index(
        steps, lambda s: "update_vcpkg_hashes.py" in s.get("run", ""), "hash update"
    )
    assert steps[download]["with"]["path"] == "ffi-staging"
    assert steps[download]["with"]["merge-multiple"] is True
    assert "ffi-staging" in steps[update]["run"]
    assert download < update


def test_ffi_consumers_need_prepare_release():
    # publish-ffi and pack-nuget get the tag ordering from prepare-release directly.
    jobs = _jobs()
    assert "prepare-release" in jobs["publish-ffi"]["needs"]
    assert "prepare-release" in jobs["pack-nuget"]["needs"]


def test_publish_ffi_never_overwrites_archives():
    # A later ffi re-trigger cannot replace archives whose hashes are committed.
    steps = _jobs()["publish-ffi"]["steps"]
    release = _step_index(
        steps,
        lambda s: s.get("uses", "").startswith("softprops/action-gh-release@"),
        "gh-release",
    )
    assert steps[release]["with"]["overwrite_files"] is False
