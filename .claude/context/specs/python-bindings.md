# Spec: Python Bindings — Drop-in Compatibility with iscc-core

The Python package `iscc_lib` is a drop-in replacement for `iscc-core`. Callers can replace
`import iscc_core as ic` with `import iscc_lib as ic` and get identical behavior for all 9
`gen_*_v0` functions. Use deepwiki MCP to query `iscc/iscc-core` for exact signatures and return
values.

## Return Types

All `gen_*_v0` functions return `dict` with exactly the same keys and value types as iscc-core:

| Function               | Dict keys                                                        |
| ---------------------- | ---------------------------------------------------------------- |
| `gen_meta_code_v0`     | `iscc`, `name`, `metahash`, and optionally `description`, `meta` |
| `gen_text_code_v0`     | `iscc`, `characters`                                             |
| `gen_image_code_v0`    | `iscc`                                                           |
| `gen_audio_code_v0`    | `iscc`                                                           |
| `gen_video_code_v0`    | `iscc`                                                           |
| `gen_mixed_code_v0`    | `iscc`, `parts`                                                  |
| `gen_data_code_v0`     | `iscc`                                                           |
| `gen_instance_code_v0` | `iscc`, `datahash`, `filesize`                                   |
| `gen_iscc_code_v0`     | `iscc`                                                           |

Field definitions:

- `iscc`: ISCC code string (e.g., `"ISCC:AAAZXZ6OU74YAZIM"`)
- `metahash`: hex-encoded BLAKE3 multihash with `1e20` prefix of the metadata payload
- `name`: normalized name string
- `description`: normalized description string (only present when description was provided)
- `meta`: metadata as Data-URL string (only present when meta was provided)
- `characters`: character count of text after normalization
- `parts`: list of input Content-Code strings (passed through unchanged)
- `datahash`: hex-encoded BLAKE3 multihash with `1e20` prefix of the instance data
- `filesize`: byte length of the input data

## Structured Return Types in Rust Core

The Rust core API returns structured result types (not plain strings) that carry the same additional
fields as iscc-core dicts. All binding crates (Python, Node.js, WASM, C FFI) have access to these
structured results and can expose them idiomatically in their target language. The Rust API
documentation reflects the structured return types.

## Input Types for Streaming Functions

`gen_data_code_v0` and `gen_instance_code_v0` accept both `bytes` and file-like objects (anything
with a `.read()` method), matching iscc-core's `Stream` type
(`Union[BinaryIO, mmap.mmap, BytesIO, BufferedReader]`).

## Type Stubs

The `.pyi` type stubs reflect the actual return types (`dict[str, Any]`) and input types
(`Union[bytes, BinaryIO]` for streaming functions).

## Verification Criteria

- [ ] All 9 `gen_*_v0` return `dict` with correct keys matching iscc-core
- [ ] `result["iscc"]` matches iscc-core output for all conformance vectors
- [ ] `gen_meta_code_v0`: `result["metahash"]` and `result["name"]` match iscc-core
- [ ] `gen_text_code_v0`: `result["characters"]` matches iscc-core
- [ ] `gen_instance_code_v0`: `result["datahash"]` and `result["filesize"]` match iscc-core
- [ ] `gen_mixed_code_v0`: `result["parts"]` matches iscc-core
- [ ] `gen_data_code_v0(BytesIO(b"data"))` works (file-like input)
- [ ] `gen_instance_code_v0(BytesIO(b"data"))` works (file-like input)
- [ ] Conformance tests pass using `result["iscc"]` access pattern
- [ ] Type stubs reflect `dict` return types and `Union[bytes, BinaryIO]` input types
- [ ] `ruff check` and `ruff format --check` clean
- [ ] Rust core API returns structured types with the additional fields
