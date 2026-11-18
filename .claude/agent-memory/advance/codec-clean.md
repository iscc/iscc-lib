---
name: codec-clean
description: iscc_clean ports — Rust (191-193, 198) and Go isccClean (200); semantics, empty guard, perf baseline history
metadata:
  type: project
---

# Codec input cleaning (`iscc_clean`)

Rust codec input-cleaning (iter 191): private `codec::iscc_clean(&str)->IsccResult<String>` ports
`iscc_core.codec.iscc_clean` — trims ws, case-insensitive `iscc:` scheme, strips dashes UNLESS first
char is multibase prefix (`f/b/v/z/u`). Routed through 4 sites: `iscc_decompose`, `iscc_normalize`,
`gen_mixed_code_v0`, `gen_iscc_code_v0` (`cleaned` is now `Vec<String>`). Go half of the same issue
is NOT done (packages/go isccNormalize etc.). `iscc_clean` is `pub(crate)` → its dedicated unit
tests live in codec.rs, not integration tests. Iter 192: `iscc_clean` now GUARDS empty-cleaned
result → `Err(InvalidInput("Empty ISCC string"))` (else
`iscc_decompose("   ")`/`"-"`/`"iscc:"`/`"----"` silently `Ok([])` since `decode_base32("")`=Ok). Go
port MUST replicate this guard, not copy the gap. Empty-input assertions live in
`tests/codec_clean.rs`. Iter 193: `iscc_clean` returns `Cow<'_,str>` (allocation-free borrow when no
scheme + no dash), `gen_iscc_code_v0.cleaned` is now `Vec<Cow<'_,str>>` — cut the 191/192 perf
regression (four_units +36.82%→+11.71%, mixed +18.30%→+6.95% GREEN). BUT four_units STILL red: probe
proved the residual ~+11.71% is INTRINSIC scanning cost of reference-correct cleaning (trim + 2×
memchr per code), NOT allocation (strip_prefix-only iscc_clean = +2.6%). std memchr beats a scalar
single-pass, so unfixable without a baseline bump (human/gate, next.md-forbidden here). iscc_clean
cyclo 7→9 in `.crap-baseline.json`. RESOLVED 2026-07-29: the human raised the four_units baseline to
13,400 (decisions.md) — Perf gate green since

Iter 198: all 4 codec trim sites (iscc_clean outer/scheme/code + `encode_seq` empty-unit guard) use
private `is_py_strip_char` = `char::is_whitespace` ∪ U+001C..=U+001F (Python `str.strip()` set,
CPython 3.13). Perf: four_units +0.19% vs pre-edit local run (12,784 Ir; baseline 13,369).
`utils.rs` `text_trim` / `text_remove_newlines` still use Rust `.trim()`/`split_whitespace` →
probe-confirmed divergence on Tier 1 (`'\x1eab'`), left for a separate issue. Go `isccClean` port
must copy the Python set (verify Go's `unicode.IsSpace`/`TrimSpace` against it — unprobed, don't
assume).

Iter 200 — Go half DONE: `packages/go/codec.go` `isccClean` (+ `isPyStripChar` = `unicode.IsSpace` ∪
U+001C..1F, `isIsccScheme` ASCII-only `|0x20` compare) routed through `IsccDecompose`,
`isccNormalize` (checkPrefix on cleaned), `GenIsccCodeV0`, `GenMixedCodeV0` (`clean code %d`
errors). Errors reason-only (no input echo). Tests `packages/go/clean_test.go` (`-run IsccClean`, 12
funcs, metamorphic forms × 4 entry points). Test-data gotchas: `IsccDecompose` of a wide-unit
ISCC-CODE yields 64-bit units; `EU…` (mixed vector input) fails `checkPrefix` so `IsccDecode`
rejects it — use `EI…`. Go `EncodeSeq`/`DecodeSeq` should reuse `isccClean`.
