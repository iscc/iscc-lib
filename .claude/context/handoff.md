## 2026-03-04 — Add Ruby documentation (howto guide, README, root README)

**Done:** Created the Ruby how-to guide at `docs/howto/ruby.md` (422 lines) covering all 10 gen
functions, streaming API, codec/diagnostics, text utilities, algorithm primitives, constants, and
error handling. Expanded `crates/iscc-rb/README.md` from a 31-line stub to a 93-line full guide with
badges, ISCC explanation, installation, quick start, API overview table, and links. Added Ruby
install and quickstart sections to root `README.md` (after Go, before WASM) and updated the polyglot
language lists. Added Ruby nav entry to `zensical.toml` after Python.

**Files changed:**

- `docs/howto/ruby.md`: Created — comprehensive Ruby how-to guide following Go guide structure
- `crates/iscc-rb/README.md`: Expanded from stub to full guide with badges, API table, links
- `README.md`: Added Ruby install section, quickstart example, updated language lists in Key
    Features and What is iscc-lib paragraphs
- `zensical.toml`: Added `{ "Ruby" = "howto/ruby.md" }` to How-to Guides nav after Python

**Verification:** All 8 verification criteria pass:

- howto guide exists (422 lines, ≥300 required)
- crate README expanded (93 lines, ≥60 required)
- `gem install iscc-lib` in root README
- `ruby.md` in zensical.toml nav
- howto contains `gen_meta_code_v0`, `DataHasher`, `conformance_selftest`
- `mise run check` exits 0 — all 14 pre-commit hooks pass

**Next:** The Ruby binding issue's documentation deliverables are complete. Remaining items from
state.md: standard Ruby linting (`standard` gem, `.standard.yml`, CI wiring) — this is lower
priority infrastructure work. The root README's Go quickstart example still uses the old WASM-bridge
API (`NewRuntime`, `rt.GenMetaCodeV0`) instead of the pure Go API (`iscc.GenMetaCodeV0`) — this is
an out-of-scope stale documentation issue.

**Notes:**

- The howto guide follows the Go guide structure (closest analog: package-level functions, typed
    results) with Ruby-specific patterns (keyword args, attribute-style access, method chaining).
- mdformat auto-fixed table column widths on first format run — second run was clean.
- Root README Go quickstart example is stale (uses WASM-bridge API) — noted for future cleanup but
    not touched since it's out of scope.
