---
name: audit-watch-list
description: Verified audit findings not filed yet (issues.md budget or ownership), first candidates for the next audit pass
metadata:
  type: project
---

Verified true in the iter-249 sweep (HEAD deb5c5e). Not filed because issues.md hit its 300-line
budget with 4 entries. Re-check against the tree before filing; drop any that a commit has fixed.

**Why:** every one survived a skeptic pass; only the issues.md budget kept them out. **How to
apply:** file the top 1-2 next pass if issues.md has room; target.md/decisions items are `low` +
`**Triage:** untriaged` (human-owned).

Context drift (CID-fixable specs / per-crate CLAUDE.md):

- Tag-triggered release still claimed in `specs/swift-bindings.md:238-247`, `specs/c-ffi-dx.md:128`,
    `crates/iscc-py/CLAUDE.md`, `crates/iscc-wasm/CLAUDE.md` (the ci-cd.md part is already filed).
- `specs/python-bindings.md:274,302-305`: names `py.allow_threads` and says the GIL is "still held".
    The code uses `.detach(` at 12 sites.
- `specs/rust-core.md:485`: the JS recipe destructures `iscc_decode` as an array. napi/WASM return
    an object.
- `specs/swift-bindings.md:147` promises defaults; UniFFI exports have none (Kotlin/Swift).
- `crates/iscc-lib/CLAUDE.md:7-8,40-44`: the Tier 2 codec list and the binding list (no uniffi) are
    stale.

Human-owned (target.md / decisions / agent prompts):

- decisions.md:90 and :260-264 still say "Fable 5.2 as advisor" and cite a missing `ADVISOR_ROLES`.
    :397 reversed both.
- decisions-archive.md:376 still calls the removed Unicode freeze "the complete intended response".
- `.claude/agents/advance.md:153-154` and `review.md:342` say "stability-committed (v1.0.0+)". The
    version is 0.7.0.
- target.md hard-codes "35" Tier 1 symbols about 13 times, and the gen count disagrees (9 vs 10:
    :92/:138 vs :342/:360/:394).

Docs: `docs/llms.txt:5,9` lists 6 languages/3 installs; `docs/development.md` hook and task tables
are stale; docs/index.md + benchmarks.md say all 10 gen functions are ISO-specified and benchmarked
(gen_sum_code_v0 is neither).

Duplication (real, lower priority): IDv1 range checks copied into py/napi/wasm/rb/jni with 4 message
vocabularies; error-mapping closures repeated (py 26, wasm 32, napi 23); WASM `gen_sum_code_v0`
hand-rebuilds `SumHasher`; core parses the meta JSON twice in `gen_meta_code_v0`.

Loop tooling (for meta-improve/human, not the CID queue): `tools/metrics.py` gate timings run
warm-cache commands that differ from the pre-push hooks, so the cargo_test/clippy trend means
nothing. TIMEOUT rows log cost 0.0 (`tools/cid.py:281-288`). 8 untriaged `low` entries wait on the
human while the loop idles.
