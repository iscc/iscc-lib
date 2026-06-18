## 2026-06-18 — Add the iai-callgrind bench harness (compiles locally; CI gate is a follow-up slice)

**Done:** Landed `crates/iscc-lib/benches/iai_benches.rs`, an `iai-callgrind` 0.16 instruction-count
benchmark harness covering all 9 conformance `gen_*_v0` hot paths plus the `alg_cdc_chunks` and
`alg_minhash_256` primitives. Wired `iai-callgrind = "0.16"` into the root
`[workspace.dependencies]` and added the dev-dependency + `[[bench]]` entry (`harness = false`) to
`crates/iscc-lib/Cargo.toml`. The harness compiles and lints clean without valgrind or the runner;
actually *running* it (the CI `Perf` job + committed baseline) is the clearly-scoped follow-up
slice.

**Files changed:**

- `Cargo.toml` (root): added `iai-callgrind = "0.16"` to `[workspace.dependencies]` (latest stable
    on crates.io; next.md's `0.14` was a placeholder — confirmed `0.16.1` via `cargo search`).
- `crates/iscc-lib/Cargo.toml`: added `iai-callgrind = { workspace = true }` dev-dependency and a
    second `[[bench]]` (`name = "iai_benches"`, `harness = false`). The existing criterion
    `benchmarks` bench is untouched.
- `crates/iscc-lib/benches/iai_benches.rs` (new): 11 `#[library_benchmark]` functions in one
    `library_benchmark_group!` (`iscc_benches`) wired into `main!`. `gen_sum_code_v0` deferred (file
    I/O, syscall-dominated) — documented in the module docstring.
- `Cargo.lock`: refreshed with iai-callgrind + its transitive deps (build-only).

**Verification:** (every criterion from next.md)

- [x] `cargo build -p iscc-lib --bench iai_benches` → exit 0 (compiles with no valgrind/runner).
- [x] `cargo clippy -p iscc-lib --benches -- -D warnings` → clean.
- [x] `cargo fmt -p iscc-lib --check` → clean (exit 0).
- [x] `cargo test -p iscc-lib` → 269 + 28 + 22 + 1 doctest, **0 failed** — no regression.
- [x] `grep -q '^iai-callgrind' Cargo.toml` → OK;
    `grep -q 'iai-callgrind' crates/iscc-lib/Cargo.toml` → OK; `[[bench]]` has
    `name = "iai_benches"` + `harness = false`.
- [x] `mise run check` → all 15 pre-commit hooks Passed.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → clean (pre-push defense).

Not run (out of scope, needs valgrind absent in devcontainer): `cargo bench --bench iai_benches`.
The review agent should confirm the follow-up CI slice (valgrind runner +
`cargo binstall --force iai-callgrind-runner` + committed baseline) lands it.

**Next:** The clearly-scoped follow-up: add the `Perf` CI job (valgrind-enabled Linux runner,
install `iai-callgrind-runner@0.16.1` via `cargo binstall -y --force` — heed the rust-cache
`--force` poisoning gotcha), run `cargo bench -p iscc-lib --bench iai_benches`, commit the generated
baseline (`target/iai/.../*.summary.json` or the `--save-baseline`/`--baseline` flow), and add
`mise run bench:iai` + baseline-refresh tasks. That slice is the one that actually exercises the
gate.

**Notes:**

- **Docstring exception (technical necessity, not a style shortcut):** the 11 `#[library_benchmark]`
    functions use plain `//` comments, NOT `///` docstrings. The iai-callgrind-macros 0.6.1
    `#[library_benchmark]` attribute iterates *every* attribute on the function and `abort!`s with
    "Invalid attribute: 'doc'" on anything other than `bench`/`benches` — and a `///` comment lowers
    to a `#[doc = "..."]` attribute. Verified by reading
    `iai-callgrind-macros-0.6.1/src/lib_bench.rs:258-317`. The four helper fns
    (`deterministic_bytes`, `synthetic_text`, `deterministic_features`) are NOT macro-annotated and
    keep proper `///` docstrings. This is analogous to the documented FFI-boundary exception; the
    constraint is explained in the module docstring.
- **Measurement design:** inputs are built in `#[bench::id(expr)]` argument expressions, which
    iai-callgrind evaluates in the unmeasured setup phase, so only the ISCC computation is counted.
    Uses `std::hint::black_box` (not `criterion::black_box`) per next.md. Each bench returns
    `black_box(result)` to defeat DCE; `bench_cdc_chunks` returns `.len()` because the `Vec<&[u8]>`
    borrows from the function-local `data` and cannot escape (the chunking work is fully measured
    before `len()`).
- **Sizes mirror benchmarks.rs:** data/instance at 64KB + 1MB, cdc at 4KB/64KB/1MB, meta name-only +
    name+desc, text 1000 chars, image 1024B gradient, audio 300 features, video 10×380 frames, mixed
    2 codes, iscc 4 units. MinHash (not benched in criterion) uses a 1024-element spread feature vec
    via a new `deterministic_features` helper.
- **`iai-callgrind = "0.16"` (not `0.14`):** next.md flagged `0.14` as illustrative and told me to
    confirm the current version; crates.io has `0.16.1`. The macro crate is
    `iai-callgrind-macros   0.6.1`, runner is `iai-callgrind-runner 0.16.1`.
- **Out-of-scope, untouched per next.md:** no `Perf` CI job, no committed baseline, no `mise` bench
    tasks, no change to `.cargo-crap.toml` (its `crates/iscc-lib/benches/**` exclusion already
    covers the new file — CRAP gate unaffected), criterion `benchmarks.rs` left intact, `Semver`
    gate not flipped.
- `Cargo.lock` and `.claude/context/iterations.jsonl` both show as modified; only `Cargo.lock` is
    staged (iterations.jsonl is loop-runner-managed, not staged by this agent).
