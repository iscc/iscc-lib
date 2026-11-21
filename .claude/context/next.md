# Next Work Package

## Step: Realign the specs and crate CLAUDE.md files with the shipped code

## Goal

Pick up the `[audit]` issue "Specs and crate CLAUDE.md files describe shipped work as pending and
name missing items": rewrite each cited statement in the present tense so it matches the tree.
define-next and advance read these files as truth, so every stale claim risks becoming scoped work.

## Alternatives Considered

- **Chosen:** spec/CLAUDE.md realignment — docs-only, no gate risk, and it pays back on every later
    step (handoff names it as the highest-leverage candidate); the last three steps were code/CI.
- **Rejected:** `conformance_selftest` single section runner (Rust + Go) — a core change that also
    has to move the CI-only CRAP baseline in the same step; it is worth its own iteration, next.
- **Rejected:** batching the small user-docs entries (`KAA`, pkg-config, `development.md`,
    `llms.txt`) — independent and cheap, but the agents' own source of truth comes first.

## Scope

Audit refactor, file budget **8** (all are docs/specs; the realignment decision of 2026-11-20 means
no escalation):

- **Modify**: `.claude/context/specs/rust-core.md`, `.claude/context/specs/python-bindings.md`,
    `.claude/context/specs/go-bindings.md`, `.claude/context/specs/wasm-bindings.md`,
    `crates/iscc-napi/CLAUDE.md`, `crates/iscc-py/CLAUDE.md`, `crates/iscc-wasm/CLAUDE.md`,
    `packages/go/CLAUDE.md`
- **Reference**: `crates/iscc-lib/src/codec.rs` (`Version`, `V1`, `(Id, V1)`),
    `crates/iscc-lib/src/lib.rs:51` (`IsccError`), `crates/iscc-py/src/lib.rs` (`.detach(` sites),
    `crates/iscc-wasm/src/lib.rs:181-191` (`gen_meta_code_v0` shape), `packages/go/*.go`,
    `.github/workflows/release.yml:110-116` (Go module tag)

## Not In Scope

- The user-facing docs issues (`docs/` pages, READMEs, `llms.txt`) — separate entries.
- Rewriting specs beyond the cited statements, or touching `notes/` (never swept).
- Adding a file list anywhere — the 2026-11-21 decisions forbid file lists in per-crate `CLAUDE.md`
    files and aggregate lists in specs.
- Any code change; if a statement cannot be made true without one, leave it and say so in the
    handoff.

## Implementation Notes

Per cited item (all re-confirmed present at HEAD):

- `rust-core.md` ~:360-371 (ISCC-IDv1 "Part 1"): `Version` "has only `V0`" and the "every surface
    currently fails" block with the `ValueError` output. Rewrite as the shipped state: `Version` has
    `V1`, the codec admits `(Id, V1)`, and `iscc_decode`/`iscc_decompose` decode an ISCC-IDv1 (keep
    the reference `iscc_core.iscc_decode` example; drop the failing-output block and "can land as
    its own work package"). Read the codec to state the admitted pairs exactly.
- `python-bindings.md` → "GIL Release for Text/Video Compute Paths" (:294-): says text/video "still
    hold the GIL" and "Wrap the core calls…". Rewrite as present tense (they run inside `py.detach`;
    keep the `extract_frame_sigs` caveat as the constraint that holds). Check the actual `.detach(`
    sites before writing which functions detach.
- `go-bindings.md`: :93 "Streaming functions also accept `io.Reader`" — delete (no `io.Reader` in
    `packages/go/*.go`). Also :13 and :217 list `io.Reader` among the API conventions — drop it
    there too. :136 "Both symbols exist only on `develop`…" — the rename shipped in 0.6.0; say the
    old symbols were removed in 0.6.0 (keep the field-extraction guidance; drop the "update issue
    #43 when this lands" sentence if it reads as pending work).
- `wasm-bindings.md` → "Rust Bridge Layer" (:37-): the sketch shows `js_name = "genMetaCodeV0"`,
    `bits: Option<u32>`, a serde `JsValue` result. Replace with the real shape: snake_case exports,
    optional scalars as `JsValue` with `unchecked_optional_param_type` and an `opt_u32` check, and
    gen functions returning the `iscc` string (`gen_sum_code_v0` returns `WasmSumCodeResult`). Copy
    from `crates/iscc-wasm/src/lib.rs`, do not paraphrase.
- napi :36, :66; py :66; wasm :43 `CLAUDE.md`: `iscc_lib::Error` → `iscc_lib::IsccError`.
- `packages/go/CLAUDE.md`: remove the per-file test lists under "Test Patterns" (:95-121) and state
    the conventions against file-name patterns instead (decision 2026-11-21; e.g. "`code_*_test.go`
    read …", "`data_edge_test.go` …" is fine as a rule on a name, an enumerated unit-test list is
    not). :146 "Unlike most other language packages… wrap `iscc-ffi`" — only C++ and .NET wrap the C
    FFI; the others are native bindings (PyO3, napi, wasm-bindgen, JNI, Magnus, UniFFI). Publishing
    (:133-135): the proxy needs the `packages/go/vX.Y.Z` tag, which `release.yml` creates on a
    `workflow_dispatch` with a `version` input — no tag push triggers anything.
- Specs are gated by `scripts/check_spec_paths.py` (prek hook): every backticked repo path must be
    tracked. Run `mise run format` before committing (mdformat wraps at 100).

## Verification

- `grep -rn 'iscc_lib::Error\b' crates/*/CLAUDE.md packages/*/CLAUDE.md` prints nothing
- `grep -n 'io.Reader\|exist only on .develop' .claude/context/specs/go-bindings.md` prints nothing,
    and `grep -n 'genMetaCodeV0\|Option<u32>' .claude/context/specs/wasm-bindings.md` prints nothing
- `grep -n 'has only .V0\|currently fails' .claude/context/specs/rust-core.md` and
    `grep -n 'still hold the GIL' .claude/context/specs/python-bindings.md` print nothing
- `grep -n 'Unlike most other language packages' packages/go/CLAUDE.md` prints nothing and
    `grep -n 'packages/go/v' packages/go/CLAUDE.md` prints a line
- `uv run scripts/check_spec_paths.py` exits 0 and `mise run check` exits 0
- `git show --stat` of the advance commit lists only the 8 scoped files plus `.claude/context/` and
    agent-memory files (no code file changed)

## Done When

Every statement the audit issue cites matches the tree and all verification checks pass.
