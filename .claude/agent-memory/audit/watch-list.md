---
name: audit-watch-list
description: Verified audit findings not filed yet (5-per-pass cap or ownership), first candidates for the next audit pass
metadata:
  type: project
---

Verified true in the iter-260 sweep (HEAD 3040298) unless marked. Re-check against the tree before
filing; drop any that a commit has fixed.

**Why:** each survived a skeptic pass; only the 5-issue cap or ownership kept them out. **How to
apply:** hand every item to the verify phase next pass; file the top ones first. target.md/decisions
items are `low` + `**Triage:** untriaged` (human-owned).

Code (CID-fixable), ranked:

- IDv1 range checks repeated in py/napi/wasm/rb/jni (`gen_iscc_id_v1`), four wordings (core/py, JS
    `checked`, Ruby, JNI). On the list since iter 249 — file first.
- GLD-4: pre-push `uv run pytest` tests a hand-built `_lowlevel.abi3.so` that nothing rebuilds (no
    `[build-system]` in root pyproject, no conftest check); CI runs `maturin develop` first.
- `crates/iscc-uniffi/src/lib.rs:328` text_trim doc says "on a word boundary" (shipped in
    Swift/Kotlin generated sources).
- `_lowlevel.pyi` docstrings contradict code (encode_component ranges, meta trim units).
- .NET: null string -> IsccException, null array -> ArgumentNullException.
- Go naming: hashers `Push` (spec/others `update`), `IsccIdResult.ISCC` vs `Iscc`; Ruby
    `IdCodeResult` vs `IsccIdResult` everywhere else. API-break territory — likely v1.0.0 decisions.
- Go `MetaTrimMeta` unused, Go GenMetaCodeV0 enforces no meta size limit.
- napi alg_cdc_chunks tests "respects utf32 mode"/"accepts custom avg_chunk_size" can never fail.
- Duplication: error-mapping closures (py ~26, wasm ~32, napi ~23); JNI nested new_string/throw x21;
    napi/wasm JS number helpers copied; DataHasher/InstanceHasher wrapper pairs in FFI/.NET/C++;
    Python stream-read loop x5; conformance.rs runners x9 (report_case used by 2), Go conformance
    x8; soft_hash_meta_v0 repeats meta_name_simhash; bits_to_bytes twice (lib.rs, codec.rs);
    iscc_decode re-decodes base32 up to 4x; gen_meta_code_v0 124 lines.
- Dead: unused `serde` dep in iscc-wasm and iscc-lib dev-deps; ruff S/C901 pre-push hooks duplicate
    pyproject-selected rules.

Docs (CID-fixable):

- docs/development.md hook + mise task tables, project tree, tools table stale; prek link wrong repo
    (the open "Publish" issue covers only that section).
- Root CLAUDE.md: says mise pins tool versions (mise.toml has no [tools]); hook list misses 6.
- Getting-started says Go 1.21+, module needs 1.26.1. README binding list omits Swift. Six registry
    READMEs omit several Tier 1 functions. specs/documentation.md language lists stale.
- Binding spec CI YAML snippets pin stale action/Go versions (8 files). iscc-wasm CLAUDE.md
    flat-file threshold exceeded.

Human-owned (target.md / decisions / agent prompts), from iter 249, not re-checked at 260:

- decisions.md "Fable 5.2 as advisor" / `ADVISOR_ROLES` entries reversed later but still listed.
- `.claude/agents/advance.md`, `review.md` say "stability-committed (v1.0.0+)" at 0.7.0.
- target.md hard-codes "35" Tier 1 symbols; gen count 9 vs 10 disagrees.

Loop tooling (meta-improve/human, not the CID queue): `tools/metrics.py` times narrower warm-cache
commands than the pre-push hooks, one sample each, so cargo_test/clippy seconds are noise; TIMEOUT
rows log cost 0.0; `cid stats` Fails counts legacy PASS rows; metrics skip `scripts/` and ty
suppressions; on-demand runs consume iteration numbers.
