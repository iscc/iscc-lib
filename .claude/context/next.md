# Next Work Package

## Step: Add Ruby documentation (howto guide, README, root README)

## Goal

Write the Ruby how-to guide, expand the per-crate README from stub to full guide, and add Ruby
install/quickstart sections to the root README. This completes the documentation deliverables for
the Ruby bindings issue and makes Ruby a first-class documented language alongside the existing 7.

## Scope

- **Create**: `docs/howto/ruby.md`
- **Modify**: `crates/iscc-rb/README.md`, `README.md`, `zensical.toml` (add Ruby nav entry)
- **Reference**:
    - `docs/howto/go.md` — closest structural template (package-level functions, typed results)
    - `docs/howto/java.md` — bridge analog (compiled native extension, similar install story)
    - `docs/howto/python.md` — PyO3 bridge analog (similar wrapper pattern)
    - `crates/iscc-rb/lib/iscc_lib.rb` — Ruby wrapper API (result classes, keyword args)
    - `crates/iscc-rb/src/lib.rs` — Magnus bridge (function signatures)
    - `crates/iscc-rb/test/` — smoke tests (working code examples to adapt)
    - `README.md` — existing Install/Quick Start sections to add Ruby alongside

## Not In Scope

- Standard Ruby linting (`standard` gem, `.standard.yml`, CI wiring) — separate future step
- Ruby API reference page (`docs/ruby-api.md`) — not required by target
- Updating multi-language tabbed examples on other docs pages — separate step if needed
- Expanding smoke/unit tests or adding new test coverage

## Implementation Notes

### `docs/howto/ruby.md` (~370-430 lines, following existing guide pattern)

Use `docs/howto/go.md` as the primary structural template. Sections:

1. **Frontmatter**: `icon: lucide/gem`, description line
2. **Intro paragraph**: Native Rust extension via Magnus, precompiled gems, `gem install`
3. **Installation**: `gem install iscc-lib` + Bundler `Gemfile` alternative + "Build from source"
    admonition (like Java's "Build from source" note — `bundle exec rake compile`)
4. **Code generation**: All 10 `gen_*_v0` functions with Ruby examples using keyword args. Group:
    - Meta-Code (with description, meta JSON examples)
    - Text-Code, Image-Code, Audio-Code, Video-Code, Mixed-Code
    - Data-Code, Instance-Code (byte data via `File.binread`)
    - ISCC-Code (combining units)
    - Sum-Code (file path)
5. **Streaming API**: `DataHasher` and `InstanceHasher` with method chaining
6. **Codec and diagnostics**: `iscc_decode`, `iscc_decompose`, `encode_component`, `encode_base64`,
    `json_to_data_url`, `conformance_selftest`
7. **Text utilities**: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
8. **Algorithm primitives**: `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`
9. **Constants**: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE`

Key Ruby API patterns to show:

- Result classes use attribute-style access: `result.iscc`, `result.name`, `result.metahash`
- Keyword arguments: `gen_meta_code_v0("name", description: "desc", bits: 64)`
- Binary data as Ruby strings: `data = File.binread("file.bin")`
- Streaming: `hasher = IsccLib::DataHasher.new.update(chunk1).update(chunk2).finalize`

### `crates/iscc-rb/README.md` (expand from 31 lines to ~80-100 lines)

Follow the pattern of other per-crate READMEs (e.g., `crates/iscc-py/README.md`):

- Badges (RubyGems version, CI, license)
- What is ISCC (2-3 sentences)
- Installation (gem install + Bundler)
- Quick start (gen_meta_code_v0 + streaming example)
- API overview (list of 10 gen functions + key utilities)
- Links (lib.iscc.codes, repository, ISCC spec)
- License

### `README.md` (add Ruby sections)

Add Ruby in two places, maintaining the current ordering pattern:

1. **Installation section**: Add `### Ruby` with `gem install iscc-lib` after Go and before WASM
    (current order: Rust, Python, Node.js, Java, Go, WASM — insert Ruby after Go)
2. **Quick Start section**: Add `### Ruby` with a `gen_meta_code_v0` example after Go and before
    WASM

Ruby example for Quick Start:

```ruby
require "iscc_lib"

result = IsccLib.gen_meta_code_v0("ISCC Test Document!")
puts "Meta-Code: #{result.iscc}"
```

### `zensical.toml` (add Ruby nav entry)

Add `{ "Ruby" = "howto/ruby.md" }` to the How-to Guides navigation list. Insert after Python
(current order: Rust, Python, Node.js, WASM, Go, Java, C/C++). Place Ruby after Python to group
scripting languages together.

## Verification

- `test -f docs/howto/ruby.md && echo "EXISTS"` — howto guide created
- `wc -l docs/howto/ruby.md | awk '{print ($1 >= 300) ? "PASS" : "FAIL"}'` — substantial content
    (≥300 lines)
- `wc -l crates/iscc-rb/README.md | awk '{print ($1 >= 60) ? "PASS" : "FAIL"}'` — expanded from stub
    (≥60 lines)
- `grep -q 'gem install iscc-lib' README.md && echo "PASS"` — Ruby install in root README
- `grep -q 'ruby.md' zensical.toml && echo "PASS"` — Ruby in docs navigation
- `grep -q 'gen_meta_code_v0' docs/howto/ruby.md && echo "PASS"` — howto has code examples
- `grep -q 'DataHasher' docs/howto/ruby.md && echo "PASS"` — howto covers streaming API
- `grep -q 'conformance_selftest' docs/howto/ruby.md && echo "PASS"` — howto covers diagnostics
- `mise run check` exits 0 — all pre-commit hooks pass (formatting, YAML, etc.)

## Done When

All verification commands pass: Ruby howto guide exists with comprehensive API coverage (≥300
lines), per-crate README expanded from stub, root README includes Ruby install and quickstart, docs
navigation updated, and all formatting hooks clean.
