## 2026-11-18 — Review of: Go `decodeBase32` rejects spare-bit aliases; Rust and Go tests pin strict base32

**Verdict:** PASS_WITH_NOTES

**Summary:** Go `decodeBase32` now re-encodes the decoded bytes and rejects any input that differs
from its upper-cased form. That closes spare-bit aliases and the interior `\r`/`\n` that
`encoding/base32` skips. New Rust (`test_strict_base32.rs`, 7 tests) and Go (3 tests, one bad input
per subtest) suites pin the issue's 3 rejects with a `base32 decode error` check and the 2 canonical
accepts. The diff stays in scope: 3 files, Rust `src/` untouched. The `[human]` issue is resolved
and deleted. The spec already states the rule, so it needs no edit.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_strict_base32`: 7 passed. All 3 rejects carry the message
    check, plus the 2 canonical accepts (the expected values match my own probe). It also passes
    under all 3 CI feature-matrix configs
- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` ok. `go vet` is clean and `gofmt -l` is
    empty
- [x] `-run 'Base32|Strict'`: the 3 `IsccDecode` reject subtests, `decodeBase32` interior `\n` (plus
    `\r`, spare bits, padding) and the 2 canonical accepts all pass
- [x] `grep -l GAAAAAAAAB …` lists both files
- [x] `cargo test -p iscc-lib`: all 7 binaries green (297 lib). `mise run check` exits 0, and so
    does `cargo clippy -p iscc-lib --all-targets -D warnings`

**Issues found:**

- **Pre-existing, filed as `normal`:** both decoders upper-case with Unicode rules before decoding.
    Non-ASCII letters therefore alias canonical codes: `…CNGı` (U+0131) decodes in Rust and Go, and
    `ß`/`ﬁ` decode in Rust only. iscc-core rejects all of them (ASCII-only `b32decode`). I probed
    the reference, Rust and Go live, except the Go side of `ß`/`ﬁ` (advance's probe). The Go
    re-encode compare runs after `ToUpper`, so it cannot catch this. See issues.md "Base32 decoding
    accepts non-ASCII case-fold aliases".
- **Expected side effect:** Go now rejects `ISCC:GAAAAAAAAA\n`, which it used to accept by accident
    and which the reference strips. I noted it in the Go `iscc_clean` issue, so the port repairs it.
- Nit: the Go error quotes the whole input with `%q`, so it grows with the input (see Codex below).
    Rust reports a position instead.

**Codex review:** one P2. `codec.go:410` `%q` copies the entire input into the error. A long
`IsccDecompose` sequence with one bad character gives a megabyte-scale message. Advisory only: it
does not break the contract. Fold a reason-only or offset-style message into the `isccClean` step,
which touches the same function anyway.

**Next:** The Go `isccClean` port (issues.md "Go codec input cleaning diverges…"). It is now more
urgent, because it also repairs the trailing-newline rejection above. Use one shared helper with the
Python strip set including U+001C–U+001F, dash rules, a case-insensitive scheme and an empty guard,
routed through all 4 Go sites. After that, the new case-fold alias issue: a small change on both
sides, but the Rust part touches the iai-gated codec path, so it needs `bench:iai:check`. Then Go
`EncodeSeq`/`DecodeSeq`.

**Notes:**

- Not re-run: advance's mutation check. The issue's table already shows Go aliasing before the fix.
- `cargo clippy -p iscc-lib --no-default-features --all-targets` fails on `benches/` (unresolved
    `gen_meta_code_v0`). This predates the step, and CI's no-default clippy line has no
    `--all-targets`, so it is not a CI gap. Ignore it.
- Go tests need no fixture changes. No hand-typed Go fixture had non-zero spare bits.
