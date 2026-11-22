## 2026-11-22 — Review of: Rust core rejects ISCC-UNITs above 256 bits (encode and decode)

**Verdict:** PASS

**Summary:** `codec::encode_length` now accepts only 32 to 256 bits for Meta through Flake, and a
`pub(crate)` checked helper `codec::unit_bit_length` replaces all four internal `decode_length`
calls. No `codec` signature changed, the diff stays inside next.md's scope (3 core files,
`docs/rust-api.md`, the regenerated CRAP baseline), and every criterion passes. A differential
against iscc-core 1.5.0 through the Python binding agrees on all six decode cases; the encode side
diverges on purpose (decision of 2026-11-22, upstream iscc-core#142).

**Verification:**

- [x] `cargo test -p iscc-lib` (lib 341) and `--no-default-features` (lib 284) pass;
    `cargo test -p iscc-lib above_256` runs 16 tests, 0 failed
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `decode_length` signature grep prints one line (`codec.rs:414`);
    `git grep -n "decode_length(" crates/iscc-lib/src/lib.rs` prints nothing
- [x] `mise run coverage && cargo crap … --fail-regression --fail-above` exits 0 (0 regressed, 0
    new, 0 moved, 190 unchanged, so the committed baseline equals a fresh run)
- [x] `mise run bench:iai:check` exits 0: 16 benchmarks within 10% (`bench_mixed_code` −4.22% and
    `bench_iscc_code` −5.24% against the committed baseline)
- [x] `uv run pytest tests/test_rust_api_signatures.py` 5 passed; `mise run check` exits 0
- [x] Extra: feature sets `text-processing` and `meta-code` alone pass; `uv run zensical build`
    reports no issues; no binding test or doc example calls a gen function, a `finalize` or
    `encode_component` with more than 256 bits (repo-wide `git grep`)

**Issues found:**

- Filed `low`, untriaged: "`iscc_decode` reads an 88-bit ISCC-ID that iscc-core rejects". Old
    behaviour that next.md excluded; measured while checking the decode side. Lengths above 96 are
    rejected by both
- Not filed (nit): `test_iscc_decode_above_256` and `test_iscc_decompose_above_256` stay green with
    the helper removed, because `iscc_decompose` re-encodes each unit and the encode bound then
    rejects it with a message that also contains "256". The rejection itself is pinned; the helper
    is pinned by its own test and by the mixed and wide tests
- Not filed (nit): the `gen_image_code_v0` row in `docs/rust-api.md` says "max 256" and not
    "multiple of 32, 32 to 256". True, but worded differently from the eight other rows
- No gen function in `lib.rs` has an `# Errors` section, so there was nothing to update there

**Codex review:** no actionable findings ("The 256-bit checks preserve the intended composite and ID
behavior").

**Next:** the Go port of the same bound, which is the next link of the chain to v0.8.0. The issue is
rewritten to the three parts that remain (Go, shared vector, API pages). For Go:

- `encodeLength` (`packages/go/codec.go:279`) gets the 32 to 256 bound and the Rust error wording
    ("must be a multiple of 32 from 32 to 256"); `codec_test.go` asserts on the old text
- `decodeLength` has four unchecked callers, the same four as in Rust: `codec.go:641`
    (`IsccDecompose`), `codec.go:792` (`IsccDecode`), `code_content_mixed.go:76` and the wide check
    in `code_iscc.go:88-89`. The wide check already short-circuits through `&&`; keep that order, so
    a 64-bit Data unit next to a 288-bit Instance unit still succeeds as in iscc-core
- Expected results to copy for the Go tests, all measured on Rust and iscc-core: a 288-bit Meta unit
    fails `IsccDecode` and `IsccDecompose`; a 288-bit Content unit fails mixed; a 288-bit Data unit
    fails `GenIsccCodeV0(.., wide=true)` and succeeds with `wide=false`

After Go: the `data-edge.json` rows (`diverges`), then the binding API pages. The pages that still
say only "multiple of 32" are listed by `git grep -c "multiple of 32" -- docs crates packages`.

**Notes:**

- The 10 surfaces on the core inherit the bound without an edit. Measured on the Python binding
    only: `bits=288` on meta, text, audio, data, instance, both hashers' `finalize` and
    `encode_component(…, 512, …)` raise `ValueError`. The other bindings are pinned once the
    `data-edge.json` rows exist
- Mixed with `bits=288` was already rejected before this step ("Content-Code too short"), because no
    valid input unit has 288 bits. The data-edge row for mixed therefore proves nothing about the
    encode bound; use a gen function without input units for that row
- Error order differs from iscc-core in mixed (Rust checks main type then length per code, the
    reference all main types first). Both reject, only the message can differ
- `.iai-baseline.json` was left alone, as next.md asked; the stale `bench_instance_code` rows are
    the existing `low` issue
