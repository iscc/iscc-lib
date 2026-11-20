# Next Work Package

## Step: Binding specs stop naming repository paths that do not exist

## Goal

Resolve the `normal` [review] issue "`wasm-bindings.md` names a committed
`crates/iscc-wasm/package.json` that does not exist", and fix the same defect wherever else it
occurs in the binding specs: a backticked repository path that no file backs. A future step could
"restore" such a file and create a second, unsynced source.

## Alternatives Considered

- **Chosen:** remove every phantom repo path in `.claude/context/specs/` (wasm + go + nodejs). It
    closes one `normal` issue completely, and the sibling phantoms are the same defect in the same
    files. Fixing only wasm would leave two forms of the problem.
- **Rejected:** the Tier 1 parity check (`normal` [human] derived-facts issue). Which pages
    enumerate the list, and how each language's names map back to the Rust names, are unprobed. It
    needs its own scoping pass. A sweep of hand-written counts waits for that check, because the
    issue removes only counts that "no check covers".

## Scope

- **Modify**: `.claude/context/specs/wasm-bindings.md`, `.claude/context/specs/go-bindings.md`,
    `.claude/context/specs/nodejs-bindings.md` (docs only, so no file budget applies)
- **Reference**: `.claude/context/specs/ci-cd.md` ("Version source of truth" and "Automatic
    propagation" sections), `scripts/version_sync.py` (`TARGETS` list, which has no wasm or Go
    entry), `.claude/context/specs/documentation.md` ("Navigation and Structure" → Reference list)

## Not In Scope

- Tier 1 counts ("all 35 Tier 1 symbols"), test/vector counts and the Crate Structure trees' missing
    files (e.g. the wasm tree omits `tests/unit.rs`, `tests/data_edge.rs`). Those belong to the
    derived-facts issue and wait for its parity check
- Creating `crates/iscc-wasm/package.json`, `packages/go/version.go`, `docs/go-api.md` or
    `docs/nodejs-api.md`. None is required by `target.md` or `documentation.md`
- `notes/` (the historical design record; never swept), `target.md`, and the CI YAML snippets inside
    the specs
- `edge-vectors.md`'s `scripts/gen_ffi_boundary_vectors.py`: a deliberate historical reference to a
    removed script ("removed … added in commit"), so leave it
- issues.md (review resolves the issue)

## Implementation Notes

Five phantom sites, found by checking that every backticked
`crates/|packages/|scripts/|docs/|tests/` path in `specs/*.md` exists:

1. `wasm-bindings.md` Crate Structure tree: the top-level `├── package.json  # npm package config`
    line. Remove it. The indented `pkg/` child `package.json` (wasm-pack output) **stays**. Add
    nothing new to the tree.
2. `wasm-bindings.md` "Version Sync": the table row for `crates/iscc-wasm/package.json`. Replace the
    table with one or two sentences: wasm-pack generates `pkg/package.json` from
    `crates/iscc-wasm/Cargo.toml` (version, `license`, `description`), release.yml's "Set npm
    package name and version" step patches `name`, `repository` and `version`, and `Cargo.toml` is
    the single source of npm metadata. Refer to ci-cd.md's "Automatic propagation" section **by
    name**, never by line number. Check the wording against `.github/workflows/release.yml` first.
3. `go-bindings.md` "Version Sync": the `packages/go/version.go` row. No such file and no sync
    target exists, so drop the table and the "embedded version constant is synced" sentence. Keep
    the point that Git tags set the Go module version, and correct the tag form: the spec says
    `go/vX.Y.Z`, while ci-cd.md and target.md use `packages/go/vX.Y.Z`.
4. `go-bindings.md` Documentation: `**API reference**: docs/go-api.md`. Remove the line.
5. `nodejs-bindings.md` Documentation: `**API reference**: docs/nodejs-api.md`. Remove the line.

For 4 and 5: the how-to guide and README lines stay. `documentation.md`'s Reference list (Python,
Rust, Java, C FFI) is the authority on which API pages exist.

The prek mdformat hook reflows Markdown, so run `mise run format` before committing.

## Verification

- `grep -nE '^├── package\.json' .claude/context/specs/wasm-bindings.md` prints nothing, and
    `grep -c 'package.json' .claude/context/specs/wasm-bindings.md` is ≥ 1 (the `pkg/` child line
    survives)

- `grep -rn 'crates/iscc-wasm/package.json\|packages/go/version.go\|docs/go-api.md\|docs/nodejs-api.md' .claude/context/specs/`
    prints nothing

- `grep -n 'go/vX' .claude/context/specs/go-bindings.md | grep -v 'packages/go/vX'` prints nothing

- The loop below prints exactly two lines, `packages/go/vX.Y.Z` (a tag) and
    `scripts/gen_ffi_boundary_vectors.py` (the historical reference):

    ```bash
    for f in .claude/context/specs/*.md; do
        grep -oE '`(crates|packages|scripts|docs|tests)/[^` *:]+`' "$f" | tr -d '`' | sort -u |
        while read p; do [ -e "$p" ] || echo "$p"; done
    done | sort -u
    ```

- `mise run check` exits 0

## Done When

All verification checks pass on the working tree, and no binding spec names a repository file that
does not exist.
