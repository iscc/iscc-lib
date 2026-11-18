## 2026-11-18 — Review of: Python `encode_seq` / `decode_seq` binding with an `iscc_core` differential test

**Verdict:** PASS_WITH_NOTES

**Summary:** The Python binding now exposes ISCC-SEQ. `encode_seq` returns `bytes` and `decode_seq`
returns `list[str]`. The wrapper matches the reference's type contract, and the docstrings state
that base32 decoding is strict. The diff is small and in scope: 3 non-test files plus the new
`tests/test_seq.py`. Review added a shared-rejection differential (19 cases, 66 tests in total) and
found one pre-existing core cleaning divergence, now filed as an issue.

**Verification:**

- [x] `maturin develop -m crates/iscc-py/Cargo.toml` succeeds: built and installed.
- [x] `uv run pytest tests/test_seq.py` passes (47 from advance, 66 after review's addition). The
    full `uv run pytest` passes: 477 tests, 496 after the addition.
- [x] `encode_seq` returns `bytes`, and `decode_seq(bytearray)` round-trips: one-liner exits 0.
- [x] `{'encode_seq','decode_seq'} <= set(__all__)`: exits 0.
- [x] `uv run ty check` exits 0, re-run after the review edit.
- [x] `mise run check` exits 0. `cargo clippy -p iscc-py -- -D warnings` and the workspace
    `--all-targets` clippy are clean.
- Probes:
    - No suppressions in `@{upstream}..HEAD`.
    - CI's Python job runs `uv sync --group dev`, and 5 other test files already import `iscc_core`,
        so `test_seq.py` will collect in CI.
    - Reject-parity probe across 18 bad inputs: both libraries raise the same class, except the
        whitespace case below. `binascii.Error` is a subclass of `ValueError`.

**Issues found:**

- **Fixed in review:** the spec's Python criterion (`rust-core.md` "Verified when") requires that
    both sides reject the same inputs. next.md asked only for the base32 divergence and canonical
    agreement. Review added `test_differential_{encode,decode}_rejects_same_inputs`:
    - 11 encode cases;
    - 8 decode cases, including all of IEP-0020 Table 2.
- **Filed (`normal`):** `iscc_clean` trims fewer whitespace characters than iscc-core.
    - Python's `str.strip()` also strips U+001C–U+001F; Rust's `trim` keeps them.
    - So `"\x1eISCC:GAAAAAAAAA"` succeeds in the reference and fails in iscc-lib, for `encode_seq` and
        `iscc_decode` alike.
    - The cause is core, pre-existing since iteration 191; this diff did not introduce it.
    - Until it is fixed, the spec's Python "reject the same inputs" checkbox stays unmet. Do not tick
        it.
- Nit, not filed: a list holding both a malformed unit and a non-string, e.g. `["", 1]`, raises
    `TypeError` here and `ValueError` in the reference. PyO3 extracts the whole list before the core
    checks any unit; the reference checks units in order.
- Stale and deferred to the Tier-1 sweep: `crates/iscc-py/CLAUDE.md` still says "27 #[pyfunction]s".
- Trimmed on sight: the stale `codec.py` line references in the Go cleaning issue now point to the
    1.5.0 lines.

**Codex review:** 2 findings, both confirmed by probe.

- P2: no shared-rejection differential. Resolved in review (see above).
- P3: U+001C–U+001F trimming parity. Filed as an issue. The cause is in core `iscc_clean`, not the
    binding.

**Next:**

- Strongest candidate: the Go `decodeBase32` spare-bit fix plus Rust and Go strict-base32 tests
    (`normal` `[human]`). It and the Go `isccClean` port are both prerequisites for Go seq. Folding
    the U+001C–U+001F set into the Go port is natural, since the new issue says Go must strip the
    same set.
- Seq fan-out on surfaces with no prerequisite: Node, WASM, Ruby, JNI.
    - Copy this step's test shape: fixture conformance, round trip, one bad input per case, and the
        explicit strict-base32 divergence.
    - Only Python has a live reference, so the other surfaces assert the fixture and the strict
        rejections.
- The C FFI seq step must regenerate `iscc.h` and `NativeMethods.g.cs` in the same step.
- The core whitespace fix touches the iai-gated `iscc_clean`. Scope `mise run bench:iai:check` with
    it.

**Notes:**

- ISCC-SEQ is bound on 1 of 11 surfaces (Python). The Tier-1 count stays 33 in docs until all 11
    land; then one sweep takes it to 35.
- The `cid(update-state)` / define-next / advance commits in this batch are context and memory files
    only. Nothing else is unpushed.
