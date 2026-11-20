# Next Work Package

## Step: Move the iscc-ffi unit tests out of `lib.rs` into `src/tests.rs`

## Goal

Close the `normal` issue "`crates/iscc-ffi/src/lib.rs` is past the crate's own split threshold" by
moving the `#[cfg(test)] mod tests` block (lines 1716–2875) to its own file. `lib.rs` drops to about
1715 lines and every `extern "C"` function stays in `lib.rs`, so csbindgen and cbindgen keep seeing
the same input.

## Alternatives Considered

- **Chosen:** FFI test-module move. It is the handoff's "Next", it is the only open `normal` issue
    that is not `[human]`-sourced, and it is a mechanical change with byte-identical generated files
    as an objective check.
- **Rejected:** stricter `data-edge.json` compares (Swift scalar compare plus Boolean rejection,
    Python `0`/`False`). It is `normal` and worth doing next, but it needs a Swift toolchain run and
    touches two harnesses. The FFI move is smaller and fully settled. The derived-facts parity check
    is the larger third candidate.

## Scope

- **Create**: `crates/iscc-ffi/src/tests.rs`
- **Modify**: `crates/iscc-ffi/src/lib.rs` (replace the inline block with
    `#[cfg(test)] mod tests;`), `crates/iscc-ffi/CLAUDE.md` (docs: layout tree, "Single-file crate"
    paragraph, the "Rust tests (`#[cfg(test)]` in `lib.rs`)" heading)
- **Reference**: `crates/iscc-ffi/build.rs` (csbindgen reads only `src/lib.rs`),
    `crates/iscc-ffi/cbindgen.toml`, `.github/workflows/ci.yml` job "C FFI" (the cbindgen command
    line)

## Not In Scope

- Moving any `extern "C"` function, `#[repr(C)]` type or helper out of `lib.rs`. That would need
    `build.rs` to list the new files, or csbindgen silently drops them from `NativeMethods.g.cs`
- Changing the ~2500-line threshold in `CLAUDE.md`, or splitting `tests.rs` further
- Editing, renaming, adding or deleting any test. This is a verbatim move, apart from removing one
    indent level and adding a module docstring
- The other open `normal` issues (data-edge compare looseness, derived-facts parity check)

## Implementation Notes

- `tests.rs` holds the body of the current `mod tests { … }`, with the 4-space indent removed, and
    starts with a `//!` docstring (e.g. "Unit tests for the iscc-ffi C exports."). `use super::*;`
    and `use std::ffi::CString;` keep working in a file module. Private helpers in `lib.rs` are
    still reachable through `super::`.
- In `lib.rs`, keep `#[cfg(test)]` on the `mod tests;` line so non-test builds never compile the
    file. Run `cargo fmt -p iscc-ffi` afterwards. Watch for rustfmt reflowing lines that become
    shorter after de-indenting; that is expected.
- The test file has no `#[no_mangle]` or `extern "C"` items (confirm with grep). If it has any, stop
    and report, because cbindgen and csbindgen would treat them differently after the move.
- Regenerate both generated files the way CI and `build.rs` do:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`,
    then `cargo build -p iscc-ffi` (this rewrites `NativeMethods.g.cs`). Both must come out
    unchanged.
- `CLAUDE.md`: list `tests.rs` in the layout tree. Reword "Single-file crate" so it says that all
    exports, types and helpers live in `lib.rs` and the unit tests in `src/tests.rs`. Point the
    "Rust tests" heading at `src/tests.rs`. Do not write line counts.
- Gates: iscc-ffi is not part of the CRAP/coverage or iai gates (those cover `iscc-lib` only), and
    no public API changes, so semver is unaffected.

## Verification

- `grep -n 'mod tests' crates/iscc-ffi/src/lib.rs` prints exactly one line, `mod tests;`, and the
    line above it is `#[cfg(test)]`. `wc -l < crates/iscc-ffi/src/lib.rs` prints a number below 2500
- `grep -c '#\[test\]' crates/iscc-ffi/src/lib.rs` prints `0`, and the passed count from
    `cargo test -p iscc-ffi --lib` equals `grep -c '#\[test\]' crates/iscc-ffi/src/tests.rs`, with 0
    failed
- After the cbindgen command above plus `cargo build -p iscc-ffi`,
    `git status --porcelain crates/iscc-ffi/include/iscc.h packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    prints nothing
- `grep -rn 'no_mangle\|extern "C"' crates/iscc-ffi/src/tests.rs` prints nothing
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings` and `cargo fmt -p iscc-ffi --check` are
    clean
- `mise run check` exits 0

## Done When

All six verification criteria pass, with `lib.rs` below the threshold and both generated files
unchanged.
