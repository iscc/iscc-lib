## 2026-02-27 — Review of: Propagate 7 Tier 1 symbols to Go binding

**Verdict:** PASS

**Summary:** Clean, focused implementation adding 4 package-level constants, a `DecodeResult`
struct, and 3 new `Runtime` methods (`JsonToDataUrl`, `EncodeComponent`, `IsccDecode`) to the
Go/wazero binding. All 30/30 Tier 1 symbols are now accessible from Go. The implementation follows
existing bridge patterns consistently, with thorough WASM memory management and comprehensive tests.

**Verification:**

- [x] `cd packages/go && mise exec -- go test -v -run TestConstants ./...` — PASS
- [x] `cd packages/go && mise exec -- go test -v -run TestJsonToDataUrl ./...` — PASS (both
    variants)
- [x] `cd packages/go && mise exec -- go test -v -run TestEncodeComponent ./...` — PASS
- [x] `cd packages/go && mise exec -- go test -v -run TestIsccDecode ./...` — PASS (valid + invalid)
- [x] `cd packages/go && mise exec -- go test -v -run TestEncodeDecodeRoundtrip ./...` — PASS
- [x] `cd packages/go && mise exec -- go test -count=1 ./...` — PASS (all existing + new tests)
- [x] `cd packages/go && mise exec -- go vet ./...` — clean
- [x] `grep -c` for constants returns 4
- [x] `grep -c 'func (rt \*Runtime)'` returns 48 (≥ 26)
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** No issues identified. Codex confirmed the new Go binding symbols follow existing
allocation/call/free patterns and the added Go tests pass against the embedded WASM module.

**Next:** All 30/30 Tier 1 symbols are now propagated to all 6 language bindings (Python, Node.js,
WASM, C FFI, Java JNI, Go/wazero). The project can move on to other priorities from target.md —
likely addressing the remaining issues in issues.md (dict auto-serialization for `gen_meta_code_v0`,
documentation updates) or other target milestones.

**Notes:** The WASM binary (`packages/go/iscc_ffi.wasm`) was stale and had to be rebuilt locally
despite next.md stating it was already up-to-date. The binary is gitignored (build artifact), so
this didn't affect the commit. The advance agent correctly identified and resolved this
prerequisite. The `IsccDecode` implementation handles the complex WASM sret ABI correctly with
proper cleanup in all error paths (sret dealloc, string dealloc, digest copy before free).
