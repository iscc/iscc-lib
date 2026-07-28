# Next Work Package

## Step: Bump criterion 0.7 → 0.8 (dev-dependency, bench harness only)

## Goal

Land the first of the two authorized dependency majors that gate the v0.6.0 release: move the
`criterion` workspace dev-dependency to 0.8 and refresh `Cargo.lock`. This is fully verifiable
locally and leaves the published MSRV untouched.

## Alternatives Considered

- **Chosen:** `criterion` 0.7 → 0.8 — the smaller of the two remaining release-gating items, it
    touches one manifest plus a generated lockfile, and every gate that could react (clippy
    `-D warnings`, `cargo bench --no-run`, `cargo deny`) runs in this container.
- **Rejected:** `uniffi` 0.31 → 0.32 — the other authorized item, but it regenerates two checked-in
    binding files and its Swift half is only verifiable by the `swift` CI job. Doing the cheap,
    fully local bump first keeps a red CI attributable to the uniffi step when it lands.

## Scope

- **Modify**: `Cargo.toml` (root — the `criterion` pin and its now-stale rationale comment),
    `Cargo.lock` (generated), and `crates/iscc-lib/benches/benchmarks.rs` **only if** the build
    demands it (no API used there was removed in 0.8 — see notes).
- **Reference**: `.claude/context/issues.md` → "Dependency review and refresh across the project"
    (the binding constraints), `crates/iscc-lib/Cargo.toml` (the two `[[bench]]` targets),
    `.github/workflows/ci.yml` → the `bench` job.

## Not In Scope

- **Do not touch `rust-version = "1.85"`** anywhere. criterion's rustc-1.86 floor is a
    contributor/bench toolchain floor; a dev-dependency is never built by downstream consumers.
    Raising the declared MSRV is a human-gated v1.0.0 decision (`low` issue).
- Do not bump `uniffi` (that is the next step), `iai-callgrind`, or anything else in the same
    commit; do not run a bare `cargo update`.
- Do not refresh `.iai-baseline.json` or `.crap-baseline.json` — no `iscc-lib` source line moves, so
    neither the perf nor the CRAP gate has anything to re-record.
- Do not edit `issues.md`; the review agent owns issue resolution. Put progress in the handoff.
- Do not add a `deny.toml` entry (see notes — the one new transitive crate is already allow-listed).

## Implementation Notes

Measured while scoping, all read-only from the sparse index and the 0.8.2 `.crate` tarball:

- Latest is **0.8.2** (0.8.0/0.8.1/0.8.2 all unyanked, `rust-version = "1.86"`). Write the pin as
    `criterion = { version = "0.8", features = ["html_reports"] }` — the `html_reports` feature
    still exists in 0.8 with the same name, as do `async`, `cargo_bench_support`, `real_blackbox`.
- The only 0.8.0 BREAKING entry is **"Drop async-std support"** (unused here). Everything
    `benches/benchmarks.rs` imports survives: `BenchmarkId` and `Bencher` are re-exported from
    `benchmark_group`, `Criterion` and `Throughput` are defined in `lib.rs`, and the
    `criterion_group!` / `criterion_main!` macros are unchanged. `benches/iai_benches.rs` mentions
    criterion only in doc comments — it is an iai-callgrind target and does not link criterion.
- New transitive dependency on unix/windows: **`alloca` 0.4.0 (MIT)**, plus its `cc` build
    dependency (memory-layout randomisation added in 0.8). MIT is already in `deny.toml`'s allow
    list, so the audit gate needs no change — but run it, since the lock gains crates.
- Refresh the lock by editing the pin and letting `cargo` resolve (or `cargo update -p criterion`).
    Keep the `Cargo.lock` diff scoped to criterion, criterion-plot and newly required crates.
- Replace the four-line `# authorized 2026-07-28 (pending bump): …` comment above the pin with a
    short evergreen note: criterion is a dev-dependency of `iscc-lib` consumed by the `benchmarks`
    bench target, so its rustc-1.86 floor is a contributor/bench floor, not the published MSRV.
    Leave the `uniffi` comment block exactly as it is.
- `cargo deny` is not preinstalled here: `cargo binstall cargo-deny@0.19.9 --force` first.

## Verification

- Root `Cargo.toml` pins criterion at `version = "0.8"`, and the `criterion` package entry in
    `Cargo.lock` resolves to a `0.8.x` version.
- `grep -n 'rust-version = "1.85"' Cargo.toml` still matches (the MSRV line is untouched).
- `cargo bench --no-run` exits 0 (the exact command of the CI `bench` job).
- `cargo bench -p iscc-lib --bench benchmarks -- --test` exits 0 (every bench body executes once
    under the new harness, not just compiles).
- `mise run lint` exits 0 (clippy `--workspace --all-targets -D warnings` — this is what catches a
    fresh deprecation, as criterion 0.6 did with `criterion::black_box`).
- `mise run test` and `mise run audit` both exit 0.

## Done When

`criterion` 0.8 is pinned in the root manifest with a refreshed `Cargo.lock`, the MSRV line is
unchanged, and all six verification checks pass.
