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
- **203 = text-utils U+001C–U+001F parity, Rust + Go bundled** (utils.rs, codec.rs
    `is_py_strip_char` → `pub(crate)`, utils.go; 3 files). No conformance vector moves (`text_clean`
    drops Cc first in `gen_meta_code_v0`), so only new tests prove it: one case per `text_trim`
    branch, Python live-strip-set differential, Go literal code points. `text_clean` trims +
    `code_meta.go:102` left out (input already Cc-free). iai tools confirmed installed
    (`command -v`). Next: 9 seq binding surfaces (FFI before .NET/C++) + small normal issues (py
    docstring, wasm metadata, Ruby json/locale, uv.lock, c-ffi-api names).
- **204 = napi + WASM seq together** (2 source files; NOT the fan-out valve — wrappers differ: napi
    `Vec<String>`/`Buffer`, wasm `JsValue`+serde like `gen_iscc_code_v0`). napi
    `index.d.ts`/`index.js` are gitignored (not tracked) → grep the regenerated d.ts after
    `npm --prefix crates/iscc-napi run build:debug`. Seq vectors: 4+4, `bytes:<hex>`, no error cases
    → negatives from tests (empty list/unit, spare bits, `ſ` alias, bare string, empty and truncated
    bytes). Next: FFI seq (`iscc.h` + `NativeMethods.g.cs` regen, C ownership of the returned
    buffer/string array), then .NET/C++, JNI, Ruby, UniFFI (Swift+Kotlin hard-code counts), then the
    33→35 doc sweep; small `normal` issues interleaved.
- **205 = C FFI seq** (lib.rs + Cargo.toml `serde_json` dev-dep + test_iscc.c; regen `iscc.h` +
    `NativeMethods.g.cs`). FFI had NO data.json harness (only `iscc_conformance_selftest`) → Rust
    tests walk data.json seq sections via `include_str!`. Decision 2026-07-27 (generated C vector
    header) applied to the removed Unicode fixture; the header/script no longer exist → C test pins
    one round trip for ABI/ownership only. Valgrind kept as a note, not a criterion (pre-existing
    leaks unknown). Header-freshness criterion = regen to /tmp + `diff` (CI's `git diff` fails
    pre-commit). Next: .NET + C++ wrappers, JNI, Ruby, UniFFI, then 33→35 sweep.
- **206 = .NET seq** (IsccLib.cs only + tests; `NativeMethods.g.cs` already carries both symbols).
    `dotnet-bindings.md` has no seq section → rust-core.md is canonical. Traps: C# `fixed` on an
    empty `byte*[]` yields NULL → FFI "units must not be NULL" masks core "at least one ISCC-UNIT"
    (use a stack sentinel); `iscc_decode_seq` checks len 0 before NULL, so no sentinel is needed.
    Pin loop left copied (3rd copy). Next: C++ `iscc.hpp` (RAII on buffer + string array), JNI,
    Ruby, UniFFI (Swift+Kotlin hard-code counts), then the 33→35 sweep.
