"""Sync non-Cargo manifest versions with the workspace version from root Cargo.toml.

Reads the canonical version from `[workspace.package] version` in the root `Cargo.toml` and updates:
- `crates/iscc-napi/package.json` — `"version"` field
- `crates/iscc-jni/java/pom.xml` — `<version>` element for the project artifact

Usage:
    uv run scripts/version_sync.py           # Sync all manifests
    uv run scripts/version_sync.py --check   # Validate without modifying (exit 1 if mismatch)
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CARGO_TOML = ROOT / "Cargo.toml"
PACKAGE_JSON = ROOT / "crates" / "iscc-napi" / "package.json"
POM_XML = ROOT / "crates" / "iscc-jni" / "java" / "pom.xml"

VERSION_RE = re.compile(r'^version\s*=\s*"(.+?)"', re.MULTILINE)
POM_VERSION_RE = re.compile(
    r"(<groupId>io\.iscc</groupId>\s*<artifactId>iscc-lib</artifactId>\s*<version>).+?(</version>)",
    re.DOTALL,
)


def read_workspace_version():
    """Read the workspace version from root Cargo.toml."""
    text = CARGO_TOML.read_text(encoding="utf-8")
    match = VERSION_RE.search(text)
    if not match:
        print("Error: could not find version in Cargo.toml", file=sys.stderr)
        sys.exit(1)
    return match.group(1)


def get_package_json_version():
    """Read the current version from package.json."""
    data = json.loads(PACKAGE_JSON.read_text(encoding="utf-8"))
    return data.get("version", "")


def sync_package_json(version):
    """Update the version field in package.json."""
    data = json.loads(PACKAGE_JSON.read_text(encoding="utf-8"))
    data["version"] = version
    PACKAGE_JSON.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def get_pom_xml_version():
    """Read the current project version from pom.xml."""
    text = POM_XML.read_text(encoding="utf-8")
    match = re.search(
        r"<groupId>io\.iscc</groupId>\s*<artifactId>iscc-lib</artifactId>\s*<version>(.+?)</version>",
        text,
        re.DOTALL,
    )
    return match.group(1) if match else ""


def sync_pom_xml(version):
    """Update the project version in pom.xml using regex replacement."""
    text = POM_XML.read_text(encoding="utf-8")
    new_text = POM_VERSION_RE.sub(rf"\g<1>{version}\2", text)
    POM_XML.write_text(new_text, encoding="utf-8")


def check_mode(workspace_version):
    """Validate that all manifests match the workspace version. Return True if all match."""
    all_match = True

    pkg_version = get_package_json_version()
    if pkg_version != workspace_version:
        print(
            f"MISMATCH: {PACKAGE_JSON.relative_to(ROOT)} has {pkg_version!r}, expected {workspace_version!r}"
        )
        all_match = False
    else:
        print(f"OK: {PACKAGE_JSON.relative_to(ROOT)} = {pkg_version}")

    pom_version = get_pom_xml_version()
    if pom_version != workspace_version:
        print(
            f"MISMATCH: {POM_XML.relative_to(ROOT)} has {pom_version!r}, expected {workspace_version!r}"
        )
        all_match = False
    else:
        print(f"OK: {POM_XML.relative_to(ROOT)} = {pom_version}")

    return all_match


def sync_mode(workspace_version):
    """Update all manifests to match the workspace version."""
    sync_package_json(workspace_version)
    print(f"Synced {PACKAGE_JSON.relative_to(ROOT)} to {workspace_version}")

    sync_pom_xml(workspace_version)
    print(f"Synced {POM_XML.relative_to(ROOT)} to {workspace_version}")

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
