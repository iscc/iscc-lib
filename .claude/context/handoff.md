## 2026-11-22 — Review of: The first hasher `finalize()` consumes on every surface, whatever rejects the argument

**Verdict:** PASS_WITH_NOTES

**Summary:** WASM, napi, JNI, Python and Ruby hashers now take the inner hasher before any argument
conversion or check, one commit per surface, each with consumption tests (binding-rejected and
core-rejected `bits`, one bad case per fresh hasher). All 7 criteria pass, exception classes and
messages are unchanged apart from the napi non-number message that next.md asked for, and the
fan-out made no decision the ruling left open. Review fixed two small things (see Notes).

**Verification:**

- [x] WASM: `wasm-pack test --node crates/iscc-wasm --features conformance` — 11 + 8 + 136 passed,
    the 10 `*_consumes` tests among them
- [x] napi in a `git archive HEAD` copy under `/tmp`:
    `npm ci && npx napi build --platform && npm test` — 243 pass, 0 fail (before and after the
    review's edit). Built `index.d.ts` types both methods as
    `finalize(bits?: number | undefined | null): string`
- [x] JNI: `cargo build -p iscc-jni && mvn -f crates/iscc-jni/java/pom.xml test` — exit 0
- [x] Python: `maturin develop`, then `uv run pytest tests/` — 924 passed; `uv run ty check` clean
- [x] Ruby: `LANG=C.UTF-8 mise exec -- bundle exec rake compile test` — 175 runs, 0 failures
- [x] `cargo clippy --workspace --all-targets -- -D warnings` and `mise run check` exit 0 (both run
    again after the review's edits)
- [x] `grep -il "even when"` over the 5 named doc files lists all 5

Probes beyond next.md (3): napi `finalize(undefined | null)` keeps the default, and `1.5`, `NaN`,
`2**32+64`, a BigInt, an object, a boolean and `"64"` all throw and leave the hasher finalized.
Python keeps `inspect.signature` as `(self, /, bits=64[, wide=False, add_units=False])`, and
`finalize(None)` is still a `TypeError`. A conversion that re-enters the hasher (Python `__index__`,
Ruby `to_int` calling `finalize`/`update`) raises an ordinary error on both, no panic.

**Issues found:**

- Filed `low`, untriaged: "No test pins a failed first `finalize` consumes on C FFI, C++, .NET,
    Swift and Kotlin". These surfaces comply by code order (confirmed by reading), but their suites
    only test a second `finalize` after a successful one, while the how-to pages now state the rule
- Not filed (not false, only less explicit): `docs/howto/java.md` (~278), `docs/c-ffi-api.md` (~742)
    and the two `*HasherFinalize` Javadoc blocks in `IsccLib.java` state single use without "even
    when it fails"
- A call the language rejects before the method runs (Python or Ruby unknown keyword, wrong arity)
    leaves the hasher usable. Review reads that as outside the ruling and recorded it in
    decisions.md; Titusz may see it differently

**Codex review:** ran over the whole advance range (`--base` the define-next commit). No actionable
findings.

**Next:** the `alg_minhash_256([])` error (`normal`, core API break, approved), alone as next.md
planned; its `data-edge.json` section follows as a fan-out. Then `bits` above 256. That step edits
every hasher `finalize` and the API pages on all 12 surfaces anyway, so it can add the missing
"failed `finalize`, then `finalize(64)` reports finalized" assertion on C FFI, C++, .NET and UniFFI
(closes the new `low` issue if Titusz promotes it) and the three doc spots named above.

**Notes:**

- Resolved and deleted: "`@iscc/wasm` hasher `finalize()` stays usable after some rejected
    arguments". Two `normal` issues remain (`alg_minhash_256`, `bits` above 256)
- Review edits (behaviour-neutral): `crates/iscc-napi/src/lib.rs` reads the number with the safe
    `f64::from_unknown(value)` instead of `unsafe { value.cast::<f64>() }` (same body in napi 3.11,
    and the crate has no `unsafe` block again); a 118-column doc comment in
    `crates/iscc-wasm/src/lib.rs` re-wrapped; `crates/iscc-napi/CLAUDE.md` got the `Option<Unknown>`
    row that advance named as a follow-up
- Python's `Deferred` adds the same parameter-name note as PyO3 0.29 does (`add_note`, ignored when
    it fails, as on Python 3.10 where the method does not exist)
- decisions.md was over budget (425): one entry added for this step, then the runner's
    `archive_overflow` rotated the three oldest into the archive (now 398). learnings.md stays at
    200
- The unpushed range also holds the `cid(human)` commit 9a140825 (agent prompts, `target.md`,
    decisions, issues): no gate was weakened in it
