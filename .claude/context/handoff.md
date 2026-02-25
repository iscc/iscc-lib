## 2026-02-25 — Review of: Add Go section and fix body text in root README

**Verdict:** PASS

**Summary:** Root README now includes Go Reference badge, Go installation section, Go quick-start
example, updated Key Features bullet, and corrected "What is iscc-lib" body text listing all six
binding ecosystems. The change is minimal, well-scoped, and matches the reference code in
`packages/go/README.md` exactly.

**Verification:**

- [x] `grep 'pkg.go.dev' README.md` exits 0 — Go Reference badge present
- [x] `grep 'go get github.com/iscc/iscc-lib/packages/go' README.md` exits 0 — Go install command
    present
- [x] `grep 'iscc.NewRuntime' README.md` exits 0 — Go quick-start example present
- [x] `grep 'Python, Java, Go, Node.js, WebAssembly, and C' README.md` exits 0 — body text fixed
- [x] `grep 'Python, Java, Go, Node.js, WASM, and C FFI' README.md` exits 0 — Key Features updated
- [x] `grep -c '### Go' README.md` outputs `2` — two Go sections (Installation + Quick Start)

**Issues found:**

- (none)

**Next:** Documentation how-to guides (`docs/howto/go.md` and `docs/howto/java.md`) or address
tracked issues from issues.md (codec optimization, DataHasher buffer allocation, etc.).

**Notes:** All 14 pre-commit hooks pass. Only `README.md` was modified (no code changes). Go section
ordering is Rust → Python → Node.js → Java → Go → WASM, consistent in both Installation and Quick
Start. The mdformat-safe rewording trick (using "language bindings" to control line wrapping) is
noted in advance agent memory for future reference.
