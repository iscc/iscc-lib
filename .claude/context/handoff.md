> **HUMAN REVIEW REQUESTED**: The enforcing `Perf (iai-callgrind)` gate stays RED on develop
> (`bench_iscc_code.four_units` +11.71% > 10%) and the loop is blocked. Review confirms the residual
> is **intrinsic to reference-correct cleaning**, not a removable defect — next.md's premise that
> the cost was "genuinely removable" is falsified. Resolving it is a perf-baseline/dependency
> decision the CID loop must not make autonomously. Pick one (detail in issues.md, `critical`
> entry): **(A, recommended)** bump `.iai-baseline.json` four_units ~11,968→~13,400 + a decisions.md
> note — the cost is genuine, intrinsic to the deliberate 191/192 correctness fix, and negligible in
> absolute terms; **(B)** authorize the `memchr` crate for a `memchr2` single-pass (touches 2
> Cargo.toml + Cargo.lock + `deny.toml`); **(C)** revert 191/192 (reopens codec divergence — no).

## 2026-07-29 — Review of: Restore `iscc_clean` to zero allocation on clean input (Perf gate)

**Verdict:** NEEDS_WORK

**Summary:** The advance agent correctly made `codec::iscc_clean` return `Cow<'_, str>`, borrowing
allocation-free on the no-scheme/no-dash path and allocating only to strip dashes. Behavior is
byte-identical (faithful port of reference `codec.py:644`; empty guard from iter 192 preserved). The
fix greened `bench_mixed_code.two_codes` (+18.30%→+6.95%) and cut `four_units` +36.82%→+11.71%, but
four_units **stays over the 10% gate**, so the step's Done When is not met and CI stays red. The
commit is correct and strictly better and should NOT be reverted — but it cannot green the gate
within the authorized scope.

**Verification:**

- [x] `cargo test -p iscc-lib` — 309 + 10 (codec_clean) + 28 + 22 + 4 + 1 doctest pass, 0 failed
- [x] `iscc_clean` errors on `"   "`/`"-"`/`"iscc:"`/`"----"`, bad scheme, extra colon; valid
    hyphenated/padded/lowercase-scheme/multibase forms clean identically (codec_clean 10/10)
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt --check` clean
- [x] `.crap-baseline.json` regenerated (cyclo 7→9, 100% cov); `.cargo-crap.toml` untouched
- [ ] `mise run bench:iai:check` — mixed_code +6.95% PASS; **four_units +11.71% FAIL** vs the
    unmodified committed baseline (`.iai-baseline.json` untouched, `git diff --quiet` confirmed).
    **Primary criterion NOT met.**

**Probes (verified independently):**

- Reference `iscc_clean` (`codec.py:644`) does exactly split-on-`:`/strip-each/multibase-check/strip
    dash — the port is faithful; no cheaper conformant algorithm in std.
- Pre-191 `gen_iscc_code_v0` did only `strip_prefix("ISCC:")` (NOT reference-correct — no trim, no
    dash-strip, no lowercase scheme). So the committed baseline was set for cheaper-but-wrong code;
    191/192 traded that cost for correctness — why removing allocations alone can't restore it.
- The residual is two std `memchr` scans (`split_once(':')` + `contains('-')`); a scalar single-pass
    measured *worse* (advance probe: 13,499 Ir). Irreducible without the out-of-scope `memchr`
    crate.

**Issues found:**

- Primary: four_units Perf gate red — filed as `critical` [review] with the (A)/(B)/(C) decision.
- No gate weakening in the unpushed range; no API break (`iscc_clean` is `pub(crate)`; `Cow` derefs
    to `str`, all call sites source-compatible). Scope clean (2 source files + 2 generated
    baselines).

**Codex review:** Concurs — flags the same P1 (four_units 13,369 vs 11,968 = +11.71%, CI red) and
suggests "combine the delimiter checks or obtain an approved rebaseline." Combining scans is what
the advance already measured as *slower* (std SIMD memchr beats a scalar merge); the approved
rebaseline is option (A). Advisory only — does not change the verdict.

**Next:** BLOCKED on the human decision above. Once four_units is green (option A/B), unblock in
priority order: Go `iscc_clean` port (issues.md `normal` — must replicate the iter-192 empty guard),
then Ruby `gen_iscc_id_v1` validation-order and the ASCII-only iai-bench gap.

**Notes:**

- NOT pushed — verdict NEEDS_WORK, Perf gate red. Remote develop is already red at the iter-192 tip
    (b2f56b6); iter 192's review missed the 191 regression because `mise run check` skips iai.
- The iter-193 `Cow` commit (`5ffa5cb`) stays regardless of which option lands — it is correct and
    strictly reduces the regression.
