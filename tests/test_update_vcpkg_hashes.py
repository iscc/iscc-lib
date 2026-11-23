"""Tests for ``scripts/update_vcpkg_hashes.py`` and its wiring in the release workflow.

The script tests run against a copy of the real portfile, a ``vcpkg.json`` and a
``conandata.yml`` path in ``tmp_path``, with small fake archives — no network, and the
tracked files are never touched. The wiring tests read the real
``.github/workflows/release.yml``: the workflow is ``workflow_dispatch``-only, so these
tests are the gate that keeps ``prepare-release`` writing the hashes from the
``build-ffi`` archives into the commit it tags. The Conan recipe is read with ``ast``
(no ``conan`` import) to tie its targets and its ``download`` call to the same archives.
"""

import ast
import hashlib
import importlib.util
import json
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
PORTFILE = ROOT / "packages" / "cpp" / "portfile.cmake"
CONANDATA = ROOT / "packages" / "cpp" / "conandata.yml"
CONANFILE = ROOT / "packages" / "cpp" / "conanfile.py"
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

    Archives are named for ``version``; the manifest always says ``VERSION``. The
    ``conandata.yml`` path lives in ``tmp_path`` and does not exist until the script runs.
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


def _conandata(portfile):
    """Return the ``conandata.yml`` path that ``_run`` uses next to ``portfile``."""
    return portfile.parent / "conandata.yml"


def _run(portfile, manifest, archive_dir, *extra):
    """Run the script's ``main`` on the given files and return its exit code."""
    argv = [
        str(archive_dir),
        "--portfile",
        str(portfile),
        "--manifest",
        str(manifest),
        "--conandata",
        str(_conandata(portfile)),
    ]
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


def test_rewrite_writes_conan_sha256_per_target(tmp_path):
    # conandata.yml maps every portfile target to the sha256 of its archive.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    data = yaml.safe_load(_conandata(portfile).read_text(encoding="utf-8"))
    expected = {
        name: hashlib.sha256(path.read_bytes()).hexdigest()
        for name, path in archives.items()
    }
    assert data == {"sha256": expected}


def test_missing_archive_writes_nothing(tmp_path):
    # One missing archive fails the run and writes neither the portfile nor conandata.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    archives[_targets()[-1].name].unlink()
    assert _run(portfile, manifest, archive_dir) == 1
    assert portfile.read_bytes() == PORTFILE.read_bytes()
    assert not _conandata(portfile).exists()


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
    conan_written = _conandata(portfile).read_bytes()
    assert _run(portfile, manifest, archive_dir, "--check") == 0
    assert portfile.read_bytes() == written
    assert _conandata(portfile).read_bytes() == conan_written


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


def test_check_fails_on_stale_conan_hash(tmp_path, capsys):
    # A conandata.yml hash that differs (portfile still current) fails --check, named.
    portfile, manifest, archive_dir, archives = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    stale = _targets()[2].name
    good = hashlib.sha256(archives[stale].read_bytes()).hexdigest()
    conandata = _conandata(portfile)
    text = conandata.read_text(encoding="utf-8")
    altered = text.replace(good, ("0" if good[0] != "0" else "1") + good[1:])
    assert altered != text
    conandata.write_text(altered, encoding="utf-8")
    assert _run(portfile, manifest, archive_dir, "--check") == 1
    err = capsys.readouterr().err
    assert f"stale Conan SHA256 for {stale}" in err
    assert "SHA512" not in err
    assert conandata.read_text(encoding="utf-8") == altered


def test_check_fails_on_missing_conandata(tmp_path, capsys):
    # With current portfile hashes but no conandata.yml, --check fails and names targets.
    portfile, manifest, archive_dir, _ = _setup(tmp_path)
    assert _run(portfile, manifest, archive_dir) == 0
    _conandata(portfile).unlink()
    assert _run(portfile, manifest, archive_dir, "--check") == 1
    err = capsys.readouterr().err
    assert all(f"stale Conan SHA256 for {t.name}" in err for t in _targets())
    assert not _conandata(portfile).exists()


def test_committed_conandata_is_rendered_form():
    # The tracked conandata.yml is byte-identical to what the script renders from it.
    data = yaml.safe_load(CONANDATA.read_text(encoding="utf-8"))["sha256"]
    names = list(data)
    rendered = uvh.render_conandata(names, [data[n] for n in names])
    assert CONANDATA.read_bytes().decode("utf-8") == rendered


def _conanfile_tree():
    """Return the parsed AST of the real Conan recipe."""
    return ast.parse(CONANFILE.read_text(encoding="utf-8"))


def _conanfile_triples():
    """Return the target triples of the recipe's ``_target_triple`` mapping."""
    funcs = [
        node
        for node in ast.walk(_conanfile_tree())
        if isinstance(node, ast.FunctionDef) and node.name == "_target_triple"
    ]
    assert len(funcs) == 1
    dicts = [node for node in ast.walk(funcs[0]) if isinstance(node, ast.Dict)]
    assert len(dicts) == 1
    triples = [ast.literal_eval(value)[0] for value in dicts[0].values]
    assert len(triples) == len(set(triples))
    return set(triples)


def _matrix_targets():
    """Return the target triples of the release workflow's ``build-ffi`` matrix."""
    matrix = _jobs()["build-ffi"]["strategy"]["matrix"]["include"]
    return {entry["target"] for entry in matrix}


def test_conandata_targets_match_build_ffi_matrix():
    # conandata.yml pins a SHA256 for exactly the archives build-ffi produces.
    data = yaml.safe_load(CONANDATA.read_text(encoding="utf-8"))
    assert set(data) == {"sha256"}
    assert len(data["sha256"]) >= 5
    assert set(data["sha256"]) == _matrix_targets()
    assert all(len(h) == 64 and int(h, 16) >= 0 for h in data["sha256"].values())


def test_conanfile_targets_match_build_ffi_matrix():
    # The recipe's os/arch mapping covers exactly the archives build-ffi produces.
    assert _conanfile_triples() == _matrix_targets()


def test_conanfile_download_passes_sha256():
    # The recipe's one download() call verifies the archive against a sha256 keyword.
    calls = [
        node
        for node in ast.walk(_conanfile_tree())
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "download"
    ]
    assert len(calls) == 1
    keywords = {kw.arg: kw.value for kw in calls[0].keywords}
    assert "sha256" in keywords
    value = keywords["sha256"]
    assert not (isinstance(value, ast.Constant) and value.value is None)


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
    assert {t.name for t in _targets()} == _matrix_targets()


def test_prepare_release_needs_build_ffi():
    # prepare-release waits for the FFI archives; build-ffi no longer waits for it.
    jobs = _jobs()
    assert "build-ffi" in jobs["prepare-release"]["needs"]
    assert "prepare-release" not in (jobs["build-ffi"].get("needs") or [])


def _git_add_paths(step):
    """Return every path that the ``git add`` lines of a step's ``run`` script stage."""
    lines = step.get("run", "").splitlines()
    adds = [line.split()[2:] for line in lines if line.strip().startswith("git add ")]
    return {path for paths in adds for path in paths}


def test_hashes_written_before_commit_and_tag():
    # The hash update runs before the commit that stages portfile + conandata, before tag.
    steps = _jobs()["prepare-release"]["steps"]
    update = _step_index(
        steps, lambda s: "update_vcpkg_hashes.py" in s.get("run", ""), "hash update"
    )
    commit = _step_index(
        steps,
        lambda s: "packages/cpp/portfile.cmake" in _git_add_paths(s),
        "portfile commit",
    )
    staged = _git_add_paths(steps[commit])
    assert "packages/cpp/conandata.yml" in staged
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
