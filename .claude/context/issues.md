# Issues

Tracked issues and feature requests for the CID workflow.

## Purpose

This file is a lightweight, append-only channel for humans and agents to flag problems that
define-next can prioritize. It bridges the gap between ad-hoc observations and the formal
state→target planning loop.

## Format

Each issue is an H2 heading with a priority tag, description, and source attribution:

```markdown
## [priority] Issue title

Description of the problem, including context and any relevant file paths or error messages.

**Source:** [human|review|advance]
**Spec:** .claude/context/target.md#section-name (optional — only if rooted in a spec gap)
**Upstream:** iscc/iscc-core (optional — only if the fix belongs in the upstream reference)
```

### Spec field

The optional `**Spec:**` field links an issue to a specific section in target.md or a sub-spec file
that needs updating. Source determines what happens:

- **`[human]` + `Spec:`** — the review agent updates the referenced spec directly when resolving the
    issue. The human created the issue, so the spec change is implicitly authorized.
- **`[review]`/`[advance]` + `Spec:`** — the review agent flags with `HUMAN REVIEW REQUESTED` and
    describes the proposed spec change in the issue description. It does NOT modify target.md. The
    human must approve and make the change.

### Upstream field

The optional `**Upstream:**` field marks an issue as belonging to an external repository (e.g.,
`iscc/iscc-core`). Upstream issues are always human-gated — filing on GitHub is a visible public
action that requires review regardless of source:

- Any issue with `**Upstream:**` triggers `HUMAN REVIEW REQUESTED` in the CID loop
- The issue description should include concrete evidence: failing conformance vectors, expected vs
    actual output, specific code references in the upstream repo
- The issue stays in issues.md as a draft until the human reviews it and either asks an interactive
    session to file it via `gh issue create -R <repo>` or files it manually
- After filing, update the issue description with the GitHub issue URL and delete it from issues.md

## Priority Levels

- **critical** — must be addressed in the next iteration; overrides normal gap analysis
- **normal** — considered alongside state→target gaps; define-next weighs it against other work
- **low** — pick up when no higher-priority work remains

## Management Rules

- **Append only** — add new issues at the end of this file
- **Resolution** — the review agent deletes resolved issues after verifying the fix (history lives
    in git)
- **Source tags** — agents that add issues must include a source tag: `[human]`, `[review]`, or
    `[advance]`
- **Scope** — track any problems that affect correctness, architecture, maintainability, or
    performance; not style preferences or minor nits

<!-- Add issues below this line -->

## [normal] gen_meta_code_v0: accept dict for meta parameter (auto-serialize to data URL)

GitHub: https://github.com/iscc/iscc-lib/issues/5

`gen_meta_code_v0` only accepts `str | None` for the `meta` parameter. In `iscc-core`, it also
accepts a `dict`, which it serializes to a JSON data URL (`data:application/json;base64,...`). This
blocks drop-in replacement of `iscc-core` in `iscc-sdk`.

**Approach:** Split into two layers:

1. **Rust core:** Add a `json_to_data_url(json: &str) -> String` utility function as Tier 1 API.
    Accepts a JSON string, base64-encodes it, and returns a `data:application/json;base64,...` data
    URL. Use `application/ld+json` media type when the JSON contains an `@context` key (matching
    iscc-core). This gives all bindings the encoding logic for free.
2. **Each binding (thin):** Type-check whether `meta` is the language's native dict/object type. If
    so, serialize to compact JSON string and call `json_to_data_url()`. This is inherently
    language-specific (Python `isinstance(meta, dict)`, JS `typeof meta === 'object'`) but minimal
    — just type dispatch + JSON serialize, then delegate to Rust.

**Source:** [human]

## [normal] Expose encode_component() and type enums (MT, ST, VS) for ISCC unit construction

GitHub: https://github.com/iscc/iscc-lib/issues/6

`iscc-lib` is missing `encode_component()` and the associated type enums (`MT`, `ST`, `VS`) in its
public API. These are needed for constructing ISCC units from raw digests. The Rust code already
exists in `crates/iscc-lib/src/codec.rs` but is Tier 2 (not exposed through bindings).

**Approach:** Promote to Tier 1 in the Rust core so all bindings get it for free:

1. **Rust core:** Promote `encode_component` from `codec` module to a Tier 1 crate-root re-export.
    Signature takes integer types for enums:
    `encode_component(mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8]) -> IsccResult<String>`.
    Consider rejecting `len(digest) < bit_length / 8` rather than silently truncating.
2. **Bindings (thin pass-through):** PyO3/napi/wasm expose the function with native integer args.
    Each binding provides language-idiomatic enum wrappers (Python `IntEnum`, TS `const enum`,
    etc.) that map to the same `u8` values as the Rust `MainType`/`SubType`/`Version` enums.

**Source:** [human]

## [normal] Add iscc_decode() for decomposing ISCC units into components

GitHub: https://github.com/iscc/iscc-lib/issues/7

`iscc-lib` is missing `iscc_decode()` which decodes an ISCC unit string back into its header
components and raw digest. This is the inverse of `encode_component()` and is needed for inspecting,
truncating, and re-encoding ISCC units. Used by `iscc-sdk` for `code_sum()`.

**Approach:** Implement in Rust core as Tier 1, expose through all bindings:

1. **Rust core:** Add `iscc_decode(iscc_unit: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>` as a
    Tier 1 crate-root function. Reuse existing codec helpers (`decode_base32`, `decode_header`,
    `decode_length`). Strip optional `ISCC:` prefix, base32-decode, decode header, compute
    `bit_length` via `decode_length()`, return exactly `bit_length//8` bytes from tail as digest.
    Return tuple: `(maintype, subtype, version, length_index, digest_bytes)` using `u8` for enum
    fields (same pattern as `encode_component`).
2. **Bindings (thin pass-through):** Expose the function, wrap returned `u8` enum fields in
    language-idiomatic enum types (Python `IntEnum` `MT`/`ST`/`VS` from issue #6).

Depends on #6 (shared enum types).

**Source:** [human]

## [normal] Expose algorithm configuration constants (core_opts equivalent)

GitHub: https://github.com/iscc/iscc-lib/issues/8

`iscc-lib` does not expose the algorithm configuration constants that `iscc-core` provides through
its `core_opts` object. These are needed by `iscc-sdk` and other ISCC libraries.

Constants to expose (verified against `iscc-core` `options.py`):

- `META_TRIM_NAME` = 128
- `META_TRIM_DESCRIPTION` = 4096 (note: GitHub issue text incorrectly says 2048; iscc-core and our
    Rust code both use 4096)
- `IO_READ_SIZE` = 4_194_304 (4 MB)
- `TEXT_NGRAM_SIZE` = 13

**Approach:** Define once in Rust core, expose through all bindings:

1. **Rust core:** Add `pub const` values in a dedicated `constants` module (or crate root). These
    are algorithm parameters defined by ISO 24138 — read-only by design.
2. **Bindings (thin pass-through):** Expose the constants directly. Each binding also provides a
    `core_opts` namespace/object for iscc-core API parity (Python `SimpleNamespace`, JS frozen
    object, etc.) — this is a thin wrapper mapping `core_opts.meta_trim_name` to the Rust constant.

**Source:** [human]
