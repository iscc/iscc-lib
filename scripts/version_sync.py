"""Sync all version references with the workspace version from root Cargo.toml.

Reads the canonical version from `[workspace.package] version` in the root `Cargo.toml` and updates
all files that contain version references. Cargo workspace members inherit the version automatically;
this script handles everything else.

Synced targets:
- `pyproject.toml` — root project version (dev workspace)
- `crates/iscc-napi/package.json` — npm package version
- `crates/iscc-jni/java/pom.xml` — Maven artifact version
- `mise.toml` — default `--version` flag for test_install task
- `scripts/test_install.py` — fallback version for registry checks
- Maven/Gradle version snippets in docs and READMEs

Usage:
    uv run scripts/version_sync.py           # Sync all targets
    uv run scripts/version_sync.py --check   # Validate without modifying (exit 1 if mismatch)
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CARGO_TOML = ROOT / "Cargo.toml"

VERSION_RE = re.compile(r'^version\s*=\s*"(.+?)"', re.MULTILINE)

# Maven version pattern: <groupId>io.iscc</groupId> ... <artifactId>iscc-lib</artifactId> ... <version>X.Y.Z</version>
MAVEN_DEP_RE = re.compile(
    r"(<groupId>io\.iscc</groupId>\s*<artifactId>iscc-lib</artifactId>\s*<version>)"
    r"[^<]+"
    r"(</version>)",
    re.DOTALL,
)

# Gradle version pattern: 'io.iscc:iscc-lib:X.Y.Z' or "io.iscc:iscc-lib:X.Y.Z"
GRADLE_DEP_RE = re.compile(
    r"(io\.iscc:iscc-lib:)\d+\.\d+\.\d+",
)


def read_workspace_version():
    """Read the workspace version from root Cargo.toml."""
    text = CARGO_TOML.read_text(encoding="utf-8")
    match = VERSION_RE.search(text)
    if not match:
        print("Error: could not find version in Cargo.toml", file=sys.stderr)
        sys.exit(1)
    return match.group(1)


# --- Sync target definitions ---
# Each target: (file_path, get_version_fn, sync_fn)
# get_version_fn(text) -> version string found in file
# sync_fn(text, version) -> updated text


def _get_pyproject_version(text):
    """Extract version from root pyproject.toml."""
    m = re.search(r'^version\s*=\s*"(.+?)"', text, re.MULTILINE)
    return m.group(1) if m else ""


def _sync_pyproject(text, version):
    """Update version in root pyproject.toml."""
    return re.sub(
        r'^(version\s*=\s*")(.+?)(")',
        rf"\g<1>{version}\3",
        text,
        count=1,
        flags=re.MULTILINE,
    )


def _get_package_json_version(text):
    """Extract version from package.json."""
    data = json.loads(text)
    return data.get("version", "")


def _sync_package_json(text, version):
    """Update version in package.json."""
    data = json.loads(text)
    data["version"] = version
    return json.dumps(data, indent=2) + "\n"


def _get_pom_version(text):
    """Extract project version from pom.xml."""
    m = re.search(
        r"<groupId>io\.iscc</groupId>\s*<artifactId>iscc-lib</artifactId>\s*<version>(.+?)</version>",
        text,
        re.DOTALL,
    )
    return m.group(1) if m else ""


def _sync_pom(text, version):
    """Update project version in pom.xml."""
    return MAVEN_DEP_RE.sub(rf"\g<1>{version}\2", text, count=1)


def _get_mise_version(text):
    """Extract default --version flag from mise.toml."""
    m = re.search(r"--version\s+(\d+\.\d+\.\d+)", text)
    return m.group(1) if m else ""


def _sync_mise(text, version):
    """Update default --version flag in mise.toml."""
    return re.sub(r"(--version\s+)\d+\.\d+\.\d+", rf"\g<1>{version}", text)


def _get_test_install_version(text):
    """Extract fallback version from test_install.py."""
    m = re.search(r'version\s+or\s+"(\d+\.\d+\.\d+)"', text)
    return m.group(1) if m else ""


def _sync_test_install(text, version):
    """Update fallback and docstring versions in test_install.py."""
    text = re.sub(r'(version\s+or\s+")\d+\.\d+\.\d+(")', rf"\g<1>{version}\2", text)
    text = re.sub(r"(--version\s+)\d+\.\d+\.\d+", rf"\g<1>{version}", text)
    return text


def _get_maven_doc_version(text):
    """Extract Maven dependency version from a doc/README file."""
    m = MAVEN_DEP_RE.search(text)
    return m.group(0).split("<version>")[1].split("</version>")[0] if m else ""


def _sync_maven_doc(text, version):
    """Update Maven dependency versions in a doc/README file."""
    text = MAVEN_DEP_RE.sub(rf"\g<1>{version}\2", text)
    text = GRADLE_DEP_RE.sub(rf"\g<1>{version}", text)
    return text


# Registry of all sync targets: (relative_path, get_fn, sync_fn)
TARGETS = [
    ("pyproject.toml", _get_pyproject_version, _sync_pyproject),
    ("crates/iscc-napi/package.json", _get_package_json_version, _sync_package_json),
    ("crates/iscc-jni/java/pom.xml", _get_pom_version, _sync_pom),
    ("mise.toml", _get_mise_version, _sync_mise),
    ("scripts/test_install.py", _get_test_install_version, _sync_test_install),
    ("README.md", _get_maven_doc_version, _sync_maven_doc),
    ("crates/iscc-jni/README.md", _get_maven_doc_version, _sync_maven_doc),
    ("docs/howto/java.md", _get_maven_doc_version, _sync_maven_doc),
    ("docs/java-api.md", _get_maven_doc_version, _sync_maven_doc),
]


def check_mode(workspace_version):
    """Validate that all targets match the workspace version. Return True if all match."""
    all_match = True
    for rel_path, get_fn, _ in TARGETS:
        filepath = ROOT / rel_path
        if not filepath.exists():
            continue
        text = filepath.read_text(encoding="utf-8")
        found = get_fn(text)
        if not found:
            continue
        if found != workspace_version:
            print(f"MISMATCH: {rel_path} has {found!r}, expected {workspace_version!r}")
            all_match = False
        else:
            print(f"OK: {rel_path} = {found}")
    return all_match


def sync_mode(workspace_version):
    """Update all targets to match the workspace version."""
    for rel_path, get_fn, sync_fn in TARGETS:
        filepath = ROOT / rel_path
        if not filepath.exists():
            continue
        text = filepath.read_text(encoding="utf-8")
        found = get_fn(text)
        if not found:
            continue
        if found == workspace_version:
            print(f"OK: {rel_path} = {found}")
            continue
        new_text = sync_fn(text, workspace_version)
        filepath.write_text(new_text, encoding="utf-8")
        print(f"Synced {rel_path}: {found} -> {workspace_version}")
    print("Version sync complete.")


def main():
    """Entry point: sync or check manifest versions against workspace version."""
    workspace_version = read_workspace_version()
    print(f"Workspace version: {workspace_version}")

    if "--check" in sys.argv:
        if not check_mode(workspace_version):
            sys.exit(1)
    else:
        sync_mode(workspace_version)


if __name__ == "__main__":
    main()
