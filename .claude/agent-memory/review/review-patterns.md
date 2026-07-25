# Detailed Review Patterns

Moved from MEMORY.md to keep it under 200 lines. Referenced from MEMORY.md.

## Documentation Review Patterns

- **C FFI type prefix**: cbindgen config has `[export] prefix = "iscc_"` for types but
    `[fn] prefix = ""` for functions. Code examples must use `iscc_FfiDataHasher`,
    `iscc_IsccSumCodeResult`, etc. The advance agent used short names — always cross-check type
    names in C code blocks against `crates/iscc-ffi/include/iscc.h`
- Always verify documented API names against actual binding source code attributes (`js_name`,
    `#[pyfunction]`, `#[napi(js_name)]`) — next.md specs may have incorrect naming that the advance
    agent faithfully reproduces
- WASM constants have `js_name = "META_TRIM_NAME"` (uppercase) despite Rust function being
    `meta_trim_name()` — this is a known divergence point
- Cross-check version requirements in docs against build config files (e.g., `pom.xml`
    `maven.compiler.source`, `go.mod` go version). Advance agents may introduce version claims that
    don't match actual build requirements (e.g., Java 11+ claimed but pom.xml requires 17+)
- Doc tab conversions: verify WASM `init()` is shown at least once. Each language's return type
    differs (Python/Rust/Go return result structs; Node.js/Java/WASM return plain strings)
- **Count-sensitive docstrings**: "10 gen functions" is correct for library-wide descriptions, but
    conformance tests, benchmarks, and type stubs that reference data.json should say "9" (no
    gen_sum_code_v0 vectors exist). Always verify the actual code scope before accepting numeric
    count changes in docstrings. Blanket find-replace of counts across all files is error-prone
- **External project claims**: When updating counts/coverage for external projects (e.g.,
    iscc-core-ts in ecosystem.md), verify against the function table in the same file. Don't assume
    all projects implement the same set of functions

## Verification Patterns

- `grep -c` counts ALL matching lines including function definitions — when next.md specifies "4
    call sites" but the function name also appears in a definition, expect count = call sites + 1.
    This is a valid pass if the arithmetic checks out
- `grep -c '---' site/llms-full.txt` does NOT reliably count page dividers — doc pages contain
    internal `---` horizontal rules. Use the script's "N pages" stdout as the authoritative check

## Issues Cleanup

- The review agent only cleans up issues resolved in the *current* iteration's advance step. It does
    NOT sweep the full issues.md backlog for stale entries resolved in prior CID loops. This led to
    issues #5-#8 persisting for 4+ iterations after their fixes landed. **Mitigation:** after
    verifying the advance work, also scan issues.md for any other entries that are now resolved
    (check state.md "met" sections against issue descriptions)

## Claim-Probing Recipes (resolved cases, still-live technique)

- **Backend-activation claims — verify the build.rs gating** (iters 117-118, #42 RESOLVED): don't
    trust "flag set → goal met". blake3 WASM SIMD is activated by the `blake3/wasm32_simd` **Cargo
    feature** (`CARGO_FEATURE_WASM32_SIMD` → `blake3_wasm32_simd` cfg → `Platform::WASM32_SIMD`),
    NOT `-C target-feature=+simd128` alone. Honest wiring proof:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` must show
    `wasm32_simd`. Counting `v128` opcodes ALONE is a FALSE POSITIVE (LLVM auto-vectorizes the
    portable path). NUANCE (118): blake3's SIMD fns carry `#[target_feature(enable = "simd128")]`,
    so a no-flag `cargo build --target wasm32-unknown-unknown` still compiles the backend — the
    global RUSTFLAGS only broadens simd128 to the whole crate. Test a no-flag wasm build before
    accepting any "the flag is needed so intrinsics compile" doc claim
- **Decode body-length must be EXACT, not `>= nbytes`** (found iter 119; Go 120, Rust core 121 —
    RESOLVED): a `len(tail) < nbytes` guard silently drops trailing base32 bytes, so `...AB` aliases
    `...ABAA` — codec-wide, NOT ID-specific. Both `IsccDecode` and `iscc_decode` now use a 2-branch
    "too short"/"too long" form (keeps the "too short" message/test intact); all 11 bindings inherit
    the Rust fix. Non-breaking for the Tier 1 symbol — rationale in `decisions.md` 2026-07-24.
    Composite `iscc_decompose` legitimately consumes trailing units — do NOT harden it. Probe
    exact-length rejection with a throwaway test whenever new decode/parse surface lands
- **Docs-site rendering check** (iter 132): `uv run zensical build` reports "No issues found" even
    when an `!!! note` body is mis-indented and degrades to a plain paragraph. Grep the RENDERED
    page — `site/howto/<lang>/index.html` for `<div class="admonition note">` — after any
    admonition/tab edit. `mise run version:check`'s file list IS `scripts/version_sync.py` `TARGETS`
    (21 entries), so doc edits near a dependency line can break it

## Gotchas

- Git log shows iteration numbering resets when a new CID run starts (iteration 12 → iteration 1) —
    this is normal, each `mise run cid:run` starts a new run
- Go via mise requires `mise exec --` prefix — `go` is not on PATH in all environments
- The advance commit is at HEAD (not HEAD~1) when the review hasn't committed yet — use
    `git diff HEAD~1..HEAD` for the advance diff (define-next → advance)
- When HEAD is a previous review commit, the advance is at HEAD~1 and the advance diff is
    `git diff HEAD~2..HEAD~1`. Always verify with `git log --oneline -5` first
