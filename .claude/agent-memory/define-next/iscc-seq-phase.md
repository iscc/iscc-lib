---
name: iscc-seq-phase
description: v0.8.0 ISCC-SEQ (encode_seq/decode_seq, #45) slice plan, scoping traps, and what iter 195 scoped
metadata:
  type: project
---

v0.8.0 = ISCC-SEQ on Rust core + 11 surfaces (Tier 1 33→35). Iter 195 scoped the Rust core slice:
codec.rs (raw u32 header codec + `read_unit` + `SEQ_MAINTYPES` + encode/decode_seq), lib.rs
re-export, conformance.rs (seq sections), new `tests/test_seq.rs`.

**Why:** spec order is Rust core → Go `isccClean` (normal issue, prereq only for Go seq) → bindings.

**How to apply:**

- Reference `conformance_selftest` walks EVERY data.json section with `bytes:` decoding, so the Rust
    selftest (and thus every binding's selftest) must include seq sections.
- Porting trap: typed `decode_header` rejects SubType≥8 and Version 1 on non-ID; seq needs a raw-u32
    path; never widen enums. `read_unit` offset>len must Err (Rust slice panics).
- `decode_header` is on the iai hot path → flag `bench:iai:check` + CRAP baseline regen per codec
    edit.
- Count/docs sweep 33→35 deferred to after the fan-out (IDv1 precedent, iter 190).
- Binding fan-out: Swift/Kotlin hard-code per-section counts; Go loader loads only `gen_*`; FFI step
    must regen+commit `iscc.h` and `NativeMethods.g.cs`.
- 196 = CI-red fix (deny ignore + macos-15). **197 = Python seq binding** (lib.rs + `__init__.py`
    wrapper for the TypeError/bytes-like contract + `.pyi` + `tests/test_seq.py` differential vs
    `iscc_core` 1.5.0). Next candidates: Go `decodeBase32` spare-bit fix + Rust/Go strict tests
    (normal issue), Go `isccClean`, then remaining 9 binding surfaces.
- Strict-base32 divergence case set (probed 197): 32-bit `ISCC:GAAAAAAAAB`, `ISCC:GAAAAAAAAA======`,
    256-bit `ISCC:EAD2…GBI` → last char `J`. Ref `encode_seq` accepts all, Rust rejects. 64-bit
    units have NO spare bits (16 chars exact); `KUAK…J` is an ISCC-CODE (ref rejects by MainType) —
    useless for seq. Seq vector sections have 4 cases each, zero error cases → negatives come from
    tests.
- PyO3 trap: `Vec<u8>` return → `list[int]`; return `PyBytes`. `&[u8]` arg rejects bytearray.
- **198 = core `iscc_clean` trim parity** (U+001C–U+001F; Python strip set = Rust `is_whitespace` +
    those 4, probed CPython 3.13, reverse set empty). 4 trim sites in codec.rs incl. the
    `encode_seq` empty guard. Chosen over handoff's Go `decodeBase32` because Go `isccClean` must
    copy the final set. Likely same divergence in `utils.rs` text_trim/text_collapse (ref
    `.strip()`/`.split()`) — left for review to file; touching it affects Meta/Text-Code output.
    Next: Go `decodeBase32` + Rust strict tests, Go `isccClean`, then 10 surfaces.
- **199 = Go `decodeBase32` strict (re-encode compare) + Rust/Go strict tests**; Go `isccClean` port
    deliberately NOT bundled (separate design, 4-fn × 5-form matrix → would blow 6 criteria). Facts
    for the `isccClean` step: Go `base32.DecodeString` silently ignores `\r`/`\n` anywhere — 199's
    re-encode compare makes Go reject a trailing newline until `isccClean` trims it (the step must
    flip that; 199 pinned only an interior-`\n` unit reject). `codec.go:397` is the ONLY
    `b32Encoding.DecodeString` site. `unicode.IsSpace` vs U+001C–U+001F still unprobed — have the
    step check `go doc unicode.IsSpace`. Go cmds without cd: `go -C packages/go test ./...`. After
    that: text-utils trim parity, then 10 seq binding surfaces + small normal issues.
- **200 = Go `isccClean`** (3 files: codec.go, code_iscc.go, code_content_mixed.go). CI red on
    5abda9f was macOS arm64 runner CAPACITY ("not acquired by Runner of type hosted"), not a label
    retirement → no file fix, not a step; tell them apart via the check-run annotations API. Probed
    iscc_core 1.5.0: `iſcc:`/`İSCC:` schemes → Invalid scheme, so Go scheme compare must be
    ASCII-only (EqualFold folds ſ). **Case-fold alias step owns the Codex `%q` whole-input echo in
    Go `decodeBase32` error** (handoff gets overwritten — carry it). After 200: case-fold alias
    (Rust+Go, iai), text-utils trim parity (iai), then Go seq + 9 binding surfaces.
- **201 = case-fold alias fix** (codec.rs L520/L834, lib.rs L229, codec.go L405/L454 + drop `%q`
    echo). CI red again = capacity (annotations API confirmed), not a step. Traps handed over:
    non-ASCII char must sit after the 2-char prefix; `encode_seq` unit case (`gaawdlqng6ſsdpf2`)
    since prefix sites are grep-gated only. Next: Go `EncodeSeq`/`DecodeSeq` (ASCII upper-case
    BEFORE `isccClean`), text-utils trim parity, then 9 binding surfaces.
- **202 = Go `EncodeSeq`/`DecodeSeq`** (codec.go + conformance.go + new seq_test.go). Go traps:
    `MainType/SubType/Version` are `uint8` → raw-`uint32` header helpers needed (SubType 4679
    truncates silently); Go `decodeHeader` silently skips a NON-ZERO padding nibble → only the
    re-encode prefix compare in `readUnit` rejects it; `vectorEntry.Outputs` is a map → seq sections
    need their own loader. Bundled `asciiUpper` zero-alloc fast path (AllocsPerRun==0). Next:
    text-utils trim parity (Rust, iai), then 9 binding surfaces (FFI before .NET/C++).
