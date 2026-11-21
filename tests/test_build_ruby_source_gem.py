"""Tests for the Ruby source-gem staging script ``scripts/build_ruby_source_gem.py``.

The pure helpers (manifest rendering, version checks, crate extraction) run on synthetic
input shaped like ``cargo metadata`` output, so no Rust toolchain is needed; the release
profile helper runs on the real root ``Cargo.toml`` and on fixture text. Repository
anchors pin the wiring around the script: the gemspec globs the script's carried-core
directory and declares ``rb_sys`` only in a tree that carries it (the staged source gem),
``ci.yml`` and ``release.yml`` both build the source gem with the script (never a bare
``gem build``), the CI install takes the built source-gem file in an empty gem home, and the
cross-gem action's own ``rb_sys`` lookup on ``Gemfile.lock`` finds the pinned version. The
end-to-end build and clean install run in CI, not here: ``cargo package`` needs the
crates.io index. No mocks.
"""

import importlib.util
import io
import re
import shutil
import subprocess
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

ROOT_CARGO_TOML = _REPO_ROOT / "Cargo.toml"
PROFILE_KEYS = "lto = true\nstrip = true\n"
WORKFLOWS = _REPO_ROOT / ".github" / "workflows"
CI_WORKFLOW = WORKFLOWS / "ci.yml"
RELEASE_WORKFLOW = WORKFLOWS / "release.yml"
GEM_DIR = _REPO_ROOT / "crates" / "iscc-rb"
SCRIPT_CALL = "scripts/build_ruby_source_gem.py"
CROSS_GEM_ACTION = "oxidize-rb/actions/cross-gem"
BARE_GEM_BUILD = re.compile(r"\bgem\s+build\b")
SEMVER = re.compile(r"[0-9]+\.[0-9]+\.[0-9]+")
RUNTIME_DEPS_RB = (
    "Gem::Specification.load(ARGV[0]).runtime_dependencies"
    '.each { |d| puts "#{d.name} #{d.requirement}" }'
)
LOCK_WITH_PATH_DEPENDENCY = """PATH
  remote: .
  specs:
    iscc-lib (0.7.0)
      rb_sys (~> 0.9)

GEM
  remote: https://rubygems.org/
  specs:
    rb_sys (0.9.123)
      rake-compiler-dock (= 1.10.0)
"""


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


def render(package):
    """Render the standalone manifest of ``package`` with the fixture release profile."""
    return gem.render_manifest(package, PROFILE_KEYS)


def root_cargo_toml():
    """Return the text of the real root ``Cargo.toml``."""
    return ROOT_CARGO_TOML.read_text(encoding="utf-8")


def scan_release_keys(text):
    """Return the key lines of ``[profile.release]`` by an independent line scan.

    Starts after the exact header line and stops at the next line that begins with
    ``[``; blank and comment lines are skipped.
    """
    lines = text.splitlines()
    start = lines.index("[profile.release]") + 1
    keys = []
    for line in lines[start:]:
        if line.startswith("["):
            break
        if line.strip() and not line.lstrip().startswith("#"):
            keys.append(line.strip())
    return keys


def manifest_release_table(manifest):
    """Return the lines of the rendered manifest's ``[profile.release]`` table."""
    lines = manifest.splitlines()
    start = lines.index("[profile.release]") + 1
    table = []
    for line in lines[start:]:
        if line.startswith("["):
            break
        table.append(line.strip())
    return table


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


def cross_gem_rb_sys_version(lock_text):
    """Return the ``rb_sys`` version the ``cross-gem@v1`` action reads from lock text.

    Copies the action's ``grep rb_sys Gemfile.lock | head -n 1 | grep -oE
    '[0-9]+\\.[0-9]+\\.[0-9]+'``: the first line containing ``rb_sys`` anywhere, then the
    first ``X.Y.Z`` in it. None when that line carries no version (the action then falls
    back to the latest remote ``rb_sys``).
    """
    first = next((line for line in lock_text.splitlines() if "rb_sys" in line), None)
    if first is None:
        return None
    match = SEMVER.search(first)
    return match.group(0) if match else None


def runtime_dependencies(gemspec, cwd):
    """Return ``"name requirement"`` lines of a gemspec's runtime dependencies via ``ruby``."""
    ruby = shutil.which("ruby")
    assert ruby is not None, "ruby is required to evaluate the gemspec"
    result = subprocess.run(
        [ruby, "-e", RUNTIME_DEPS_RB, str(gemspec)],
        cwd=cwd,
        check=True,
        stdout=subprocess.PIPE,
        text=True,
    )
    return result.stdout.splitlines()


def cross_gem_step():
    """Return the ``oxidize-rb/actions/cross-gem`` step of ``release.yml``."""
    release = load_workflow(RELEASE_WORKFLOW)
    return next(
        step
        for job in release["jobs"].values()
        for step in job.get("steps") or []
        if str(step.get("uses", "")).startswith(CROSS_GEM_ACTION)
    )


def test_render_manifest_is_standalone():
    """The rendered manifest pins every inherited key and never points outside the gem."""
    manifest = render(rb_package())
    assert 'version = "1.2.3"' in manifest
    assert 'edition = "2024"' in manifest
    assert 'rust-version = "1.85"' in manifest
    assert 'iscc-lib = { path = "core" }' in manifest
    assert 'magnus = { version = "^0.8", features = ["rb-sys"] }' in manifest
    assert manifest.rstrip().endswith("[workspace]")
    assert not re.search(r"workspace\s*=\s*true", manifest)
    assert "../" not in manifest


def test_release_profile_copies_root_table():
    """Every key line of the root ``[profile.release]`` lands in the staged table."""
    text = root_cargo_toml()
    keys = scan_release_keys(text)
    assert keys
    table = manifest_release_table(
        gem.render_manifest(rb_package(), gem.release_profile(text))
    )
    for key in keys:
        assert key in table, key


def test_release_profile_stops_at_bench_profile():
    """The slice ends at ``[profile.bench]``: its ``strip``/``debug`` keys never leak."""
    text = root_cargo_toml()
    assert "[profile.bench]" in text.splitlines()
    manifest = gem.render_manifest(rb_package(), gem.release_profile(text))
    assert "strip = false" not in manifest
    assert "debug = true" not in manifest


def test_release_profile_requires_table():
    """Cargo.toml text without a ``[profile.release]`` table stops the build."""
    text = '[workspace]\nmembers = ["a"]\n\n[profile.bench]\ndebug = true\n'
    with pytest.raises(SystemExit, match="expected one"):
        gem.release_profile(text)


def test_release_profile_rejects_sub_table():
    """A ``[profile.release.<x>]`` sub-table stops the build instead of being dropped."""
    text = (
        "[profile.release]\nlto = true\n\n"
        '[profile.release.package."iscc-lib"]\nopt-level = 3\n'
    )
    with pytest.raises(SystemExit, match="sub-tables"):
        gem.release_profile(text)


def test_release_profile_requires_keys():
    """An empty ``[profile.release]`` table stops the build."""
    text = "[profile.release]\n# nothing\n\n[profile.bench]\ndebug = true\n"
    with pytest.raises(SystemExit, match="no keys"):
        gem.release_profile(text)


def test_release_profile_is_derived():
    """A key removed from the root text is absent from the manifest: nothing is retyped."""
    text = root_cargo_toml()
    keys = scan_release_keys(text)
    removed = keys[-1]
    trimmed = "\n".join(line for line in text.splitlines() if line.strip() != removed)
    table = manifest_release_table(
        gem.render_manifest(rb_package(), gem.release_profile(trimmed))
    )
    assert removed not in table
    assert set(keys) - {removed} <= set(table)


def test_render_manifest_keeps_extension_names():
    """``extconf.rb`` builds ``iscc_lib/iscc_rb``: package, lib name and crate type stay put."""
    manifest = render(rb_package())
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
        render(rb_package(deps))


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
    assert "criterion" not in render(rb_package(deps))


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


def test_gemspec_carries_core_and_lock():
    """The gemspec globs the script's core directory and lists the shipped ``Cargo.lock``."""
    gemspec = (GEM_DIR / gem.GEMSPEC).read_text(encoding="utf-8")
    assert f'Dir["{gem.CORE_DIR}/**/*"]' in gemspec
    assert '"Cargo.lock"' in gemspec


def test_gemspec_has_no_runtime_dependency_in_repository(tmp_path):
    """In ``crates/iscc-rb`` (platform gems) the gemspec declares no runtime dependency.

    Loaded from a cwd that has a ``core/`` directory: the check must follow the gemspec's
    own directory, not the cwd.
    """
    (tmp_path / gem.CORE_DIR).mkdir()
    assert runtime_dependencies(GEM_DIR / gem.GEMSPEC, tmp_path) == []


def test_gemspec_depends_on_rb_sys_in_staged_tree(tmp_path):
    """A tree carrying ``core/`` (the staged source gem) declares exactly ``rb_sys ~> …``."""
    shutil.copy2(GEM_DIR / gem.GEMSPEC, tmp_path / gem.GEMSPEC)
    version_rb = tmp_path / "lib" / "iscc_lib" / "version.rb"
    version_rb.parent.mkdir(parents=True)
    shutil.copy2(GEM_DIR / "lib" / "iscc_lib" / "version.rb", version_rb)
    (tmp_path / gem.CORE_DIR).mkdir()
    deps = runtime_dependencies(tmp_path / gem.GEMSPEC, GEM_DIR)
    assert len(deps) == 1
    name, requirement = deps[0].split(" ", 1)
    assert name == "rb_sys"
    assert requirement.startswith("~>")


def test_cross_gem_lookup_reads_pinned_rb_sys():
    """The cross-gem lookup on the real ``Gemfile.lock`` equals the Gemfile pin and ``tag:``."""
    gemfile = (GEM_DIR / "Gemfile").read_text(encoding="utf-8")
    pin = re.search(r'^gem "rb_sys", "([^"]+)"', gemfile, re.MULTILINE)
    assert pin is not None
    lock = (GEM_DIR / "Gemfile.lock").read_text(encoding="utf-8")
    assert cross_gem_rb_sys_version(lock) == pin.group(1)
    assert str(cross_gem_step()["with"]["tag"]) == pin.group(1)


def test_cross_gem_lookup_fires_on_path_dependency():
    """A version-less ``rb_sys`` line under ``PATH`` specs comes first: the lookup finds none."""
    assert cross_gem_rb_sys_version(LOCK_WITH_PATH_DEPENDENCY) is None


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


def test_ci_source_gem_install_takes_built_source_gem_file():
    """The CI install takes the built source-gem file, never a registry lookup, in an empty gem home.

    Installing the file is what makes it a source install on any Ruby: ``gem install iscc-lib``
    by name would pick a precompiled platform gem on a Ruby the cross-gem step covers.
    """
    ci = load_workflow(CI_WORKFLOW)
    (job_id,) = script_jobs(ci)
    steps = ci["jobs"][job_id]["steps"]
    rubies = [
        str(s["with"]["ruby-version"])
        for s in steps
        if str(s.get("uses", "")).startswith("ruby/setup-ruby")
    ]
    assert rubies
    build = next(s for s in steps if SCRIPT_CALL in s.get("run", ""))
    out_dir = re.search(r"--out\s+(\S+)", build["run"].replace('"', "")).group(1)
    install = next(s for s in steps if "gem install" in s.get("run", ""))
    install_run = install["run"].replace('"', "")
    assert f"gem install {out_dir}/iscc-lib-*.gem" in install_run
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
