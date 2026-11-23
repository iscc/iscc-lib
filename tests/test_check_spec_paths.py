"""Tests for the spec path-existence gate in ``scripts/check_spec_paths.py``.

The anchor test runs the check against the real specs and the real ``git ls-files``
set and asserts zero errors — this carries the gate into CI (CI runs pytest, never
prek) and catches deletions the ``files:``-scoped hook never sees. The mutation
tests write a spec into ``tmp_path`` and inject a known-path set, one bad case per
call. A real-regression test replays the Go structure tree the specs carried before
the check existed against a frozen snapshot of the ``packages/go/`` files that tree
names and that exist, and asserts exactly its 11 phantom files light up. Every
fixture tree carries the "not every file" note the gate demands after its fence.
"""

import importlib.util
from pathlib import Path

# Load scripts/check_spec_paths.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "check_spec_paths.py"
_spec = importlib.util.spec_from_file_location("check_spec_paths", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
csp = importlib.util.module_from_spec(_spec)
_loader.exec_module(csp)

FENCE = "```"

# The partial-list note every tree needs after its closing fence, as a paragraph.
NOTE = "\nThe tree lists the load-bearing entries, not every file.\n"

# The Go "Package Structure" tree as committed before this gate (go-bindings.md).
OLD_GO_TREE = (
    """```
packages/go/
├── go.mod                  # Module: github.com/iscc/iscc-lib/packages/go
├── go.sum
├── iscc.go                 # Main API (gen_*_v0 functions)
├── iscc_test.go            # Conformance tests against data.json
├── codec.go                # Header encode/decode, base32, component encoding
├── codec_test.go
├── text.go                 # Unicode normalization, cleaning, trimming, collapsing
├── text_test.go
├── algorithms.go           # CDC, MinHash, SimHash, sliding window
├── algorithms_test.go
├── dct.go                  # Nayuki fast recursive DCT
├── dct_test.go
├── wta.go                  # WTA-Hash (video fingerprinting)
├── wta_test.go
├── hasher.go               # DataHasher, InstanceHasher streaming types
├── hasher_test.go
├── constants.go            # Algorithm constants (META_TRIM_*, IO_READ_SIZE, etc.)
├── testdata/
│   └── data.json           # Vendored conformance vectors
└── README.md               # Per-package README
```
"""
    + NOTE
)

OLD_GO_PHANTOMS = [
    "iscc.go",
    "iscc_test.go",
    "text.go",
    "text_test.go",
    "algorithms.go",
    "algorithms_test.go",
    "wta.go",
    "wta_test.go",
    "hasher.go",
    "hasher_test.go",
    "constants.go",
]


def _known(*paths):
    """Return the known-path set for `paths`: each path plus all its ancestors."""
    known = set()
    for path in paths:
        parts = path.split("/")
        known.update("/".join(parts[:i]) for i in range(1, len(parts) + 1))
    return known


KNOWN = _known(
    "packages/demo/go.mod",
    "packages/demo/gradle/wrapper/gradle-wrapper.properties",
    "packages/demo/testdata/data.json",
    "packages/demo/code_meta.go",
    "scripts/tool.py",
    ".github/workflows/ci.yml",
)

# A consistent tree: nesting, a multi-segment name, a glob entry, a GENERATED child.
GOOD_TREE = f"""{FENCE}
packages/demo/
├── go.mod                  # module file
├── code_*.go               # glob entry
├── gradle/wrapper/         # multi-segment name
├── testdata/
│   └── data.json
└── back/
{FENCE}
{NOTE}
{FENCE}
crates/iscc-wasm/
└── pkg/
    └── iscc_wasm.js        # generated, untracked
{FENCE}
{NOTE}"""

# The packages/go/ files the old tree names that exist — a frozen snapshot, so the
# replay tests do not depend on the live index.
OLD_GO_KNOWN = _known(
    *(
        f"packages/go/{name}"
        for name in (
            "go.mod",
            "go.sum",
            "codec.go",
            "codec_test.go",
            "dct.go",
            "dct_test.go",
            "testdata/data.json",
            "README.md",
        )
    )
)

GOOD_PROSE = (
    "Run `scripts/tool.py:12` on `packages/demo/` per `packages/go/vX.Y.Z`"
    " in `.github/workflows/ci.yml` under `.github/workflows/`.\n"
)


def _spec_file(tmp_path, text):
    """Write `text` as a spec markdown file in tmp_path and return its path."""
    spec = tmp_path / "demo.md"
    spec.write_text(text, encoding="utf-8")
    return spec


def _errors(tmp_path, text, known=None):
    """Run the checks on a single spec holding `text`; return the error list."""
    known = (KNOWN | _known("packages/demo/back/x")) if known is None else known
    return csp.run_checks([_spec_file(tmp_path, text)], known)


def test_real_repo_passes():
    # The gate passes on the repository as committed, with non-trivial counts,
    # and main() (the prek hook entry point) exits 0.
    known = csp.tracked_paths(csp.ROOT)
    specs = sorted(csp.SPEC_DIR.glob("*.md"))
    totals = [sum(csp.check_spec(s, known)[1][i] for s in specs) for i in range(3)]
    assert all(total > 0 for total in totals)
    assert csp.run_checks(specs, known) == []
    assert csp.main() == 0


def test_tracked_paths_include_ancestors_not_build_output():
    # Directories count as known through their tracked children; untracked build
    # output never does, even where it exists on a local disk.
    known = csp.tracked_paths(csp.ROOT)
    assert {"crates", "crates/iscc-lib", "crates/iscc-lib/Cargo.toml"} <= known
    assert "crates/iscc-napi/index.js" not in known


def test_fixture_spec_passes(tmp_path):
    # Nested entries, a multi-segment name, a glob, a GENERATED child, a `:line`
    # suffix, a trailing slash and a NOT_PATHS string all resolve.
    assert _errors(tmp_path, GOOD_TREE + GOOD_PROSE) == []


def test_tree_entries_resolve_against_root_and_depth():
    # The stack resolves each entry to root + its ancestors at shallower depths.
    roots, entries = csp.tree_entries(GOOD_TREE)
    assert roots == 2
    assert [path for _, path in entries] == [
        "packages/demo/go.mod",
        "packages/demo/code_*.go",
        "packages/demo/gradle/wrapper",
        "packages/demo/testdata",
        "packages/demo/testdata/data.json",
        "packages/demo/back",
        "crates/iscc-wasm/pkg",
        "crates/iscc-wasm/pkg/iscc_wasm.js",
    ]


def test_phantom_tree_entry_fires(tmp_path):
    tree = GOOD_TREE.replace("├── go.mod ", "├── ghost.go ")
    assert _errors(tmp_path, tree + GOOD_PROSE) == [
        "demo.md:3: tree entry `packages/demo/ghost.go` is not tracked"
    ]


def test_phantom_nested_tree_entry_fires(tmp_path):
    tree = GOOD_TREE.replace("└── data.json", "└── ghost.json")
    assert _errors(tmp_path, tree + GOOD_PROSE) == [
        "demo.md:7: tree entry `packages/demo/testdata/ghost.json` is not tracked"
    ]


def test_phantom_backticked_path_fires(tmp_path):
    prose = GOOD_PROSE + "See `crates/ghost/lib.rs`.\n"
    assert _errors(tmp_path, GOOD_TREE + prose) == [
        "demo.md:21: backticked `crates/ghost/lib.rs` is not tracked"
    ]


def test_non_matching_glob_fires(tmp_path):
    prose = GOOD_PROSE + "Every `packages/demo/*.rs` file.\n"
    assert _errors(tmp_path, GOOD_TREE + prose) == [
        "demo.md:21: backticked `packages/demo/*.rs` is not tracked"
    ]


def test_entry_without_root_fires(tmp_path):
    # The root resets at each fence, so an entry in a later fence with no root
    # line of its own is an error rather than a child of the previous root.
    orphan = f"{FENCE}\n├── go.mod\n{FENCE}\n{NOTE}"
    assert _errors(tmp_path, GOOD_TREE + orphan + GOOD_PROSE) == [
        "demo.md:21: tree entry with no root line"
    ]


def test_generated_prefix_needs_exact_segment(tmp_path):
    # `pkg` exempts `pkg/…` but not a sibling that merely shares the prefix.
    tree = GOOD_TREE.replace("└── pkg/", "└── pkgx/")
    assert _errors(tmp_path, tree + GOOD_PROSE) == [
        "demo.md:15: tree entry `crates/iscc-wasm/pkgx` is not tracked",
        "demo.md:16: tree entry `crates/iscc-wasm/pkgx/iscc_wasm.js` is not tracked",
    ]


def test_floor_zero_roots(tmp_path):
    # Entries but no root line anywhere: the orphan error plus the root floor.
    assert _errors(tmp_path, f"{FENCE}\n├── go.mod\n{FENCE}\n{NOTE}" + GOOD_PROSE) == [
        "demo.md:2: tree entry with no root line",
        "floor: zero tree root lines found — parse failure or moved specs?",
    ]


def test_floor_zero_entries(tmp_path):
    assert _errors(tmp_path, f"{FENCE}\npackages/demo/\n{FENCE}\n" + GOOD_PROSE) == [
        "floor: zero tree entries found — parse failure or moved specs?"
    ]


def test_floor_zero_backticked_paths(tmp_path):
    assert _errors(tmp_path, GOOD_TREE) == [
        "floor: zero backticked paths found — parse failure or moved specs?"
    ]


def test_old_go_tree_reports_its_eleven_phantoms(tmp_path):
    # Real regression: the pre-gate Go tree, resolved against the frozen snapshot
    # of the packages/go/ files it names that exist, reports exactly the 11 files
    # packages/go never had.
    spec = _spec_file(tmp_path, OLD_GO_TREE)
    errors, counts = csp.check_spec(spec, OLD_GO_KNOWN)
    assert counts == (1, 20, 0)
    phantoms = [e.split("`")[1].removeprefix("packages/go/") for e in errors]
    assert phantoms == OLD_GO_PHANTOMS


def test_main_exits_nonzero_on_phantom(tmp_path, monkeypatch, capsys):
    # main() (the prek hook entry point) returns 1 and prints the phantom.
    spec_dir = tmp_path / "specs"
    spec_dir.mkdir()
    (spec_dir / "go.md").write_text(OLD_GO_TREE + GOOD_PROSE, encoding="utf-8")
    monkeypatch.setattr(csp, "SPEC_DIR", spec_dir)
    monkeypatch.setattr(csp, "tracked_paths", lambda root: OLD_GO_KNOWN | KNOWN)
    assert csp.main() == 1
    assert "go.md:5: tree entry `packages/go/iscc.go` is not tracked" in (
        capsys.readouterr().out
    )


def test_single_star_stays_in_one_segment(tmp_path):
    # KNOWN holds packages/demo/testdata/data.json but no JSON directly under
    # packages/demo/, so `*` (one segment) fires while `**` (any depth) resolves.
    good = GOOD_PROSE + "All `packages/demo/**/*.json` under `packages/demo/**`.\n"
    assert _errors(tmp_path, GOOD_TREE + good) == []
    bad = GOOD_PROSE + "Every `packages/demo/*.json` file.\n"
    assert _errors(tmp_path, GOOD_TREE + bad) == [
        "demo.md:21: backticked `packages/demo/*.json` is not tracked"
    ]


def test_not_paths_does_not_exempt_tree_entry(tmp_path):
    # A NOT_PATHS string exempts a backticked span only: as a tree entry it
    # claims a file exists, so it must resolve against the tracked set.
    tree = f"{FENCE}\nscripts/\n└── gen_ffi_boundary_vectors.py\n{FENCE}\n{NOTE}"
    assert _errors(tmp_path, GOOD_TREE + tree + GOOD_PROSE) == [
        "demo.md:22: tree entry `scripts/gen_ffi_boundary_vectors.py` is not tracked"
    ]
    prose = GOOD_PROSE + "History: `scripts/gen_ffi_boundary_vectors.py`.\n"
    assert _errors(tmp_path, GOOD_TREE + prose) == []


def test_glob_double_star_crosses_segments_single_star_does_not():
    known = _known("crates/iscc-lib/src/codec.rs", "packages/go/code_meta.go")
    assert csp.glob_regex("crates/*/src/*.rs").fullmatch("crates/iscc-lib/src/codec.rs")
    assert not csp.glob_regex("crates/*.rs").fullmatch("crates/iscc-lib/src/codec.rs")
    assert csp.glob_regex("crates/**.rs").fullmatch("crates/iscc-lib/src/codec.rs")
    assert csp.resolves("crates/iscc-lib/**", known)
    assert csp.resolves("packages/go/code_*.go", known)
    assert not csp.resolves("crates/*/codec.rs", known)
    assert not csp.resolves("packages/*.go", known)
    # Regex metacharacters in a glob are literal: `.` must not match the `o` of `.go`.
    assert not csp.resolves("packages/go/code_*.g.", known)


# A one-entry tree whose fence opens on line 1 of the spec, for the note tests.
SMALL_TREE = f"{FENCE}\npackages/demo/\n└── go.mod\n{FENCE}\n"


def test_tree_followed_by_heading_fires(tmp_path):
    # A heading right after the fence means the note is missing, even when a
    # note paragraph follows the heading.
    text = SMALL_TREE + "\n## Next\n" + NOTE + GOOD_PROSE
    assert _errors(tmp_path, text) == [
        'demo.md:1: tree has no "not every file" note after its fence'
    ]


def test_note_one_paragraph_late_fires(tmp_path):
    # Only the first paragraph after the fence counts.
    text = SMALL_TREE + "\nThe module file comes first.\n" + NOTE + GOOD_PROSE
    assert _errors(tmp_path, text) == [
        'demo.md:1: tree has no "not every file" note after its fence'
    ]


def test_note_wrapped_across_lines_passes(tmp_path):
    # mdformat wraps at 100 columns; the paragraph's lines are joined, so a
    # phrase split across lines (in the second sentence) still counts.
    note = "\nThe module file comes first. The tree lists the load-bearing entries, not\nevery file.\n"
    assert _errors(tmp_path, SMALL_TREE + note + GOOD_PROSE) == []


def test_phantom_github_backticked_path_fires(tmp_path):
    # `.github/` spans are checked like the other prefixes.
    prose = GOOD_PROSE + "Run `.github/workflows/x.yml`.\n"
    assert _errors(tmp_path, GOOD_TREE + prose) == [
        "demo.md:21: backticked `.github/workflows/x.yml` is not tracked"
    ]
