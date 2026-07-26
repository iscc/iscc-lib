"""Parity check for the hand-wired documentation page lists.

Adding a page under `docs/` requires editing three unrelated lists by hand, and nothing
else checks they agree: the `nav` table in `zensical.toml`, `ORDERED_PAGES` in
`scripts/gen_llms_full.py`, and the link list in `docs/llms.txt`. A page missing from
`ORDERED_PAGES` silently drops out of the generated `llms-full.txt`; one missing from
`docs/llms.txt` is invisible to LLM consumers; one missing from the nav is unreachable
on the site.

This gate builds four sets of page paths relative to `docs/` — the `.md` files on disk
(minus snippet partials), the nav entries, `ORDERED_PAGES`, and the `llms.txt` links —
and asserts they are equal, using the disk set as the reference. It reports every
mismatch before exiting non-zero. Pure local: no network, no writes.

Runs as a prek hook scoped to the four inputs and in CI via
`tests/test_check_docs_nav.py`.

Usage:
    uv run scripts/check_docs_nav.py
"""

from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS_DIR = ROOT / "docs"
ZENSICAL_TOML = ROOT / "zensical.toml"
GEN_LLMS_FULL = ROOT / "scripts" / "gen_llms_full.py"
LLMS_TXT = ROOT / "docs" / "llms.txt"

# Directories under docs/ excluded from the page set (mirrors gen_llms_full.py)
EXCLUDE_DIRS = {"includes"}  # snippet partials, not pages

# Quoted nav values ending in .md (nav keys like "C# / .NET" never end in .md).
# Regex parsing instead of tomllib: CI's python-test matrix includes 3.10.
NAV_MD_RE = re.compile(r'"([^"]+\.md)"')

# Absolute page links in docs/llms.txt; llms-full.txt is excluded by the .md suffix
LLMS_MD_RE = re.compile(r"https://lib\.iscc\.codes/(\S+?\.md)")


def disk_pages(docs_dir: Path) -> set[str]:
    """Return all .md page paths under `docs_dir`, relative, skipping EXCLUDE_DIRS."""
    pages = set()
    for md_file in docs_dir.rglob("*.md"):
        rel = md_file.relative_to(docs_dir)
        if rel.parts[0] in EXCLUDE_DIRS:
            continue
        pages.add(rel.as_posix())
    return pages


def nav_pages(toml_path: Path) -> set[str]:
    """Extract the .md page paths from the `nav = [ ... ]` block of zensical.toml."""
    text = toml_path.read_text(encoding="utf-8")
    match = re.search(r"^nav = \[(.*?)^\]", text, re.DOTALL | re.MULTILINE)
    if match is None:
        sys.exit(f"error: no `nav = [ ... ]` block found in {toml_path}")
    return set(NAV_MD_RE.findall(match.group(1)))


def ordered_pages(script_path: Path) -> set[str]:
    """Load gen_llms_full.py by path and return its ORDERED_PAGES as a set."""
    spec = importlib.util.spec_from_file_location("gen_llms_full", script_path)
    if spec is None or spec.loader is None:
        sys.exit(f"error: cannot load {script_path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return set(module.ORDERED_PAGES)


def llms_pages(llms_path: Path) -> set[str]:
    """Extract the linked page paths from docs/llms.txt."""
    return set(LLMS_MD_RE.findall(llms_path.read_text(encoding="utf-8")))


def diff_report(name: str, reference: set[str], actual: set[str]) -> list[str]:
    """Report pages missing from / unexpected in `actual` relative to `reference`."""
    errors = []
    missing = sorted(reference - actual)
    if missing:
        errors.append(f"{name}: missing {len(missing)} page(s): {missing}")
    unexpected = sorted(actual - reference)
    if unexpected:
        errors.append(f"{name}: unexpected {len(unexpected)} page(s): {unexpected}")
    return errors


def run_checks(
    docs_dir: Path, toml_path: Path, script_path: Path, llms_path: Path
) -> list[str]:
    """Compare the three hand-wired lists against the pages on disk."""
    reference = disk_pages(docs_dir)
    errors = []
    errors.extend(diff_report("zensical.toml nav", reference, nav_pages(toml_path)))
    errors.extend(diff_report("ORDERED_PAGES", reference, ordered_pages(script_path)))
    errors.extend(diff_report("llms.txt", reference, llms_pages(llms_path)))
    return errors


def main() -> int:
    """Run the parity checks against the repository and report the result."""
    errors = run_checks(DOCS_DIR, ZENSICAL_TOML, GEN_LLMS_FULL, LLMS_TXT)
    for error in errors:
        print(f"error: {error}")
    if errors:
        return 1
    count = len(disk_pages(DOCS_DIR))
    print(
        f"OK: {count} documentation pages consistent across nav, "
        "ORDERED_PAGES and llms.txt."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
