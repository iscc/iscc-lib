## 2026-02-23 — Review of: Scaffold napi-rs crate with all 9 gen functions

**Verdict:** PASS

**Summary:** The `crates/iscc-napi/` crate was scaffolded correctly with all 9 `gen_*_v0` functions
exposed via napi-rs `#[napi]` attributes. The implementation follows the same thin-wrapper pattern
as the PyO3 bindings — type conversion and error mapping only, no logic in the binding layer. All
verification criteria pass: build, clippy, fmt, 143 Rust tests, 49 Python tests.

**Issues found:**

- (none)

**Next:** The napi crate compiles but has no runtime tests yet (requires Node.js + `npm install` +
`@napi-rs/cli`). The next logical steps are: (1) **WASM bindings** (`crates/iscc-wasm/` via
wasm-bindgen) to complete the binding layer scaffold, or (2) **CI workflow update** to include the
napi crate in the build matrix (at minimum `cargo build -p iscc-napi` + clippy), or (3) **Node.js
integration tests** if Node.js tooling is available. WASM scaffold would be the most impactful next
step as it completes another target deliverable and follows the same pattern.

**Notes:** The design doc (`notes/02-language-bindings.md`) uses `crates/iscc-node/` as the crate
directory name, but `crates/iscc-napi/` was chosen instead — this is fine and more descriptive. The
design doc also shows per-platform npm packages under `npm/` and a `__tests__/` directory — these
are for later when platform selection and CI publishing are set up. napi-rs type mappings: owned
`String` (not `&str`), `Buffer` (not `&[u8]`), `Option<T>` with `.unwrap_or()` for defaults.
