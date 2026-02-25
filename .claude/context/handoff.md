## 2026-02-25 — Create Go how-to guide and add to navigation

**Done:** Created `docs/howto/go.md` with a comprehensive Go how-to guide following the exact
structure from `docs/howto/python.md` and `docs/howto/nodejs.md`. Added the Go entry to
`zensical.toml` navigation after WebAssembly. All function signatures verified against
`packages/go/iscc.go`.

**Files changed:**

- `docs/howto/go.md`: Created Go how-to guide with sections for installation, runtime setup
    (NewRuntime/Close lifecycle), all 9 gen functions (Meta, Text, Image, Audio, Video, Mixed, Data,
    Instance, ISCC-CODE), streaming (DataHasher/InstanceHasher with chunked file reading), text
    utilities (TextClean, TextRemoveNewlines, TextTrim, TextCollapse), algorithm primitives
    (SlidingWindow, AlgMinhash256, AlgCdcChunks, AlgSimhash), conformance testing, and error
    handling
- `zensical.toml`: Added `{ "Go" = "howto/go.md" }` to How-to Guides nav after WebAssembly

**Verification:** All 9 verification criteria pass (file exists, install command present, NewRuntime
documented, GenMetaCodeV0 documented, DataHasher documented, TextClean documented,
ConformanceSelftest documented, nav entry present with correct path). `uv run zensical build`
succeeds with `/howto/go/` in output. All 14 pre-commit hooks pass.

**Next:** Create `docs/howto/java.md` — the remaining documentation gap for binding how-to guides.
Or address tracked issues from issues.md (codec optimization, DataHasher buffer allocation, etc.).

**Notes:** The Go API has a unique `Runtime` lifecycle pattern (NewRuntime/Close) not present in
other bindings — this is prominently documented. `GenIsccCodeV0` in Go does not expose a `bits`
parameter (wide is hardcoded to false). The `bits` parameters in Go are `uint32` (next.md said
`int32` but the actual code uses `uint32`). The streaming examples use explicit `for` loops with
`io.EOF` checks rather than Go's `io.ReadAll` pattern to demonstrate true chunked processing.
