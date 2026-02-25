## 2026-02-25 — Create iscc-ffi per-crate README

**Done:** Created `crates/iscc-ffi/README.md` following the established per-crate README pattern.
Includes all required sections: title, badges (CI + License, no registry badge), experimental
notice, tagline, "What is ISCC", "Building" (with platform-specific output paths and cbindgen header
generation), "Quick Start" (C example with `iscc_gen_meta_code_v0` and `iscc_free_string`), "API
Overview" (9 gen functions with `iscc_` prefixes), "Memory Management" (4 free functions +
`iscc_last_error`), "Links", and "License".

**Files changed:**

- `crates/iscc-ffi/README.md`: Created (new file, ~120 lines)

**Verification:** All 7 verification checks from next.md pass:

- `test -f crates/iscc-ffi/README.md` exits 0 (file exists)
- `grep "iscc-ffi"` matches (title heading)
- `grep "What is ISCC"` matches (standard section)
- `grep "iscc_gen_meta_code_v0"` matches (C function names in table and quick start)
- `grep "iscc_free_string"` matches (memory management section)
- `grep "Apache-2.0"` matches (license section)
- No other files modified (only README.md and handoff.md changed)
- `mise run check` -- all 14 pre-commit hooks pass

**Next:** All 7 per-crate READMEs are complete. The "Per-Crate READMEs" gap in target.md is closed.
The project is ready for the `v0.0.1` release -- consider creating a PR from `develop` to `main`.

**Notes:** The streaming utilities bullet initially caused an mdformat-mkdocs crash ("Formatted
Markdown renders to different HTML than input Markdown") because the long chain of
`iscc_data_hasher_new`/`iscc_data_hasher_update`/... backtick-separated names exceeded the 100-char
wrap limit in a way that triggered a rendering divergence bug in the mdformat-mkdocs plugin. Fixed
by restructuring as shorter references (`iscc_data_hasher_new`/`update`/`finalize`/`free`). Also
avoided `char *` in prose text (replaced with "string pointer") since backtick-asterisk combinations
can confuse markdown formatters during line wrapping.
