## 2026-03-01 — Review of: Add gen_sum_code_v0 to Node.js bindings

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation that adds `NapiSumCodeResult` struct and
`gen_sum_code_v0` napi function to the Node.js binding crate. The function correctly reads a file
path, computes Data-Code + Instance-Code in a single pass, and returns a structured object. All 6
required test cases pass, quality gates clean, no scope violations.

**Verification:**

- [x] `cargo build -p iscc-napi` — compiles without errors
- [x] `cargo clippy -p iscc-napi -- -D warnings` — clean
- [x] `cd crates/iscc-napi && npm run build && npm test` — 132 tests pass (126 existing + 6 new
    gen_sum_code_v0), 0 failures
- [x] `gen_sum_code_v0` callable from JavaScript, returns object with `iscc`, `datahash`, `filesize`
- [x] `mise run check` — all 14 hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- (none — minor test cleanup applied directly in review)

**Codex review:** Two advisory findings, both addressed or accepted:

1. u64→i64 filesize cast (P2): Codex flagged `result.filesize as i64` as potentially lossy. This is
    a deliberate design choice — napi-rs lacks u64 support, and i64 covers files up to 2^63 bytes
    (~8 EB), far beyond any practical file size. The JS Number.MAX_SAFE_INTEGER (~9 PB) is the real
    boundary. Accepted as-is — consistent with the implementation notes in next.md.
2. Unused `instResult` variable and misleading test name (P3): Fixed during review — removed dead
    variable, renamed test to accurately describe its assertions.

**Next:** Propagate `gen_sum_code_v0` to WASM bindings (`crates/iscc-wasm/`). Key design difference:
WASM has no filesystem access, so `gen_sum_code_v0` needs to accept `Uint8Array` bytes directly (or
use the streaming DataHasher + InstanceHasher pattern). Check if the Rust core exposes a data-based
variant, or if a WASM-specific wrapper composing from data+instance is needed. After WASM, continue
with C FFI, Java, and Go in that order per issue #15.

**Notes:** Issue #15 is progressively being resolved — Rust core ✅, Python ✅, Node.js ✅. Four
bindings remain: WASM, C FFI, Java, Go. State.md still shows Node.js as "partially met" — the next
update-state agent will update it to reflect the 132-test milestone.
