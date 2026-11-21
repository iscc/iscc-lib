## 2026-11-21 — Realign agent-facing specs and CLAUDE.md files with the code

**Done:** Rewrote every release-trigger claim in the specs and per-crate `CLAUDE.md` files to match
`release.yml` (`workflow_dispatch` only; a `version` input makes `prepare-release` create the
`vX.Y.Z` / `packages/go/vX.Y.Z` tags and the GitHub Release). Fixed the four audit contradictions:
`py.detach`, the JS `iscc_decode` object destructure, the UniFFI no-defaults convention, and the
`iscc-lib/CLAUDE.md` crate list and Tier 2 list. Docs only, no code or workflow changes.

**Files changed:**

- `.claude/context/specs/ci-cd.md`: Release row trigger → `workflow_dispatch`; "Tag trigger as
    convenience" → "The workflow creates the release tags"; "primary release mechanism" → "only
    release trigger"; Trigger YAML drops `push: tags:`, adds the real `version` input, and a comment
    marks it as an excerpt (`ffi`, `maven-kotlin` omitted)
- `.claude/context/specs/swift-bindings.md`: "Named parameters with defaults" → without defaults
    (UniFFI exports declare no `default(...)`; `gen_meta_code_v0` takes `Option<String>` + `u32`
    bits, all required). Release Flow steps 1-7 rewritten from `prepare-release` (dispatch → version
    check → XCFramework (cache-aware) → `sed` Package.swift → commit+push main → `git tag` +
    `push --force` → GitHub Release with zip). The "Release Workflow Integration" section held a
    hypothetical `build-xcframework` YAML job gated on
    `startsWith(github.ref, 'refs/tags/v') || inputs.swift` (neither exists); replaced by a
    paragraph naming `prepare-release`
- `.claude/context/specs/c-ffi-dx.md`: "Tag push triggers FFI builds" → full release runs them;
    "Trigger: … `refs/tags/v` … || inputs.ffi" (line 118, sweep hit) → the real `build-ffi`
    condition `inputs.version != '' || inputs.ffi || inputs.nuget`
- `.claude/context/specs/python-bindings.md`: `py.allow_threads` → `py.detach` (only mention)
- `.claude/context/specs/rust-core.md`: `const { subtype: st, digest: body } = iscc_decode(…)` —
    field names from `#[napi(object)] IsccDecodeResult` and the WASM `#[wasm_bindgen]` struct
    (`maintype, subtype, version, length, digest`); mdformat reflowed it to multi-line
- `crates/iscc-lib/CLAUDE.md`: added `iscc-uniffi` to the binding list; Tier 2 line `read_unit`,
    `SEQ_MAINTYPES` (both `pub` in `codec.rs`: lines 744, 768)
- `crates/iscc-py/CLAUDE.md`: "Tag-triggered" bullet → dispatch; workflow creates the tag
- `crates/iscc-wasm/CLAUDE.md`: publishing bullet now names the `build-wasm` job, its condition, the
    real `wasm-pack` command (`crates/iscc-wasm` arg), version from root `Cargo.toml` (not "tag
    version"), and `npm publish --access public` (the old text claimed `--provenance`; release.yml
    line 511 has none, provenance comes from trusted publishing per its comment)

**Verification:**

- `git grep -n -i -E 'tag[- ]triggered|tag trigger as|tags: \[v\*|Tag push triggers' -- .claude/context/specs 'crates/*/CLAUDE.md'`
    → no output (exit 1)
- `git grep -n 'allow_threads' -- .claude/context/specs` → none; `git grep -n 'const \[mt'` → none
- `grep -q iscc-uniffi … && grep -q read_unit … && grep -q SEQ_MAINTYPES crates/iscc-lib/CLAUDE.md`
    → exit 0
- `git grep -n 'parameters with defaults' -- …/swift-bindings.md` → none
- Extra sweep `git grep -i -E 'tag push|on: release|release: types|github\.ref|refs/tags'` over
    specs + crate/package CLAUDE.md: only the two correct "a tag push does not trigger it" /
    "pushing a tag starts nothing" sentences remain
- `uv run scripts/check_spec_paths.py` exit 0 ("15 specs resolve");
    `uv run scripts/check_tier1_docs.py` exit 0 ("20 full and 2 exact Tier 1 docs match")
- `mise run check` exit 0 (first run: mdformat reflowed, second run clean); final wasm CLAUDE.md
    touch-up checked with `uv run prek run --files` (mdformat reflowed, rerun rc=0)

**Next:** Ruby floor 3.1 → 3.3 once the human CI changes (Python 3.15 leg, .NET 8/10) are green.

**Notes:**

- Scope deviation: the Swift "Release Workflow Integration" YAML and the c-ffi-dx line 118 were not
    listed but stated the same false tag trigger (`refs/tags/v`), which the next.md grep missed;
    fixed under the "sweep once … fix hits" note.
- Out-of-scope drift seen, not fixed: `crates/iscc-py/CLAUDE.md` wheel matrix lists Linux x86_64
    only (aarch64 omitted, per next.md); `c-ffi-dx.md` section 4 still reads "Add to `release.yml`"
    as if the jobs were future work; `swift-bindings.md` "Release Process — Force-Update Tag
    Pattern" heading and its Ferrostar paragraph still name the force-move pattern, while the
    workflow tags after the checksum commit and only force-pushes.
- Commit touches only specs, crate `CLAUDE.md` files, handoff and advance memory.
