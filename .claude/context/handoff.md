## 2026-11-18 — ASCII-only upper-casing in Rust and Go base32 decoding (non-ASCII case-fold aliases)

**Done:** Rust's three Unicode `to_uppercase()` sites are now `to_ascii_uppercase()`:
`decode_base32`, `encode_seq`'s pre-clean upper-case, and the `iscc_normalize` prefix check. Go's
two `strings.ToUpper` sites (`decodeBase32`, `checkPrefix`) now call an `asciiUpper` helper that
sits next to `isIsccScheme`. Go's non-canonical error no longer echoes the input with `%q`. As a
result, `ı`/`ſ`/`ß`/`ﬁ` reach the base32 decoder and fail there with `base32 decode error`.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: L518 doc "ASCII case-insensitive", L520 + L834 swapped to
    `to_ascii_uppercase()`, L831 comment "ASCII-uppercase first". All are same-line edits (4+/4−),
    so no lines move.
- `crates/iscc-lib/src/lib.rs`: L229 prefix check swapped to `to_ascii_uppercase()` (1+/1−).
- `packages/go/codec.go`: `decodeBase32` and `checkPrefix` call `asciiUpper`, and the doc says
    "ASCII". The non-canonical error is now `iscc: base32 decode error: non-canonical encoding`.
    Added `asciiUpper` (maps `a`–`z` only) after `isIsccScheme`.
- `crates/iscc-lib/tests/test_strict_base32.rs`: added the `CASE_FOLD_ALIASES` table (4 alias/twin
    pairs from next.md) and 6 tests:
    - `iscc_decode` rejects each alias and accepts each twin
    - the bare `decode_base32` rejects `ſ` and `ı`
    - `encode_seq` rejects `gaawdlqng6ſsdpf2` and `gaat5fınqwrc4hqv`, one test each
    - `encode_seq` accepts the lowercase twins `gaawdlqng6ssdpf2` and `gaat5finqwrc4hqv`
- `packages/go/codec_test.go`: added the `caseFoldAliases` table (the same 4 pairs), a reject test
    and an accept test (one subtest per input), plus `TestCodecAsciiUpper`.

**Verification:**

- `cargo test -p iscc-lib --test test_strict_base32`: 13 passed, 0 failed.
- Rust mutation runs (temporary `sed`, then restored; `git diff --stat` shows 4+/4− after):
    - Revert L834 alone: the 2 `encode_seq` reject tests fail.
    - Revert L520 alone: 4 tests fail (`iscc_decode` aliases, bare decoder, both `encode_seq`).
    - So both sites have their own gate. L229 (and Go `checkPrefix`) still reject a non-ASCII prefix
        either way; only the message differs (`invalid prefix`), so the grep is their gate.
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...`: ok. The targeted run gives 23 PASS lines.
- Go mutation run: restoring `strings.ToUpper` in `decodeBase32` fails exactly the `ı` and `ſ`
    subtests. This matches next.md: in Go, `ß`/`ﬁ` already errored before the fix.
- `go vet ./...` is clean. `gofmt -l packages/go` prints nothing.
- `grep -rn "to_uppercase()" crates/iscc-lib/src` and
    `grep -n "strings.ToUpper" packages/go/codec.go` both print nothing.
- `cargo clippy -p iscc-lib -- -D warnings`: clean.
- `mise run bench:iai:check`: EXIT=0, "16 benchmark(s) within 10% of baseline".
    `iscc_code.four_units` is −5.45%, `mixed_code.two_codes` −4.64%, `meta_code.*` about −4.6%.
    Everything else is between −7.7% and +0.05%. `.iai-baseline.json` and `.crap-baseline.json` are
    untouched (`git diff --quiet` passes).
- `cargo test -p iscc-lib`: 7 suites, all ok (297 + 44 + 28 + 22 + 17 + 13 + 1), 0 failed.
- `mise run check`: exit 0 on the third run. Run 1 flagged `standardrb-fix` and run 2 flagged
    `mdformat` as "files were modified". Run 1's cause is likely my handoff/memory writes landing
    mid-run: no Ruby file changed, and the hook passed on runs 2 and 3. Run 2 was mdformat
    rewrapping this handoff, a legitimate fix.

**Next:** Go `EncodeSeq`/`DecodeSeq`. `EncodeSeq` must upper-case each unit **before** `isccClean`,
using the `asciiUpper` helper added here, not `strings.ToUpper`. `DecodeSeq` takes bytes and does no
cleaning. After that comes the text-utils U+001C–U+001F parity issue (it is iai-gated).

**Notes:**

- I did not touch the issues.md entry, per the ledger rule. Review should resolve the "Base32
    decoding accepts non-ASCII case-fold aliases" entry.
- No CRAP baseline refresh was needed. The Rust edits are same-line swaps, so no line below an edit
    point moved. The Go file is not under the CRAP gate.
- The Rust "Malformed"/"Invalid scheme" error echoes are unchanged, as next.md scoped.
