## 2026-11-21 — Review of: CDC parameter maths matches the reference for every `avg_chunk_size` (Rust core + Go)

**Verdict:** PASS

**Summary:** Rust `alg_cdc_params` and Go `algCdcParams` now return iscc-core's parameters for every
`u32` from 2 up, with no panic and no wrap. The diff touches two code files plus tests and the CRAP
baseline, changes no public signature and adds no error. An independent differential confirmed the
claim beyond the four tested values: both surfaces equal iscc-core on 210 values around every point
where `bits` changes.

**Verification:**

- [x] `cargo test -p iscc-lib` passes, including the new tests — rc 0; the three new `cdc::tests`
    ran by name (`--lib cdc`: 21 passed). `--no-default-features` and `+text-processing` also rc 0
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean and
    `cargo check -p iscc-lib --target wasm32-unknown-unknown` exits 0 — both rc 0 (workspace clippy
    too)
- [x] `go test ./...` and `go vet ./...` exit 0, `GOARCH=386 go vet ./...` exits 0 — all rc 0. The
    four new Go tests pass by name; on 386 three pass and the exact-64-bit test skips as designed
- [x] `mise run bench:iai:check` exits 0 — 16 benchmarks within 10%, no baseline change
- [x] `mise run coverage` + CI `cargo crap … --fail-regression --fail-above` exits 0 — "0 regressed,
    0 new, 0 moved, 158 unchanged" against the committed baseline
- [x] `mise run check` exits 0

**Probes beyond next.md (2 of 3):**

- Reference differential: iscc-core (Python 3.13) against the two Rust functions copied verbatim
    (overflow checks on) and against the real Go package, 210 values (2 to 5, every 2^k ± 1, every
    2^(k+0.5) ± 2, `u32::MAX`). Identical on all, masks compared after saturating to 32 bits
- Gear table: its largest value is 2137242950, below 2^31. So the pattern stays below 2^32 and the
    saturation argument in both doc comments holds

**Issues found:**

- The single-chunk tests cannot see the masks: a 64-byte input is shorter than `min_size`, so no
    byte is scanned. Only the parameter tests pin the masks. Not a defect, but it limits what the
    vector can prove (see Next)
- Filed `low`, untriaged: "No CI job executes the 32-bit code paths of the core or the Go port". On
    amd64 the old signed Go form is also correct, so CI would not catch a revert
- Nit, not filed: the Go error messages in `TestCdcParamsLargeAvgMatchesReference` print the
    unclamped expected value, which would mislead on a 386 failure

**Codex review:** no actionable findings. It ran the Rust suite, the Go suite and the 386 CDC tests
and judged the overflow handling and mask saturation consistent with the reference.

**Next:** the `data-edge.json` fan-out for `alg_cdc_chunks` (issue rewritten to this remainder). One
unsplittable step: 12 harnesses and the C generator (learnings 226). Facts to scope it with:

- A JSON-sized input gives one chunk equal to the input for every large value. The vector pins that
    each binding accepts the value and returns the core's result. It does not pin the masks
- Java `algCdcChunks` takes a signed `int`: 2^31 and 4294967295 cannot be passed. 2147483647 is the
    largest value every surface takes, and it reaches both former overflow sites. The issue now asks
    for 2^29 and 2147483647, which need no `except` entry (decision of 2026-11-21 on out-of-range
    integers: values a typed signature cannot carry are per-binding tests)
- Check how each harness reads `avg_chunk_size` from JSON before writing 2147483647: napi and WASM
    take an `f64`, .NET `uint`, Kotlin and Swift `UInt32`, Python and Ruby an unbounded integer
- The section also gives ordinary vectors a home (default 1024, `utf32: true`), if cheap

Other `normal` candidates, all independent: Python video frame `OverflowError` (1 file), the Ruby
floor move to 3.3, the C++ `safe_data` / .NET padding removal, the docs issues.

**Notes:**

- Decision recorded: `alg_cdc_chunks` accepts every `avg_chunk_size` from 2 up, large parameters
    saturate. Do not scope an upper bound
- The `.iai-baseline.json` readings sit 3 to 8% below the baseline on 9 of 16 benchmarks, including
    ones this step cannot reach (`bench_audio_code` −7.65%). The baseline dates from July and the
    toolchain is now rustc 1.100, so this is drift, not an effect of the diff. The gate is
    one-sided: a stale high baseline lets a real regression of that size pass. A deliberate refresh
    is a human call (decision of 2026-07-29)
- The Rust `max_size` saturation branch for a 32-bit `usize` is compiled by the wasm32 check and
    executed nowhere; it is one `try_from(..).unwrap_or(usize::MAX)`
- next.md said "`min`, `offset`, `center` cannot overflow". True for Rust, false for Go on 386. The
    advance agent caught it and documented the deviation, which is the intended handling
