# Next Work Package

## Step: Restore `iscc_clean` to zero allocation on clean input to green the Perf gate

## Goal

Fix the RED enforcing `Perf (iai-callgrind)` gate on `develop`. Routing every codec-input site
through `iscc_clean` (iters 191/192) made it allocate a `Vec` (split-collect) **plus** a `String`
(`replace`) per component, spiking `bench_iscc_code.four_units` +36.82% and
`bench_mixed_code.two_codes` +18.30%. The pre-191 inline cleaning was zero-allocation
(`strip_prefix` → borrowed `&str`). Make `iscc_clean` return `Cow<'_, str>` and skip both
allocations when the input has no scheme and no dashes, restoring cost to the committed baseline.
Nothing else goes green first.

## Alternatives Considered

- **Chosen:** optimize `iscc_clean` (allocation-free common path) — CI is red and no other work can
    land until it passes; the cost is genuinely removable, so this is a true fix, not masking.
- **Rejected:** justified `.iai-baseline.json` bump — masks a real, avoidable 18-37% regression;
    baseline moves are human/gate territory (state.md) and would leave clean-input hashing
    needlessly allocating. The Go `iscc_clean` port and other `normal` issues stay blocked behind
    green CI.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` — change `iscc_clean` to return
    `IsccResult<Cow<'_,   str>>`; replace `.split(':').collect::<Vec<_>>()` with `split_once(':')`,
    and only allocate (`replace('-', "")`) when the code `contains('-')`, else return
    `Cow::Borrowed`.
- **Modify**: `crates/iscc-lib/src/lib.rs` — line 897 collect annotation `Vec<String>` →
    `Vec<Cow<'_, str>>`; other call sites (222, 810, 924) work unchanged via deref coercion.
- **Modify (generated, excl. budget)**: `.crap-baseline.json` if `iscc_clean`'s CRAP value shifts.
- **Reference**: current `iscc_clean` (codec.rs:532-562); benchmark inputs (iai_benches.rs:99,122 —
    all dash-free, scheme-free); handoff.md; git `2537e18` (the routing that removed inline
    cleaning).

## Not In Scope

- Touching `.iai-baseline.json` — the whole point is to pass the *existing* committed baseline; if a
    bench still exceeds 10% after this, the fix is incomplete, not the baseline.
- The Go `iscc_clean` port and the other `normal` `[review]` issues — blocked on green CI.
- Any change to cleaning *semantics* (multibase dash preservation, scheme matching, the empty-input
    guard from iter 192, valid/garbage differential behavior) — behavior must be byte-identical.

## Implementation Notes

- `Cow<'_, str>` derefs to `str`, so `decode_base32(&clean)`, `code.len()`, `contains(':')` etc. all
    work at the existing call sites without change — only lib.rs:897's explicit `Vec<String>`
    annotation must become `Vec<Cow<'_, str>>`.
- Preserve exact branch logic: single-part multibase (`f/b/v/z/u` first char) keeps dashes
    (`Cow::Borrowed`); two-part requires case-insensitive `iscc` scheme then strips dashes; >1 colon
    errors "Malformed"; empty cleaned result errors "Empty ISCC string" (keep the iter-192 guard).
    Watch the two-part case: `split_once` gives `(scheme, rest)`; a second colon in `rest` must
    still error — check `rest.contains(':')` (the old `Vec` len-3 branch caught this).
- Inline `#[cfg(test)]` asserts in codec.rs compare `iscc_clean(...).unwrap()` against `&str`
    literals; if `Cow` breaks `assert_eq!` type inference, add `.as_ref()` to the left operand — do
    not change the expected values.
- CRAP gate is CI-only + enforcing: if the refactor changes `iscc_clean`'s cyclomatic value,
    regenerate `.crap-baseline.json` (`mise run coverage` then `mise run crap:baseline`) in THIS
    step; never widen the epsilon or the 30.0 threshold in `.cargo-crap.toml`.

## Verification

- `cargo test -p iscc-lib` passes (all existing codec_clean + conformance tests, 0 failed).
- `mise run bench:iai:check` reports no >10% regression on `bench_iscc_code.four_units` and
    `bench_mixed_code.two_codes` against the **unmodified** committed baseline (`git diff --quiet`
    on `.iai-baseline.json`).
- `iscc_clean` still errors (`Err(InvalidInput)`) on `"   "`/`"-"`/`"iscc:"`/`"----"` and on a
    bad-scheme / extra-colon input; valid hyphenated/padded/lowercase-scheme/multibase forms still
    clean identically.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt -p iscc-lib --check`
    clean.
- `mise run check` (prek all-files) passes; `.cargo-crap.toml` untouched.

## Done When

The two regressed iai benches are within 10% of the committed baseline and the full `iscc-lib` test
suite plus clippy/fmt are green, with `iscc_clean` behavior unchanged.
