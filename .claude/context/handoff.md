## 2026-02-23 — Restructure documentation navigation into Diataxis categories

**Done:** Reorganized the flat site navigation into Diataxis framework sections (How-to Guides,
Explanation, Reference) and created three per-language how-to guide pages for Python, Node.js, and
WebAssembly. The Python guide is the most detailed, covering structured results (`IsccResult`
subclasses with attribute access), streaming (`DataHasher`/`InstanceHasher` with `BinaryIO`
support), and all text utilities. The Node.js and WASM guides cover their respective installation,
import patterns, code generation, streaming, and binary data handling (Buffer vs Uint8Array).

**Files changed:**

- `zensical.toml`: Replaced flat `nav` list with Diataxis-organized sections (How-to Guides with
    Python/Node.js/WASM, Explanation with Architecture, Reference with Rust API/Python API, and
    top-level Benchmarks)
- `docs/howto/python.md`: Python usage guide (32 headings) covering installation, all 9 gen
    functions, structured results with attribute/dict access, DataHasher/InstanceHasher streaming,
    text utilities, conformance testing, and error handling
- `docs/howto/nodejs.md`: Node.js usage guide (19 headings) covering @iscc/lib installation, import
    pattern, all 9 gen functions with Buffer examples, DataHasher/InstanceHasher streaming with
    createReadStream, text utilities, and error handling
- `docs/howto/wasm.md`: WebAssembly usage guide (23 headings) covering @iscc/iscc-wasm installation,
    bundler/browser/Node.js setup, all 9 gen functions with Uint8Array examples, File/Blob
    conversion patterns, streaming with ReadableStream, and tip recommending @iscc/lib for Node.js
    server use

**Verification:** All verification criteria pass:

- `uv run zensical build` exits 0 (8 pages built in 1.76s)
- All 3 howto guide files exist with headings (32, 19, 23 respectively)
- `zensical.toml` contains "How-to Guides", "Explanation", "Reference" sections
- All howto paths present in nav: howto/python, howto/nodejs, howto/wasm
- Install instructions present: pip install (python), npm install (nodejs, wasm)
- gen_text_code_v0 example present in python guide
- DataHasher streaming coverage present in python guide
- All 14 pre-commit hooks pass (mdformat, Ruff, cargo fmt, TOML fmt, etc.)
- cargo clippy --workspace --all-targets clean (no Rust changes)

**Next:** The remaining documentation gaps are: (1) abbreviations file
(`docs/includes/abbreviations.md`) with ISCC-specific terms and `pymdownx.snippets` auto-append
configuration, (2) CNAME file (`docs/CNAME` with `lib.iscc.codes`), (3) OIDC trusted publishing
configuration in `release.yml`. The abbreviations and CNAME items are small and could be combined
into a single step.

**Notes:** No existing files were moved -- the Diataxis structure is achieved purely through nav
labels in `zensical.toml`, keeping existing URL paths stable. The Python guide shows the idiomatic
`result.iscc` attribute access pattern (not the `json.loads()` pattern from the Quick Start in
index.md) as specified in next.md. The WASM guide includes a tip admonition recommending `@iscc/lib`
over `@iscc/iscc-wasm` for Node.js server use. No Rust code was touched.
