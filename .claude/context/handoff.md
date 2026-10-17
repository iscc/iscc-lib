## 2026-10-17 — Interactive session (Titusz + Claude): v0.7.0 released, context resynced

**Verdict:** PASS

**Summary:** The loop paused after iteration 193 (2026-07-29). Everything since happened in human
sessions, so this handoff replaces the iteration-193 review. The iteration-193 Perf-gate blocker was
resolved by the human (`four_units` baseline raised, decisions.md 2026-07-29), v0.6.0 shipped on
2026-07-30, and on 2026-10-17 v0.7.0 shipped with the iscc-core 1.5.0 Image-Code fix. `develop` ==
`main` == `d49b953`, CI green (all jobs incl. Perf, CRAP, Swift).

**What landed on 2026-10-17 (all on `develop` and released in 0.7.0):**

- **Image-Code fix (breaking for > 64 bits):** `soft_hash_image_v0` now takes the 8×8 DCT squares at
    `(0,0), (8,0), (0,8), (8,8)` per ISO 24138 5.4.5.2 e) — Rust `crates/iscc-lib/src/lib.rs` and
    pure Go `packages/go/code_content_image.go`, each with a quadrant layout test
- **Conformance data at iscc-core 1.5.0** in all five vendored `data.json` copies. New top-level
    sections `encode_seq` / `decode_seq` (non-object outputs, `bytes:<hex>` values) are not run by
    any harness yet. The Go loader `parseConformanceData` now loads only `gen_*` sections on purpose
- `uv.lock` pins iscc-core 1.5.0; `reference/iscc-core/` is checked out at `v1.5.0`
- **npm publishes via OIDC trusted publishing** (no `NPM_TOKEN`); see decisions.md
- `central-publishing-maven-plugin` 0.7.0 → 0.12.0 (Maven 3.10 on GitHub runners broke 0.7.0)
- `crates/iscc-wasm/src/lib.rs`: `getter_with_clone` moved to non-`Copy` fields (clippy 1.99
    `clone_on_copy`); generated JS/TS byte-identical
- All CID roles and `PRIMARY_MODEL` pinned to `claude-opus-5-5` (Fable retired)

**Verification:** CI green on `d49b953`; release run `39061523822` + maven re-run `39063243788`
green; 0.7.0 confirmed on crates.io, PyPI, npm (both), Maven Central (Java + Kotlin), Go proxy,
RubyGems, NuGet, GitHub Releases (FFI + Swift XCFramework). The published PyPI wheel reproduces the
new 256-bit vector and passes `conformance_selftest()`.

**Next:** v0.8.0 — the ISCC-SEQ codec (target.md "Current Release Milestone — v0.8.0", spec
`rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ, IEP-0020)"). Suggested order: Rust core +
ported `test_seq.py` cases first; then the Go `iscc_clean` issue (prerequisite for Go `encode_seq`);
then the bindings and the conformance harnesses. The four `normal` issues filed today (wasm npm
metadata, Ruby `json` alert, Ruby locale, `cid:run` fallback model) are small and independent.

**Notes:**

- Local gate runs in the devcontainer need `LANG=C.UTF-8` (Ruby) and, for `git push`, the `gh`
    credential helper — see learnings.md
- The Tier 1 count in target/specs is now 35 (desired state); crate `CLAUDE.md`, notes and docs
    still say 33 until ISCC-SEQ lands — update them in the step that makes it true
