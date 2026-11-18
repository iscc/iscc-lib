> **HUMAN REVIEW REQUESTED**: Rust `encode_seq` now rejects non-ASCII case-fold aliases
> (`gaawdlqng6ſsdpf2`, also `ı`/`ß`/`ﬁ`) that iscc-core 1.5.0 `encode_seq` accepts, because the
> reference calls Unicode `unit.upper()` first (`codec.py:503`). `iscc_decode` rejects them on both
> sides, so that part matches the reference. Review approved the `encode_seq` divergence as an
> extension of your strict-base32 decision (decisions.md 2026-11-18, which rejected a lenient
> `encode_seq`). The divergence is pinned in `tests/test_seq.py`. But `rust-core.md` names only
> padding and spare bits as exceptions. Please confirm, and update the two spec passages, or reject
> it, which restores Unicode upper-casing at `codec.rs:834`. Details are in the issues.md entry.

## 2026-11-18 — Review of: ASCII-only upper-casing in Rust and Go base32 decoding

**Verdict:** PASS_WITH_NOTES

**Summary:** Rust's three Unicode upper-casing sites now use `to_ascii_uppercase()`. Go's two sites
now use an `asciiUpper` helper. Each change is a small same-line swap, with alias/twin tests (one
input per assertion) that advance proved by reverting each site. The issue entry claimed iscc-core
rejects every alias. That holds for `iscc_decode`, `iscc_normalize` and `iscc_decompose` (probed),
but not for `encode_seq`. So the `codec.rs:834` swap creates a new divergence, escalated above.

**Verification:**

- [x] `cargo test -p iscc-lib`: 7 suites, 0 failed. `test_strict_base32.rs` has 13 tests: 4
    `iscc_decode` aliases rejected and their twins accepted, 2 `encode_seq` units rejected and their
    twins accepted. `--no-default-features` is also green (246 + 17 + 28 + 44 + 13 + 10)
- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes. `go vet` is clean.
    `gofmt -l packages/go` prints nothing
- [x] The `to_uppercase()` and `strings.ToUpper` greps both print nothing. A wider grep over
    `crates/*/src` and `packages/go` finds no Unicode upper-casing left on any codec path
- [x] `mise run bench:iai:check` exits 0 (16 benchmarks within 10%). `.iai-baseline.json` is
    untouched
- [x] `cargo clippy -p iscc-lib -- -D warnings` is clean. The workspace `--all-targets` run also
    exits 0
- [x] `mise run check` exits 0
- [x] Probe: iscc-core 1.5.0 rejects all 4 aliases in `iscc_decode`, `iscc_normalize` and
    `iscc_decompose`, but `encode_seq` accepts all 4 (`300161ae0d37a521bcba`)

**Issues found:**

- `encode_seq` now diverges from the reference on non-ASCII case-fold aliases, and the spec does not
    list this exception. Filed as a `normal` issue and escalated. Review pinned both sides in
    `tests/test_seq.py` `STRICT_BASE32_DIVERGENCES` (rows `long_s_case_fold` and
    `sharp_s_case_fold`): iscc-core accepts, iscc-lib rejects. This needed a rebuilt binding
    (`maturin develop --release`). 82 passed; ruff, `S`/`C901` and `ty` are clean
- Go `asciiUpper` always allocates twice. `strings.ToUpper` returned an all-uppercase input without
    allocating, so canonical codes pay for the copy (Codex measured 26 → 34 allocations per
    `IsccDecode`). Not filed: the Go port has no perf gate. Suggested fix: check
    `strings.ContainsAny(s, "abcdefghijklmnopqrstuvwxyz")` and return `s` unchanged when false
- Resolved and deleted: "Base32 decoding accepts non-ASCII case-fold aliases"

**Codex review:** (1) Go allocation regression, noted above. (2) The `encode_seq` reference
divergence was confirmed by probe, then pinned, filed and escalated. Codex ran on the main commit
`536e461`; HEAD `66ec2ee` only changes the handoff.

**Next:** Go `EncodeSeq`/`DecodeSeq`. Follow the current Rust rule: upper-case each unit with
`asciiUpper` **before** `isccClean`, which the strict decision requires of Go. If Titusz rejects the
divergence, Rust and Go change together later. Port the `STRICT_BASE32_DIVERGENCES` inputs,
including the two case-fold rows, into the Go seq tests. Add the `asciiUpper` fast path in that
step, because `EncodeSeq` calls it once per unit. After that comes the text-utils U+001C–U+001F
parity issue (iai-gated).

**Notes:**

- Pre-push suites run by review before the push: full `pytest` passed 512, and
    `cargo test --workspace` passed 18 suites with 0 failed.
- decisions.md was rotated with `cid.archive_overflow` after the new entry. Stage
    `decisions-archive.md` with it.
- The red CI on `e8130a8` is a macOS arm64 runner-capacity cancel, not caused by the tree
    (define-next checked the annotations).
