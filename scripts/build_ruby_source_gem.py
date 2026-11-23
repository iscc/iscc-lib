# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""Build the standalone Ruby source gem (platform ``ruby``) for ``iscc-lib``.

``crates/iscc-rb`` cannot be packed as-is: its ``Cargo.toml`` inherits ``version``,
``edition``, ``rust-version`` and ``magnus`` from the workspace and depends on the core
by ``path = "../iscc-lib"``, and neither the workspace root nor the core ships in the
gem. This script stages a gem tree that builds on its own:

- the files the gemspec lists, copied from ``crates/iscc-rb``
- a standalone ``Cargo.toml`` for ``iscc-rb``: concrete package keys and the ``magnus``
  requirement from ``cargo metadata``, the core as ``path = "core"``, and an empty
  ``[workspace]`` table so cargo never walks up into an enclosing workspace
- the workspace ``[profile.release]`` table, copied as key lines from the root
  ``Cargo.toml`` (never retyped), so a source install builds stripped, with LTO and
  ``panic = "abort"`` like the platform gems
- the core crate under ``core/``, extracted from ``cargo package -p iscc-lib`` (whose
  manifest cargo has already normalized), minus ``Cargo.toml.orig``
- the workspace ``Cargo.lock``, so an install resolves the crate versions CI tested

It then runs ``gem build`` in the staging tree and writes ``iscc-lib-<version>.gem`` to
``--out``. The staging tree lives in a temporary directory outside the repository.
Both ``.github/workflows/ci.yml`` (install check) and ``release.yml`` (publish) build
the source gem with this script.

Usage:
    uv run --script scripts/build_ruby_source_gem.py --out DIR
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path, PurePosixPath

REPO_ROOT = Path(__file__).resolve().parent.parent
GEM_DIR = REPO_ROOT / "crates" / "iscc-rb"
GEMSPEC = "iscc-lib.gemspec"
GEM_NAME = "iscc-lib"
RB_CRATE = "iscc-rb"
CORE_CRATE = "iscc-lib"
CORE_DIR = "core"
"""In-gem directory of the carried core crate; ``iscc-lib.gemspec`` globs ``core/**/*``.

Never ``vendor/`` or ``tmp/``: both are gitignored Bundler/rake directories in
``crates/iscc-rb``, and a gemspec glob over them would sweep junk into the platform gems
that ``cross-gem`` builds from the repository directory.
"""
DROPPED_CORE_FILES = frozenset({"Cargo.toml.orig"})
RELEASE_PROFILE = "profile.release"
TOML_HEADER = re.compile(r"^\s*\[\[?([^\[\]]+)\]\]?\s*(?:#.*)?$")
"""A TOML table or array-of-tables header line; group 1 is the table name."""


def run(cmd: list[str], cwd: Path) -> str:
    """Run a command in ``cwd``, fail on a non-zero exit and return its stdout.

    Only stdout is captured: stderr passes through to the caller's stderr, so a failing
    ``cargo package`` or ``gem build`` logs its reason in the job output.
    """
    # S603: every argv is built in this script from constants (cargo, ruby, gem), no shell
    return subprocess.run(  # noqa: S603
        cmd, cwd=cwd, check=True, stdout=subprocess.PIPE, text=True
    ).stdout


def find_package(metadata: dict, name: str) -> dict:
    """Return the ``cargo metadata`` package entry called ``name``."""
    for package in metadata["packages"]:
        if package["name"] == name:
            return package
    raise SystemExit(f"cargo metadata has no package {name!r}")


def find_dependency(package: dict, name: str) -> dict:
    """Return the normal (non-dev, non-build) dependency ``name`` of a metadata package."""
    for dep in package["dependencies"]:
        if dep["name"] == name and dep["kind"] is None:
            return dep
    raise SystemExit(f"package {package['name']!r} has no dependency {name!r}")


def toml_string(value: str) -> str:
    """Return ``value`` as a TOML basic string (JSON string escapes are valid TOML)."""
    return json.dumps(value)


def render_registry_dependency(dep: dict) -> str:
    """Render a registry dependency from ``cargo metadata`` as a TOML inline table."""
    fields = [f"version = {toml_string(dep['req'])}"]
    if dep["features"]:
        features = ", ".join(toml_string(feature) for feature in dep["features"])
        fields.append(f"features = [{features}]")
    if not dep["uses_default_features"]:
        fields.append("default-features = false")
    return "{ " + ", ".join(fields) + " }"


def header_name(line: str) -> str | None:
    """Return the table name of a TOML header line without spaces, or None for other lines."""
    match = TOML_HEADER.match(line)
    return None if match is None else re.sub(r"\s+", "", match.group(1))


def is_key_line(line: str) -> bool:
    """Return True for a line that is neither blank nor a comment."""
    stripped = line.strip()
    return bool(stripped) and not stripped.startswith("#")


def release_profile(cargo_toml: str) -> str:
    """Return the key lines of the ``[profile.release]`` table in ``cargo_toml`` text.

    A verbatim text slice (no ``tomllib``: CI's ``python-test`` matrix includes 3.10):
    the lines after the header up to the next header line, minus blanks and comments.
    Fails closed when the table is missing, duplicated or empty, and when any
    ``[profile.release.<x>]`` sub-table exists, which the slice would drop silently.
    """
    lines = cargo_toml.splitlines()
    names = [header_name(line) for line in lines]
    sub_tables = sorted({n for n in names if n and n.startswith(RELEASE_PROFILE + ".")})
    if sub_tables:
        raise SystemExit(f"release_profile: unhandled sub-tables {sub_tables}")
    starts = [i for i, name in enumerate(names) if name == RELEASE_PROFILE]
    if len(starts) != 1:
        raise SystemExit(f"release_profile: expected one [{RELEASE_PROFILE}] table")
    body = []
    for line, name in zip(lines[starts[0] + 1 :], names[starts[0] + 1 :]):
        if name is not None:
            break
        body.append(line)
    keys = [line.rstrip() for line in body if is_key_line(line)]
    if not keys:
        raise SystemExit(f"release_profile: [{RELEASE_PROFILE}] has no keys")
    return "\n".join(keys) + "\n"


def render_manifest(
    rb_package: dict, release_profile_keys: str, core_dir: str = CORE_DIR
) -> str:
    """Render the standalone ``Cargo.toml`` of ``iscc-rb`` for the source gem.

    Keeps the package and lib names the gem's ``extconf.rb`` builds (``iscc_rb``
    cdylib), pins the workspace-inherited keys to concrete values and points the core at
    the in-gem ``core_dir``. ``release_profile_keys`` (from ``release_profile``) becomes
    the ``[profile.release]`` table. Any normal dependency other than the core and ``magnus``
    fails closed, so a dependency added to ``crates/iscc-rb`` cannot silently go missing.
    """
    deps = {dep["name"] for dep in rb_package["dependencies"] if dep["kind"] is None}
    unexpected = deps - {CORE_CRATE, "magnus"}
    if unexpected:
        raise SystemExit(
            f"render_manifest: unhandled iscc-rb dependencies {sorted(unexpected)}"
        )
    magnus = render_registry_dependency(find_dependency(rb_package, "magnus"))
    return (
        "# Standalone manifest generated by scripts/build_ruby_source_gem.py\n"
        "[package]\n"
        f"name = {toml_string(rb_package['name'])}\n"
        f"version = {toml_string(rb_package['version'])}\n"
        f"edition = {toml_string(rb_package['edition'])}\n"
        f"rust-version = {toml_string(rb_package['rust_version'])}\n"
        "publish = false\n"
        "\n"
        "[lib]\n"
        'name = "iscc_rb"\n'
        'crate-type = ["cdylib"]\n'
        "\n"
        "[dependencies]\n"
        f"{CORE_CRATE} = {{ path = {toml_string(core_dir)} }}\n"
        f"magnus = {magnus}\n"
        "\n"
        "# Workspace release profile, copied from the root Cargo.toml\n"
        "[profile.release]\n"
        f"{release_profile_keys}"
        "\n"
        "# Empty workspace: cargo must never walk up into an enclosing workspace\n"
        "[workspace]\n"
    )


def ruby_gem_version(version_rb: str) -> str:
    """Return the ``VERSION = "X.Y.Z"`` string from ``lib/iscc_lib/version.rb`` text."""
    match = re.search(r'VERSION\s*=\s*"([^"]+)"', version_rb)
    if match is None:
        raise SystemExit("version.rb: no VERSION constant")
    return match.group(1)


def check_versions(workspace_version: str, gem_version: str) -> None:
    """Fail unless the Ruby gem version matches the workspace (Cargo) version."""
    if workspace_version != gem_version:
        raise SystemExit(
            f"version mismatch: workspace {workspace_version} vs version.rb {gem_version}"
            " (run `mise run version:sync`)"
        )


def crate_member_path(name: str) -> PurePosixPath | None:
    """Return a ``.crate`` member path without its top-level directory, or None to skip it.

    Skips the top-level directory itself and ``DROPPED_CORE_FILES``; rejects absolute
    paths and ``..`` segments.
    """
    path = PurePosixPath(name)
    if path.is_absolute() or ".." in path.parts:
        raise SystemExit(f"unsafe path in crate archive: {name!r}")
    relative = PurePosixPath(*path.parts[1:]) if len(path.parts) > 1 else None
    if relative is None or relative.name in DROPPED_CORE_FILES:
        return None
    return relative


def extract_crate(crate_file: Path, dest: Path) -> None:
    """Extract the regular files of a ``.crate`` archive into ``dest``, flattened one level."""
    with tarfile.open(crate_file, "r:gz") as archive:
        for member in archive.getmembers():
            relative = crate_member_path(member.name)
            if relative is None or not member.isfile():
                continue
            source = archive.extractfile(member)
            if source is None:
                continue
            target = dest.joinpath(*relative.parts)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read())


def gemspec_files() -> list[str]:
    """Return the file list ``iscc-lib.gemspec`` declares, evaluated in ``crates/iscc-rb``.

    One path per line, so a path containing a space stays whole.
    """
    script = f'puts Gem::Specification.load("{GEMSPEC}").files'
    files = run(["ruby", "-e", script], cwd=GEM_DIR).splitlines()
    if not files:
        raise SystemExit(f"{GEMSPEC} lists no files")
    return files


def package_core(target_dir: Path, version: str) -> Path:
    """Run ``cargo package`` for the core crate and return the path of its ``.crate``."""
    run(
        [
            "cargo",
            "package",
            "-p",
            CORE_CRATE,
            "--no-verify",
            "--allow-dirty",
            "--quiet",
        ],
        cwd=REPO_ROOT,
    )
    crate_file = target_dir / "package" / f"{CORE_CRATE}-{version}.crate"
    if not crate_file.is_file():
        raise SystemExit(f"cargo package wrote no {crate_file}")
    return crate_file


def stage_gem_tree(stage: Path, metadata: dict) -> str:
    """Populate ``stage`` with the standalone gem tree and return the gem version."""
    rb_package = find_package(metadata, RB_CRATE)
    version = rb_package["version"]
    check_versions(
        version, ruby_gem_version((GEM_DIR / "lib/iscc_lib/version.rb").read_text())
    )
    for name in [*gemspec_files(), GEMSPEC]:
        target = stage / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(GEM_DIR / name, target)
    workspace_root = Path(metadata["workspace_root"])
    profile = release_profile(
        (workspace_root / "Cargo.toml").read_text(encoding="utf-8")
    )
    manifest = render_manifest(rb_package, profile)
    (stage / "Cargo.toml").write_text(manifest, encoding="utf-8")
    shutil.copy2(workspace_root / "Cargo.lock", stage / "Cargo.lock")
    core_version = find_package(metadata, CORE_CRATE)["version"]
    crate_file = package_core(Path(metadata["target_directory"]), core_version)
    extract_crate(crate_file, stage / CORE_DIR)
    return version


def build_gem(stage: Path, out_dir: Path, version: str) -> Path:
    """Run ``gem build`` in ``stage`` and return the written ``.gem`` path in ``out_dir``."""
    gem_file = out_dir.resolve() / f"{GEM_NAME}-{version}.gem"
    run(["gem", "build", GEMSPEC, "--output", str(gem_file)], cwd=stage)
    return gem_file


def main(argv: list[str] | None = None) -> None:
    """Stage the standalone gem tree in a temporary directory and build the source gem."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--out", type=Path, required=True, help="directory for the .gem file"
    )
    args = parser.parse_args(argv)
    args.out.mkdir(parents=True, exist_ok=True)
    metadata = json.loads(
        run(["cargo", "metadata", "--format-version", "1", "--no-deps"], REPO_ROOT)
    )
    with tempfile.TemporaryDirectory(prefix="iscc-rb-source-gem-") as tmp:
        stage = Path(tmp)
        version = stage_gem_tree(stage, metadata)
        gem_file = build_gem(stage, args.out, version)
    print(gem_file)


if __name__ == "__main__":
    main()
