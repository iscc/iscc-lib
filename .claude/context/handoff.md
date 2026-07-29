## 2026-07-29 — Restore `iscc_clean` to zero allocation on clean input (Perf gate)

**Done:** Changed `codec::iscc_clean` to return `Cow<'_, str>`, borrowing (zero allocation) when the
input has no scheme and no dashes, allocating only to strip dashes. `gen_iscc_code_v0`'s `cleaned`
is now `Vec<Cow<'_, str>>`. Behavior is byte-identical (all conformance + codec_clean tests pass).
This removes both per-component allocations (the split-collect `Vec` and the `replace` `String`).

**Result:** `bench_mixed_code.two_codes` is now GREEN (+18.30% → **+6.95%**).
`bench_iscc_code.four_units` dropped +36.82% → **+11.71%** but **remains RED** (threshold 10%).

**⚠️ Perf gate still fails on four_units — the step's premise is incomplete.** The stated cost (two
allocations) is fully removed, yet a residual remains that is **intrinsic scanning cost, not
allocation**. Evidence (Ir, the gate metric; baseline 11,968):

- Full reference-correct `iscc_clean`: 13,369 Ir (+11.71%).
- **Probe** — `iscc_clean` body replaced with `strip_prefix("ISCC:")`-only (still `Cow`/`Result`/
    collect): **12,279 Ir (+2.6%, passes)**. So Cow/call/collect adds ~311 Ir; the remaining ~1,090
    Ir is `trim()` + `split_once(':')` + `contains('-')` per code ×4 — the two byte-scans that the
    pre-191 inline `strip_prefix` never did.
- The reference (`codec.py:644`) does exactly this (split on `:`, strip each part, multibase check,
    strip `-`); there is no cheaper conformant algorithm.
- A manual single-pass byte loop (one scan for both `:` and `-`) measured **worse** (13,499) — std's
    vectorized `memchr` beats a scalar loop, so reducing scan count does not help.

**Conclusion:** the last ~1.7% (205 Ir over threshold) is unavoidable with std-only Rust. It is the
inherent cost of the reference-correct cleaning that iters 191/192 deliberately introduced, not a
removable allocation. Closing it requires a **human/gate decision**, which next.md places out of
scope:

- **Recommended — bump `.iai-baseline.json` four_units** (~11,968 → ~13,400) with justification: the
    regression is genuine but now *intrinsic to correctness*, so the baseline should reflect it.
    next.md forbids me from doing this ("Not In Scope"); I left `.iai-baseline.json` untouched
    (`git diff --quiet` confirms).
- Adding the `memchr` crate for `memchr2` was considered and **rejected**: std `str::contains`/
    `split_once` already call the same core SIMD memchr, so a dep is unlikely to help and is out of
    scope (would touch 2 `Cargo.toml` + Cargo.lock + audit gate).

I committed the allocation fix regardless — it is correct, strictly reduces the regression, and
greens mixed_code; reverting would be worse.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: `iscc_clean` → `IsccResult<Cow<'_, str>>`; `split_once(':')`,
    borrow on the no-scheme/no-dash path, allocate only on `contains('-')`; iter-192 empty guard and
    all branch semantics preserved. Inline `#[cfg(test)]` asserts unchanged (`Cow == &str`/`String`
    compile via std `impl_eq!`).
- `crates/iscc-lib/src/lib.rs`: line 894 annotation `Vec<String>` → `Vec<Cow<'_, str>>`; sites 222,
    810, 924 unchanged (deref coercion).
- `.crap-baseline.json` (generated): `iscc_clean` cyclomatic 7 → 9 (100% covered, well under 30.0);
    `.cargo-crap.toml` untouched.

**Verification:**

- `cargo test -p iscc-lib` — 309 + 10 (codec_clean) + 28 + 22 + 4 pass, 0 failed.
- `iscc_clean` still `Err(InvalidInput)` on `"   "`/`"-"`/`"iscc:"`/`"----"`, bad scheme, extra
    colon; valid hyphenated/padded/lowercase-scheme/multibase forms clean identically (codec_clean
    suite green).
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt --check` clean.
- `mise run bench:iai:check` — mixed_code +6.95% PASS; **four_units +11.71% FAIL** (see above);
    `.iai-baseline.json` unmodified.

**Next:** Human/reviewer decides four_units: bump the baseline (recommended — cost is intrinsic) or
accept the loop stays blocked. Once green, the Go `iscc_clean` port (issues.md `normal`) — which
must replicate both the `Cow` intent (its own idiom) and the iter-192 empty-cleaned guard — is
unblocked, plus the Ruby `gen_iscc_id_v1` validation-order and ASCII-only iai-bench gaps.

**Notes:**

- No API break: `iscc_clean` is `pub(crate)`; `Cow` derefs to `str` so all call sites are source-
    compatible.
- `.crap-baseline.json` diff is 30/30 (function reordering by line number after the edit) plus the
    cyclo 7→9 bump — regenerated via `mise run crap:baseline`, not hand-edited.
- Reviewer should independently re-run `mise run bench:iai:check` to confirm the four_units number
    and adjudicate the baseline decision — I cannot green it within the authorized scope.
