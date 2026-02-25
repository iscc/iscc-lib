## 2026-02-25 — Add Go section and fix body text in root README

**Done:** Added Go Reference badge, Go installation section, Go quick-start example, and updated
body text in the root README to mention all six binding ecosystems (Python, Java, Go, Node.js,
WebAssembly, and C).

**Files changed:**

- `README.md`: Added Go Reference badge (line 8), updated Key Features bullet to include Go (line
    24), fixed "What is iscc-lib" body text to list all six bindings (line 48), added Go
    installation section with `go get` command between Java and WASM (lines 106-110), added Go
    quick-start section with complete runnable example between Java and WASM (lines 156-185)

**Verification:** All six grep verification criteria pass:

1. `grep 'pkg.go.dev' README.md` — Go Reference badge present ✓
2. `grep 'go get github.com/iscc/iscc-lib/packages/go' README.md` — Go install command ✓
3. `grep 'iscc.NewRuntime' README.md` — Go quick-start example ✓
4. `grep 'Python, Java, Go, Node.js, WebAssembly, and C' README.md` — body text fixed ✓
5. `grep 'Python, Java, Go, Node.js, WASM, and C FFI' README.md` — Key Features updated ✓
6. `grep -c '### Go' README.md` outputs `2` — two Go sections ✓

All 14 pre-commit hooks pass (`mise run check`).

**Next:** Documentation how-to guides (`docs/howto/go.md` and `docs/howto/java.md`) or address
tracked issues from issues.md (codec optimization, DataHasher buffer allocation, etc.).

**Notes:** The "What is iscc-lib" body text required careful line wrapping to ensure the full
language list (`Python, Java, Go, Node.js, WebAssembly, and C`) lands on a single grep-matchable
line after mdformat auto-wraps at 100 characters. Changed "bindings" to "language bindings" to add
enough characters to push "Python" to the start of the next line, producing a stable wrapping that
mdformat doesn't re-break.
