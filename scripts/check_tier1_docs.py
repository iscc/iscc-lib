"""Tier 1 doc-parity check: docs that enumerate the Tier 1 API must match the crate root.

The Tier 1 API is whatever `crates/iscc-lib/src/lib.rs` makes public at the crate
root. Docs that list it — binding API references, how-to guides, the Rust spec and
the crate's `CLAUDE.md` — were written by hand and drifted: a symbol bound on every
surface went undocumented on some. This gate derives the symbol set from `lib.rs`
and compares every registered doc against it.

Source of truth (parsed, never typed out): column-0 `pub fn NAME`, `pub const NAME`
(and `pub const fn NAME`) and named `pub use path::{a, b}` / `pub use path::name`
items, whose group members must be bare identifiers. The parse is fail-closed: every
column-0 `pub` item must either derive symbols or match `EXCLUDED_ITEMS` (restricted
`pub(...)` visibility, `pub mod NAME;` and a few named items such as
`pub use types::*;`); any other form (`pub struct`, `pub static`, `pub unsafe fn`,
`pub use x as y`, a nested or renaming group, another glob) fails the gate. A floor
rejects a parse that collapses: the derived set must contain `FLOOR_SYMBOLS`.

Matching a symbol in a doc: identifier tokens are compared after lowercasing and
dropping `_`, so `gen_sum_code_v0`, `genSumCodeV0` and `GenSumCodeV0` all match, as
do `META_TRIM_META` and `MetaTrimMeta`. A leading C `iscc` prefix is accepted
(`iscc_meta_trim_meta`). A streaming type (PascalCase symbol) also matches a token
that starts or ends with its name (`NewDataHasher`, `iscc_data_hasher_new`). Plain
substring matching is never used: `isccdecode` occurs inside `IsccDecodeResult`.

Registry (in this file):

- `FULL`: docs that must name every Tier 1 symbol, in any casing (one-way check).
- `EXACT`: docs holding a `<!-- tier1:begin -->` / `<!-- tier1:end -->` block whose
  set of backticked spans must EQUAL the derived set — extras and repeats fail too.
- `EXEMPT`: path -> reason, for docs that enumerate the API by design but partially.

Discovery is fail-closed: every tracked `*.md` (from `git ls-files`, so staged files
count) outside `SKIP_PREFIXES` / `SKIP_FILES` that names at least
`DISCOVERY_THRESHOLD` distinct Tier 1 symbols must be registered. A registered path
that is not tracked is an error too.

Runs as a prek hook and in CI via `tests/test_check_tier1_docs.py`.

Usage:
    uv run scripts/check_tier1_docs.py
"""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LIB_RS = "crates/iscc-lib/src/lib.rs"

FLOOR_SYMBOLS = {"gen_meta_code_v0", "DataHasher", "META_TRIM_NAME", "text_clean"}
DISCOVERY_THRESHOLD = 30

FULL = (
    "crates/iscc-ffi/CLAUDE.md",
    "crates/iscc-wasm/CLAUDE.md",
    "crates/iscc-wasm/README.md",
    "docs/architecture.md",
    "docs/c-ffi-api.md",
    "docs/java-api.md",
    "docs/howto/c-cpp.md",
    "docs/howto/dotnet.md",
    "docs/howto/go.md",
    "docs/howto/java.md",
    "docs/howto/kotlin.md",
    "docs/howto/nodejs.md",
    "docs/howto/python.md",
    "docs/howto/ruby.md",
    "docs/howto/rust.md",
    "docs/howto/swift.md",
    "docs/howto/wasm.md",
    "docs/ruby-api.md",
    "docs/rust-api.md",
    "packages/go/CLAUDE.md",
    "packages/go/README.md",
)

EXACT = (
    ".claude/context/specs/rust-core.md",
    "crates/iscc-lib/CLAUDE.md",
)

EXEMPT = {
    ".claude/context/target.md": "human-owned; enumerates the API by category count",
    "docs/api.md": (
        "renders the API from the `iscc_lib` module via `::: iscc_lib` (mkdocstrings);"
        " its source lists nothing by hand"
    ),
}

# Agent-written logs and design history: rewritten each cycle or frozen, never docs.
SKIP_PREFIXES = ("notes/", ".claude/agent-memory/")
SKIP_FILES = {
    ".claude/context/decisions.md",
    ".claude/context/decisions-archive.md",
    ".claude/context/handoff.md",
    ".claude/context/issues.md",
    ".claude/context/learnings.md",
    ".claude/context/learnings-archive.md",
    ".claude/context/next.md",
    ".claude/context/proposals.md",
    ".claude/context/state.md",
}

# Column-0 `pub` items in lib.rs that are recognised but are not Tier 1 symbols.
# Named items only: a kind-wide entry (any glob, any enum) would let a new public
# item reach the crate root without the gate seeing it. Adding one is a choice.
EXCLUDED_ITEMS = {
    r"pub\(": "restricted visibility: not public at the crate root",
    r"pub mod [A-Za-z_]\w*;": "a module is a namespace, not a symbol",
    r"pub use types::\*;": "result structs, named per binding, not Tier 1 symbols",
    r"pub enum IsccError\b": "the error type every binding maps to its own idiom",
    r"pub type IsccResult\b": "the `Result` alias over `IsccError`",
}

BEGIN_MARKER = "<!-- tier1:begin -->"
END_MARKER = "<!-- tier1:end -->"

PUB_START_RE = re.compile(r"^pub\b", re.MULTILINE)
# Derive patterns are applied with `.match(text, pos)` at a `PUB_START_RE` offset;
# each ends at a terminator so a longer form (`pub use a::b as c;`) cannot match.
PUB_ITEM_RE = re.compile(
    r"pub (?:const )?fn ([A-Za-z_]\w*)\s*[<(]|pub const ([A-Za-z_]\w*)\s*:"
)
PUB_USE_RE = re.compile(
    r"pub use [A-Za-z_]\w*(?:::[A-Za-z_]\w*)*::(?:\{([^{}]*)\}|([A-Za-z_]\w*));"
)
IDENT_RE = re.compile(r"[A-Za-z_]\w*")
TOKEN_RE = re.compile(r"[A-Za-z_]\w*")
BACKTICK_RE = re.compile(r"`([^`\n]+)`")


def group_members(group: str) -> list[str] | None:
    """Return the names of a `pub use path::{...}` group, or None if any is not bare.

    One trailing comma is allowed; a renamed (`b as c`), nested (`b::c`), `self` or
    `*` member, or an empty member anywhere else, makes the group unrecognised.
    """
    members = [m.strip() for m in group.split(",")]
    if len(members) > 1 and members[-1] == "":
        members.pop()
    if all(IDENT_RE.fullmatch(m) and m != "self" for m in members):
        return members
    return None


def item_symbols(lib_text: str, pos: int) -> list[str] | None:
    """Return the symbols the `pub` item at `pos` derives, or None if it derives none."""
    if item := PUB_ITEM_RE.match(lib_text, pos):
        return [item[1] or item[2]]
    if use := PUB_USE_RE.match(lib_text, pos):
        return group_members(use[1]) if use[1] is not None else [use[2]]
    return None


def is_excluded(lib_text: str, pos: int) -> bool:
    """Return True if the `pub` item at `pos` matches an `EXCLUDED_ITEMS` entry."""
    return any(re.compile(p).match(lib_text, pos) for p in EXCLUDED_ITEMS)


def scan_lib(lib_text: str) -> tuple[set[str], list[tuple[int, str]]]:
    """Classify every column-0 `pub` item of `lib_text`.

    Return the derived Tier 1 symbols and the unrecognised items as
    `(line number, first line)` pairs: items that neither derive symbols nor match
    `EXCLUDED_ITEMS`.
    """
    symbols: set[str] = set()
    unrecognised = []
    for start in PUB_START_RE.finditer(lib_text):
        pos = start.start()
        names = item_symbols(lib_text, pos)
        if names is not None:
            symbols.update(names)
        elif not is_excluded(lib_text, pos):
            line = lib_text.count("\n", 0, pos) + 1
            unrecognised.append((line, lib_text[pos:].split("\n", 1)[0].rstrip()))
    return symbols, unrecognised


def derive_symbols(lib_text: str) -> set[str]:
    """Return the Tier 1 symbol names declared at the crate root in `lib_text`."""
    return scan_lib(lib_text)[0]


def norm(name: str) -> str:
    """Return the casing-free key of an identifier: lowercase, underscores dropped."""
    return name.lower().replace("_", "")


def is_type_name(symbol: str) -> bool:
    """Return True for a PascalCase type symbol such as `DataHasher`."""
    return symbol[0].isupper() and any(ch.islower() for ch in symbol)


def doc_tokens(text: str) -> set[str]:
    """Return the normalized identifier tokens of a document."""
    return {norm(tok) for tok in TOKEN_RE.findall(text)}


def symbol_in(symbol: str, tokens: set[str]) -> bool:
    """Return True if `symbol` appears among `tokens` in any binding's casing."""
    key = norm(symbol)
    if key in tokens or "iscc" + key in tokens:
        return True
    if is_type_name(symbol):
        return any(t.startswith((key, "iscc" + key)) or t.endswith(key) for t in tokens)
    return False


def named_symbols(text: str, symbols: set[str]) -> set[str]:
    """Return the Tier 1 symbols a document names."""
    tokens = doc_tokens(text)
    return {s for s in symbols if symbol_in(s, tokens)}


def exact_block(text: str) -> str | None:
    """Return the text between the tier1 markers, or None if not exactly one block."""
    if text.count(BEGIN_MARKER) != 1 or text.count(END_MARKER) != 1:
        return None
    start = text.index(BEGIN_MARKER) + len(BEGIN_MARKER)
    end = text.index(END_MARKER)
    return text[start:end] if start <= end else None


def check_full(path: str, text: str, symbols: set[str]) -> list[str]:
    """Check that a FULL doc names every symbol; return errors."""
    missing = sorted(symbols - named_symbols(text, symbols))
    return [f"{path}: missing Tier 1 symbols: {', '.join(missing)}"] if missing else []


def check_exact(path: str, text: str, symbols: set[str]) -> list[str]:
    """Check that an EXACT doc's marker block lists exactly the symbols; return errors."""
    block = exact_block(text)
    if block is None:
        return [f"{path}: needs exactly one {BEGIN_MARKER} ... {END_MARKER} block"]
    spans = BACKTICK_RE.findall(block)
    listed = set(spans)
    errors = []
    if repeated := sorted(s for s in listed if spans.count(s) > 1):
        errors.append(f"{path}: tier1 block repeats: {', '.join(repeated)}")
    if missing := sorted(symbols - listed):
        errors.append(f"{path}: tier1 block missing: {', '.join(missing)}")
    if extra := sorted(listed - symbols):
        errors.append(f"{path}: tier1 block extra: {', '.join(extra)}")
    return errors


def is_skipped(path: str) -> bool:
    """Return True for a tracked file that discovery never inspects."""
    return path in SKIP_FILES or path.startswith(SKIP_PREFIXES)


def check_discovery(path: str, text: str, symbols: set[str]) -> list[str]:
    """Report an unregistered doc that names at least the threshold of symbols."""
    count = len(named_symbols(text, symbols))
    if count < DISCOVERY_THRESHOLD:
        return []
    return [
        (
            f"{path}: names {count} Tier 1 symbols but is not registered "
            "(add it to FULL, EXACT or EXEMPT in scripts/check_tier1_docs.py)"
        )
    ]


def tracked_markdown(root: Path) -> list[str]:
    """Return every tracked `*.md` path under `root`, relative and sorted."""
    git = shutil.which("git") or sys.exit("error: git not found on PATH")
    # S603: the argv is fixed (`git ls-files -z`), no shell and no external input.
    out = subprocess.run(  # noqa: S603
        [git, "ls-files", "-z", "--", "*.md"],
        cwd=root,
        capture_output=True,
        check=True,
    ).stdout.decode("utf-8")
    return sorted(filter(None, out.split("\0")))


def run_checks(root: Path, tracked: list[str]) -> list[str]:
    """Run every check against the docs under `root`; return all errors."""
    symbols, unrecognised = scan_lib((root / LIB_RS).read_text(encoding="utf-8"))
    if unrecognised:
        return [
            f"{LIB_RS}:{line}: unrecognised crate-root item: {first_line}"
            for line, first_line in unrecognised
        ]
    if lost := sorted(FLOOR_SYMBOLS - symbols):
        return [f"floor: {LIB_RS} parse lost {', '.join(lost)} — parse failure?"]
    registered = {*FULL, *EXACT, *EXEMPT}
    errors = [
        f"{p}: registered but not tracked" for p in sorted(registered - {*tracked})
    ]
    for path in tracked:
        if is_skipped(path) or path in EXEMPT:
            continue
        text = (root / path).read_text(encoding="utf-8")
        if path in FULL:
            errors += check_full(path, text, symbols)
        elif path in EXACT:
            errors += check_exact(path, text, symbols)
        else:
            errors += check_discovery(path, text, symbols)
    return errors


def main() -> int:
    """Run the check against the repository and report the result."""
    errors = run_checks(ROOT, tracked_markdown(ROOT))
    for error in errors:
        print(f"error: {error}")
    if errors:
        return 1
    print(f"OK: {len(FULL)} full and {len(EXACT)} exact Tier 1 docs match {LIB_RS}.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
