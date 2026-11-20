"""Tests for the Ruby source-gem staging script ``scripts/build_ruby_source_gem.py``.

The pure helpers (manifest rendering, version checks, crate extraction) run on synthetic
input shaped like ``cargo metadata`` output, so no Rust toolchain is needed. Repository
anchors pin the wiring around the script: the gemspec globs the script's carried-core
directory and depends on ``rb_sys``, ``ci.yml`` and ``release.yml`` both build the source
gem with the script (never a bare ``gem build``), the CI install runs on a Ruby series with
no precompiled gem, and ``Gemfile.lock`` still pins the cross-gem ``rb_sys`` on a line the
cross-gem action can grep. The end-to-end build and clean install run in CI, not here:
``cargo package`` needs the crates.io index. No mocks.
"""

import importlib.util
import io
import re
import tarfile
from pathlib import Path

import pytest
import yaml

# Load scripts/build_ruby_source_gem.py by path: a repo script, not a package.
_REPO_ROOT = Path(__file__).resolve().parents[1]
_SCRIPT_PATH = _REPO_ROOT / "scripts" / "build_ruby_source_gem.py"
_spec = importlib.util.spec_from_file_location("build_ruby_source_gem", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
gem = importlib.util.module_from_spec(_spec)
_loader.exec_module(gem)

WORKFLOWS = _REPO_ROOT / ".github" / "workflows"
CI_WORKFLOW = WORKFLOWS / "ci.yml"
RELEASE_WORKFLOW = WORKFLOWS / "release.yml"
GEM_DIR = _REPO_ROOT / "crates" / "iscc-rb"
SCRIPT_CALL = "scripts/build_ruby_source_gem.py"
CROSS_GEM_ACTION = "oxidize-rb/actions/cross-gem"
BARE_GEM_BUILD = re.compile(r"\bgem\s+build\b")


def rb_package(dependencies=None):
    """Return a synthetic ``cargo metadata`` entry for ``iscc-rb``."""
    if dependencies is None:
        dependencies = [
            {
                "name": "iscc-lib",
                "kind": None,
                "req": "*",
                "features": [],
                "uses_default_features": True,
            },
            {
                "name": "magnus",
                "kind": None,
                "req": "^0.8",
                "features": ["rb-sys"],
                "uses_default_features": True,
            },
        ]
    return {
        "name": "iscc-rb",
        "version": "1.2.3",
        "edition": "2024",
        "rust_version": "1.85",
        "dependencies": dependencies,
    }


def workflow_runs(workflow):
    """Yield ``(job_id, step)`` for every step of a parsed workflow that has a ``run:``."""
    for job_id, job in (workflow.get("jobs") or {}).items():
        for step in job.get("steps") or []:
            if "run" in step:
                yield job_id, step


def load_workflow(path):
    """Parse a workflow YAML file."""
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def script_jobs(workflow):
    """Return the ids of jobs with a ``run:`` step that invokes the staging script."""
    return {
        job_id for job_id, step in workflow_runs(workflow) if SCRIPT_CALL in step["run"]
    }


def bare_gem_builds(workflow):
    """Return the ``run:`` strings of a parsed workflow that call ``gem build`` directly."""
    return [
        step["run"]
        for _, step in workflow_runs(workflow)
        if BARE_GEM_BUILD.search(step["run"])
    ]


def test_render_manifest_is_standalone():
    """The rendered manifest pins every inherited key and never points outside the gem."""
    manifest = gem.render_manifest(rb_package())
    assert 'version = "1.2.3"' in manifest
    assert 'edition = "2024"' in manifest
    assert 'rust-version = "1.85"' in manifest
    assert 'iscc-lib = { path = "core" }' in manifest
    assert 'magnus = { version = "^0.8", features = ["rb-sys"] }' in manifest
    assert manifest.rstrip().endswith("[workspace]")
    assert not re.search(r"workspace\s*=\s*true", manifest)
    assert "../" not in manifest


def test_render_manifest_keeps_extension_names():
    """``extconf.rb`` builds ``iscc_lib/iscc_rb``: package, lib name and crate type stay put."""
    manifest = gem.render_manifest(rb_package())
    assert 'name = "iscc-rb"' in manifest
    assert 'name = "iscc_rb"' in manifest
    assert 'crate-type = ["cdylib"]' in manifest


def test_render_manifest_rejects_unknown_dependency():
    """A dependency the renderer does not handle fails closed instead of being dropped."""
    deps = rb_package()["dependencies"] + [
        {
            "name": "serde",
            "kind": None,
            "req": "1",
            "features": [],
            "uses_default_features": True,
        }
    ]
    with pytest.raises(SystemExit, match="serde"):
        gem.render_manifest(rb_package(deps))


def test_render_manifest_ignores_dev_dependencies():
    """Dev dependencies are not part of the extension build and do not trip the guard."""
    deps = rb_package()["dependencies"] + [
        {
            "name": "criterion",
            "kind": "dev",
            "req": "0.8",
            "features": [],
            "uses_default_features": True,
        }
    ]
    assert "criterion" not in gem.render_manifest(rb_package(deps))


def test_render_registry_dependency_without_default_features():
    """``default-features = false`` survives the rendering."""
    dep = {"name": "x", "req": "^1", "features": [], "uses_default_features": False}
    assert (
        gem.render_registry_dependency(dep)
        == '{ version = "^1", default-features = false }'
    )


def test_ruby_gem_version_matches_workspace():
    """The real ``version.rb`` carries the workspace version from the root ``Cargo.toml``."""
    cargo = (_REPO_ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(
        r'^\[workspace\.package\]\nversion = "([^"]+)"', cargo, re.MULTILINE
    )
    assert match is not None
    version_rb = (GEM_DIR / "lib" / "iscc_lib" / "version.rb").read_text(
        encoding="utf-8"
    )
    assert gem.ruby_gem_version(version_rb) == match.group(1)


def test_check_versions_rejects_mismatch():
    """A gem version that differs from the workspace version stops the build."""
    gem.check_versions("1.2.3", "1.2.3")
    with pytest.raises(SystemExit, match="version mismatch"):
        gem.check_versions("1.2.3", "1.2.4")


def test_ruby_gem_version_requires_constant():
    """A ``version.rb`` without a ``VERSION`` constant stops the build."""
    with pytest.raises(SystemExit, match="VERSION"):
        gem.ruby_gem_version("module IsccLib; end\n")


def test_crate_member_path():
    """Members lose their top-level directory; the top dir and ``Cargo.toml.orig`` are skipped."""
    assert str(gem.crate_member_path("iscc-lib-1.2.3/src/lib.rs")) == "src/lib.rs"
    assert gem.crate_member_path("iscc-lib-1.2.3") is None
    assert gem.crate_member_path("iscc-lib-1.2.3/Cargo.toml.orig") is None
    with pytest.raises(SystemExit, match="unsafe"):
        gem.crate_member_path("iscc-lib-1.2.3/../evil")
    with pytest.raises(SystemExit, match="unsafe"):
        gem.crate_member_path("/etc/passwd")


def test_extract_crate(tmp_path):
    """A ``.crate`` archive extracts flattened, with ``Cargo.toml.orig`` dropped."""
    crate_file = tmp_path / "iscc-lib-1.2.3.crate"
    members = {
        "iscc-lib-1.2.3/Cargo.toml": b"[package]\n",
        "iscc-lib-1.2.3/Cargo.toml.orig": b"version.workspace = true\n",
        "iscc-lib-1.2.3/tests/data.json": b"{}",
    }
    with tarfile.open(crate_file, "w:gz") as archive:
        for name, data in members.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            archive.addfile(info, io.BytesIO(data))
    dest = tmp_path / "core"
    gem.extract_crate(crate_file, dest)
    files = sorted(
        p.relative_to(dest).as_posix() for p in dest.rglob("*") if p.is_file()
    )
    assert files == ["Cargo.toml", "tests/data.json"]


def test_core_dir_is_not_an_ignored_directory():
    """The carried core never lands in ``vendor/`` or ``tmp/`` (gitignored Bundler/rake dirs)."""
    ignored = {
        line.strip().rstrip("/")
        for line in (GEM_DIR / ".gitignore").read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.startswith(("#", "!"))
    }
    assert gem.CORE_DIR not in ignored | {"vendor", "tmp"}
    assert not (GEM_DIR / gem.CORE_DIR).exists()


def test_gemspec_carries_core_and_depends_on_rb_sys():
    """The gemspec globs the script's core directory and declares a loose ``rb_sys`` range."""
    gemspec = (GEM_DIR / gem.GEMSPEC).read_text(encoding="utf-8")
    assert f'Dir["{gem.CORE_DIR}/**/*"]' in gemspec
    assert '"Cargo.lock"' in gemspec
    match = re.search(r'add_dependency "rb_sys", "([^"]+)"', gemspec)
    assert match is not None
    assert match.group(1).startswith("~>")


def test_gemfile_lock_pins_cross_gem_rb_sys():
    """``cross-gem`` greps ``^    rb_sys (`` in ``Gemfile.lock``: it must name the Gemfile pin."""
    gemfile = (GEM_DIR / "Gemfile").read_text(encoding="utf-8")
    pin = re.search(r'^gem "rb_sys", "([^"]+)"', gemfile, re.MULTILINE)
    assert pin is not None
    lock = (GEM_DIR / "Gemfile.lock").read_text(encoding="utf-8")
    first = re.search(r"^    rb_sys \(([^)]+)\)", lock, re.MULTILINE)
    assert first is not None
    assert first.group(1) == pin.group(1)


@pytest.mark.parametrize("path", [CI_WORKFLOW, RELEASE_WORKFLOW])
def test_workflow_builds_source_gem_with_script(path):
    """Both workflows build the source gem with the staging script."""
    assert script_jobs(load_workflow(path))


def test_no_workflow_runs_bare_gem_build():
    """No workflow packs the gemspec directly: that gem cannot build outside the workspace."""
    offenders = {
        p.name: bare_gem_builds(load_workflow(p))
        for p in sorted(WORKFLOWS.glob("*.yml"))
    }
    assert {name: runs for name, runs in offenders.items() if runs} == {}


def test_bare_gem_build_detector_fires():
    """The detector flags the ``publish-rubygems`` step this script replaced."""
    workflow = {
        "jobs": {
            "publish-rubygems": {
                "steps": [
                    {"name": "Build source gem", "run": "gem build iscc-lib.gemspec"}
                ]
            }
        }
    }
    assert bare_gem_builds(workflow) == ["gem build iscc-lib.gemspec"]


def test_ci_source_gem_install_uses_ruby_without_precompiled_gem():
    """The CI install runs on a Ruby series outside the cross-gem list, in an empty gem home."""
    release = load_workflow(RELEASE_WORKFLOW)
    cross_gem = next(
        step
        for job in release["jobs"].values()
        for step in job.get("steps") or []
        if str(step.get("uses", "")).startswith(CROSS_GEM_ACTION)
    )
    precompiled = {
        s.strip() for s in str(cross_gem["with"]["ruby-versions"]).split(",")
    }
    ci = load_workflow(CI_WORKFLOW)
    (job_id,) = script_jobs(ci)
    steps = ci["jobs"][job_id]["steps"]
    rubies = [
        str(s["with"]["ruby-version"])
        for s in steps
        if str(s.get("uses", "")).startswith("ruby/setup-ruby")
    ]
    assert rubies
    assert not set(rubies) & precompiled
    install = next(s for s in steps if "gem install" in s.get("run", ""))
    assert install["env"]["GEM_HOME"] == install["env"]["GEM_PATH"]
    assert "runner.temp" in install["working-directory"]
    assert "conformance_selftest" in install["run"]


@pytest.mark.parametrize(
    "doc", [_REPO_ROOT / "docs" / "howto" / "ruby.md", GEM_DIR / "README.md"]
)
def test_ruby_docs_name_source_build_prerequisites(doc):
    """Both Ruby docs name the prerequisites the CI source-gem install provides."""
    text = doc.read_text(encoding="utf-8")
    for prerequisite in (
        "Rust toolchain",
        "libclang-dev",
        "Ruby development headers",
        "network access",
    ):
        assert prerequisite in re.sub(r"\s+", " ", text), prerequisite
