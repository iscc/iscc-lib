# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` in the Ruby bindings (+ locale-free conformance load)

## Goal

Expose ISCC-SEQ (`encode_seq` / `decode_seq`, IEP-0020) in the Ruby gem so it reaches 9 of 11
surfaces, with the Ruby conformance harness running both data.json seq sections. Also closes the
`normal` issue "Ruby conformance test depends on the process locale" (same test file, one line).

## Alternatives Considered

- **Chosen:** Ruby seq: one source file, a locally runnable toolchain (ruby 3.1 + bundler here), and
    it also clears a `normal` issue in the file it already touches.
- **Rejected:** UniFFI seq (Swift + Kotlin): a larger fan-out with 3 regenerated artifacts and
    hard-coded per-section counts in both suites. It comes next, once Ruby is done.

## Scope

- **Create**: `crates/iscc-rb/test/test_seq.rb`
- **Modify**: `crates/iscc-rb/src/lib.rs`, `crates/iscc-rb/test/test_conformance.rb`
- **Reference**: `crates/iscc-rb/src/lib.rs` (`iscc_decompose` L287 for `Vec<String>` out,
    `encode_base64` L277 for `RString` in, `alg_minhash_256` L369 for a binary String out);
    `.claude/context/specs/rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ, IEP-0020)";
    `crates/iscc-lib/tests/data.json` sections `encode_seq` (L579) / `decode_seq` (L605);
    `tests/test_seq.py` (Python) for the negative-case set

## Not In Scope

- The repo-wide 33→35 doc sweep (`crates/iscc-rb/README.md`, `crates/iscc-rb/CLAUDE.md`,
    `docs/howto/ruby.md`, notes, other bindings). That is one later step after UniFFI
- `lib/iscc_lib.rb` wrappers: register both functions unprefixed, like `iscc_decompose`
- The Ruby `json` Gemfile.lock Dependabot issue: `rb_sys` is held, so it is a separate step
- UniFFI / Swift / Kotlin seq, and any change to `crates/iscc-lib`

## Implementation Notes

- `encode_seq(units: Vec<String>) -> Result<RString, Error>`: the core takes `&[&str]`
    (`codec.rs:818`), so build `Vec<&str>` first (see `gen_mixed_code_v0` L119); map errors with
    `to_magnus_err`, and return `ruby.str_from_slice(&bytes)` (binary/ASCII-8BIT encoding).
- `decode_seq(data: RString) -> Result<Vec<String>, Error>`: copy the slice before any Ruby call, as
    in `encode_component`, then call `iscc_lib::decode_seq`.
- Register both with `define_module_function` in the codec block next to `iscc_decode`. Update the
    crate header docstring `Symbols (33 of 33)` to 35 and add both names. That list enumerates
    names, so it belongs in this step (IDv1 precedent).
- `test_conformance.rb` L13: `File.read(DATA_JSON, encoding: "UTF-8")`. Add a `bytes:` hex helper,
    because `decode_stream` strips only `stream:`. Add two loops in the file's existing
    `define_method` style: `encode_seq(inputs[0])` must equal the decoded bytes, and
    `decode_seq(bytes(inputs[0]))` must equal `outputs`. Each section holds 4 vectors with no error
    cases. Also update the header comment ("all 9 gen\_\*\_v0") to mention seq.
- `test_seq.rb` (Minitest, simple `test_` methods): a **byte** round trip
    `encode_seq(decode_seq(x)) == x` over every `decode_seq` vector; the `encode_seq` result has
    `Encoding::BINARY`; `decode_seq` returns an Array of Strings with the `ISCC:` prefix. Add
    negatives, **one bad input per assertion**: empty list (core "at least one ISCC-UNIT" message);
    a unit with a trailing `"\0junk"`; the case-fold alias `gaawdlqng6ſsdpf2`; a padded unit
    `ISCC:EEAYYNMFG5XOELK2====`; empty bytes `"".b`; truncated bytes (one vector minus its last
    byte). Core errors surface as `RuntimeError`.
- Measured: Ruby keeps an embedded NUL. `IsccLib.iscc_decompose("ISCC:AAAWKLHFXM75OAMK\0junk")`
    raises `RuntimeError` ("invalid symbol at 16"), so the `"\0junk"` negative should raise too. A
    non-String or non-Array argument raises a Magnus `TypeError`, not `RuntimeError`. Do not assert
    a class for marshalling failures unless you probe it.
- Rebuild the gitignored `.so` before testing: `cargo test` does not run Magnus code.
- This step changes bindings only, with no core or hot-path change, so it trips neither iai nor
    CRAP.

## Verification

- `cd crates/iscc-rb && bundle exec rake compile && env -u LANG -u LC_ALL bundle exec rake test`
    exits 0, and the output includes the seq conformance tests and `test_seq.rb`
- `cargo clippy -p iscc-rb -- -D warnings` exits 0
- `cd crates/iscc-rb && bundle exec standardrb` exits 0
- `grep -n '"encode_seq"\|"decode_seq"' crates/iscc-rb/src/lib.rs` shows both registrations
- `crates/iscc-rb/test/test_conformance.rb` contains `encoding: "UTF-8"` and reads both the
    `encode_seq` and `decode_seq` sections; `test_seq.rb` contains the `"\0junk"` negative
- `mise run check` exits 0

## Done When

The advance agent is done when all six verification criteria pass.
