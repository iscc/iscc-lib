## 2026-11-21 — Review of: Realign the specs and crate CLAUDE.md files with the shipped code

**Verdict:** PASS

**Summary:** advance rewrote every statement the audit issue cites in four specs and four per-crate
`CLAUDE.md` files, and no code file changed. I read each rewritten sentence against the code it
names and all hold, except one generalization in the WASM spec that I corrected. The audit issue is
closed and deleted.

**Verification:**

- [x] `grep -rn 'iscc_lib::Error\b' crates/*/CLAUDE.md packages/*/CLAUDE.md` prints nothing — and
    `git grep` finds the old name nowhere else in the tree
- [x] `grep -n 'io.Reader\|exist only on .develop'` on `go-bindings.md` and
    `grep -n 'genMetaCodeV0\|Option<u32>'` on `wasm-bindings.md` print nothing
- [x] `grep -n 'has only .V0\|currently fails'` on `rust-core.md` and `grep -n 'still hold the GIL'`
    on `python-bindings.md` print nothing
- [x] `grep -n 'Unlike most other language packages' packages/go/CLAUDE.md` prints nothing and
    `grep -n 'packages/go/v' packages/go/CLAUDE.md` prints line 126
- [x] `uv run scripts/check_spec_paths.py` exits 0 ("15 specs") and `mise run check` exits 0 (all
    hooks passed, no file modified)
- [x] `git show --stat` of the advance commit lists the 8 scoped files plus `handoff.md` and advance
    memory, no code file

Checked beyond the list, all against the tree:

- `rust-core.md`: `Version` has `V0`/`V1` with `#[non_exhaustive]`; the private `validate_version`
    is called from `encode_header` and `decode_header`. Live on the Python binding:
    `iscc_decode('ISCC:MAIGHFECJMOPMIAB')` gives `(MT.ID, ST.NONE, VS.V1, 0, …)`, `iscc_decompose`
    gives `['MAIGHFECJMOPMIAB']`, and a Version-1 Meta header is rejected
- `python-bindings.md`: the five `.detach(` sites sit in `gen_text_code_v0`, `gen_video_code_v0`,
    `gen_video_code_v0_flat`, `soft_hash_video_v0_flat` and `soft_hash_video_v0`; meta, audio and
    mixed have none; the module sets `gil_used = true`
- `go-bindings.md`: no tag ever held `EncodeIsccID` or `DecodeIsccID` (checked every tag), and
    `GenIsccIDV1` is first in `v0.6.0`. advance was right to depart from next.md, which said the old
    symbols were "removed in 0.6.0"
- `wasm-bindings.md`: the code block equals `gen_meta_code_v0` in `crates/iscc-wasm/src/lib.rs`
    character for character; all nine gen functions and `gen_iscc_id_v1` return `String`,
    `gen_sum_code_v0` returns `WasmSumCodeResult`; serde is used for inputs only
- `packages/go/CLAUDE.md`: every non-test `.go` file has a sibling `_test.go`; `release.yml` has
    `workflow_dispatch` as its only trigger and creates both tags in `prepare-release`, which runs
    only with a `version` input

**Issues found:**

- Fixed by me (sub-spec fact): `wasm-bindings.md` said "every export keeps its snake_case Rust
    name". The five constant getters carry a `js_name` (`META_TRIM_NAME` and so on). The sentence
    now says exported functions keep their name and names the getters as the exception
- Filed `low`, untriaged: "`crates/iscc-py/CLAUDE.md` still lists test files one by one". advance
    reported it; the step did not touch it, so it is filed and not fixed
- Closed and deleted: "Specs and crate CLAUDE.md files describe shipped work as pending and name
    missing items"

**Codex review:** no actionable findings. It confirmed that the updates match the implementation and
the release tags, and that the spec-path check and its tests pass.

**Next:** `conformance_selftest`'s single section runner (the `[audit]` issue of that name): one
generic runner in `crates/iscc-lib/src/conformance.rs` and the same helper in
`packages/go/conformance.go`, with `.crap-baseline.json` moved in the same step (the CRAP gate is
CI-only, and moving lines below an edit point reds it). Alternatives of similar size:

- the batch of small user-docs entries (`KAA` prefix, pkg-config version, `docs/development.md`
    "Publish", `llms.txt` coverage) — independent, docs-only
- "`@iscc/wasm` docs show a Node.js setup the published package cannot run" — needs a probe against
    a `--target web` release build

**Notes:**

- After this step issues.md holds 11 `normal` and 7 `low` entries (290 lines)
- learnings.md stays at 200 lines, its budget; I added nothing. A new bullet needs an equal cut
- For define-next: a claim about release history ("shipped in", "removed in") is checked per tag
    with `git grep '<pattern>' <tag> -- <path>` before it goes into next.md
- The remaining text of the ISCC-IDv1 section in `rust-core.md` (Part 2 and below) was not part of
    the audit issue and was not re-read sentence by sentence in this review
