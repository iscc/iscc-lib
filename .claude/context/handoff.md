## 2026-11-20 — Move the iscc-ffi unit tests out of `lib.rs` into `src/tests.rs`

**Done:** Moved the body of `#[cfg(test)] mod tests { … }` (old lines 1718–2874) verbatim into
`crates/iscc-ffi/src/tests.rs`, one indent level removed and a `//!` docstring added. `lib.rs` now
ends with `#[cfg(test)]` / `mod tests;` and keeps every export, type and helper. CLAUDE.md updated.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: inline test module replaced by `#[cfg(test)] mod tests;` (2875 →
    1717 lines)
- `crates/iscc-ffi/src/tests.rs`: created (1157 lines), the test module body
- `crates/iscc-ffi/CLAUDE.md`: layout tree lists `tests.rs`; "Single-file crate" paragraph reworded
    (exports/types/helpers in `lib.rs`, tests in `src/tests.rs`, plus why exports must stay in
    `lib.rs`: `build.rs` passes only `src/lib.rs` to csbindgen); "Rust tests" heading points at
    `src/tests.rs`

**Verification:**

- `grep -n 'mod tests' -B1 lib.rs` → `1716-#[cfg(test)]` / `1717:mod tests;`; `wc -l` = 1717, under
    2500
- `grep -c '#\[test\]'`: lib.rs 0, tests.rs 99; `cargo test -p iscc-ffi --lib` → 99 passed, 0 failed
- Verbatim check: `git show HEAD:…/lib.rs | sed -n 1718,2874p | sed 's/^    //'` diffed against
    `tests.rs` (minus the 2 docstring lines) — only 2 hunks, both rustfmt joining a line that fits
    in 100 cols after de-indenting (`iscc_gen_meta_code_v0(...)` call,
    `ffi_encode_seq(...).is_none()`)
- Pre-move grep: no multi-line string literals in the block (only a single-line `r#"…"#`), so
    de-indenting changed no string content
- Ran the cbindgen command from next.md, then `cargo build -p iscc-ffi`; both files got fresh mtimes
    (iscc.h 14:40:06, NativeMethods.g.cs 14:41:55) and
    `git status --porcelain crates/iscc-ffi/include/iscc.h packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    printed nothing
- `grep -rn 'no_mangle\|extern "C"' crates/iscc-ffi/src/tests.rs` → nothing
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings` clean; `cargo fmt -p iscc-ffi --check`
    clean
- `mise run check` → exit 0

**Next:** the `[human]`-sourced `normal` issue "`data-edge.json` harnesses accept a value that only
resembles the expected one" (Swift scalar compare + Boolean rejection, Python `0`/`False`), then the
derived-facts parity check.

**Notes:**

- No core Rust, public API or benchmarked path changed, so CRAP, iai and semver were not run. The
    pre-push stage (workspace clippy/test/pytest) was not run separately beyond the iscc-ffi clippy
    and test above
- The issue "`crates/iscc-ffi/src/lib.rs` is past the crate's own split threshold" can be resolved
