"""Generate llms-full.txt and per-page .md files for LLM consumption."""

import re
from pathlib import Path

DOCS_DIR = Path(__file__).parent.parent / "docs"
SITE_DIR = Path(__file__).parent.parent / "site"

# Ordered list of doc pages for llms-full.txt (matches nav in zensical.toml).
# Pages discovered on disk but not listed here are still written as per-page
# .md files — they just don't appear in llms-full.txt in a defined position.
ORDERED_PAGES = [
    "index.md",
    "tutorials/getting-started.md",
    "howto/rust.md",
    "howto/python.md",
    "howto/ruby.md",
    "howto/nodejs.md",
    "howto/wasm.md",
    "howto/go.md",
    "howto/java.md",
    "howto/dotnet.md",
    "howto/c-cpp.md",
    "howto/swift.md",
    "architecture.md",
    "ecosystem.md",
    "rust-api.md",
    "api.md",
    "c-ffi-api.md",
    "java-api.md",
    "ruby-api.md",
    "benchmarks.md",
    "development.md",
]

# Directories to exclude from auto-discovery (contain partials, not pages)
EXCLUDE_DIRS = {"includes"}

# Regex to strip YAML frontmatter
FRONTMATTER_RE = re.compile(r"\A---\n.*?\n---\n", re.DOTALL)

# Regex to strip snippet auto-append directives
SNIPPET_RE = re.compile(r"^\*\[.*?\]:.*$", re.MULTILINE)


def strip_frontmatter(content):
    """Remove YAML frontmatter from markdown content."""
    return FRONTMATTER_RE.sub("", content)


def strip_snippets(content):
    """Remove abbreviation snippet definitions appended by pymdownx.snippets."""
    return SNIPPET_RE.sub("", content)


def clean_content(content):
    """Strip frontmatter, snippets, and normalize whitespace."""
    content = strip_frontmatter(content)
    content = strip_snippets(content)
    return content.strip()


def discover_pages():
    """Auto-discover all .md pages under docs/, excluding partial directories."""
    pages = set()
    for md_file in DOCS_DIR.rglob("*.md"):
        rel = md_file.relative_to(DOCS_DIR)
        # Skip files in excluded directories (e.g., includes/)
        if rel.parts[0] in EXCLUDE_DIRS:
            continue
        pages.add(rel.as_posix())
    return pages


def main():
    """Generate llms-full.txt and individual .md files from doc sources."""
    SITE_DIR.mkdir(parents=True, exist_ok=True)

    # Combine ordered pages with auto-discovered pages (ordered first, extras appended sorted)
    discovered = discover_pages()
    ordered_set = set(ORDERED_PAGES)
    extra_pages = sorted(discovered - ordered_set)
    all_pages = ORDERED_PAGES + extra_pages

    if extra_pages:
        print(f"Auto-discovered {len(extra_pages)} extra page(s): {extra_pages}")

    parts = []
    page_count = 0

    for page in all_pages:
        path = DOCS_DIR / page
        if not path.exists():
            print(f"Warning: {page} not found, skipping")
            continue
        content = clean_content(path.read_text(encoding="utf-8"))

        if not content:
            continue
        parts.append(content)

        # Write individual .md file to site directory
        md_path = SITE_DIR / page
        md_path.parent.mkdir(parents=True, exist_ok=True)
        md_path.write_text(content + "\n", encoding="utf-8")
        page_count += 1

    # Write concatenated llms-full.txt
    output = "\n\n---\n\n".join(parts) + "\n"
    out_path = SITE_DIR / "llms-full.txt"
    out_path.write_text(output, encoding="utf-8")
    print(f"Generated {out_path} ({len(parts)} pages, {len(output)} bytes)")
    print(f"Generated {page_count} individual .md files in {SITE_DIR}")


if __name__ == "__main__":
    main()
