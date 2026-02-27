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

## Hybrid Result Objects — Dict + Attribute Access

All `gen_*_v0` functions return `IsccResult` subclass instances that support both dict-style
(`result['iscc']`) and attribute-style (`result.iscc`) access. This provides IDE code completion
while remaining a drop-in replacement for code expecting plain dicts.

### Architecture

The implementation lives entirely in the Python wrapper layer (`__init__.py`). The Rust/PyO3
`_lowlevel` module continues to return plain `PyDict` objects — no Rust changes required.

**Base class** — a single `IsccResult(dict)` with `__getattr__` delegation:

```python
class IsccResult(dict):
    """ISCC result with both dict-style and attribute-style access."""

    def __getattr__(self, name):
        try:
            return self[name]
        except KeyError:
            raise AttributeError(name) from None
```

**Typed subclasses** — one per result shape, with class-level type annotations for IDE completion:

```python
class MetaCodeResult(IsccResult):
    """Result of gen_meta_code_v0."""

    iscc: str
    name: str
    metahash: str
    description: str | None
    meta: str | None


class TextCodeResult(IsccResult):
    """Result of gen_text_code_v0."""

    iscc: str
    characters: int
```

Result type classes:

| Class                | Annotated attributes                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------ |
| `MetaCodeResult`     | `iscc: str`, `name: str`, `metahash: str`, `description: str \| None`, `meta: str \| None` |
| `TextCodeResult`     | `iscc: str`, `characters: int`                                                             |
| `ImageCodeResult`    | `iscc: str`                                                                                |
| `AudioCodeResult`    | `iscc: str`                                                                                |
| `VideoCodeResult`    | `iscc: str`                                                                                |
| `MixedCodeResult`    | `iscc: str`, `parts: list[str]`                                                            |
| `DataCodeResult`     | `iscc: str`                                                                                |
| `InstanceCodeResult` | `iscc: str`, `datahash: str`, `filesize: int`                                              |
| `IsccCodeResult`     | `iscc: str`                                                                                |

**Wrapper functions** in `__init__.py` call `_lowlevel` and wrap the returned dict:

```python
def gen_meta_code_v0(name, description=None, meta=None, bits=64) -> MetaCodeResult:
    """Generate an ISCC Meta-Code from content metadata."""
    return MetaCodeResult(_gen_meta_code_v0(name, description, meta, bits))
```

### Capabilities

| Capability                    | Supported | Mechanism                     |
| ----------------------------- | --------- | ----------------------------- |
| `result['iscc']`              | yes       | inherited from `dict`         |
| `result.iscc`                 | yes       | `__getattr__` → `__getitem__` |
| IDE code completion           | yes       | class-level type annotations  |
| `isinstance(result, dict)`    | yes       | inherits from `dict`          |
| `json.dumps(result)`          | yes       | `dict` serialization          |
| `**result` unpacking          | yes       | `dict` protocol               |
| `for k in result` iteration   | yes       | `dict` iteration              |
| `result == {'iscc': '...'}`   | yes       | `dict.__eq__`                 |
| iscc-core drop-in replacement | yes       | passes all dict-based checks  |

### What does NOT change

- Rust core crate — structured `*CodeResult` types stay as-is
- PyO3 `_lowlevel` module — continues returning plain `PyDict`
- `_lowlevel.pyi` — stays as internal stubs with `-> dict[str, Any]`

### Exports

`__init__.py` exports the 9 `gen_*_v0` wrapper functions, `IsccResult`, and the 9 typed result
classes. All are listed in `__all__`.

## iscc-core Drop-in Compatibility Extensions

The following extensions are needed for `iscc-lib` to serve as a drop-in replacement for `iscc-core`
in `iscc-sdk`. Each addresses a specific API gap.

### PIL Pixel Data for gen_image_code_v0

GitHub: https://github.com/iscc/iscc-lib/issues/4

`gen_image_code_v0` must accept PIL's `ImagingCore` object (a `Sequence[int]`) in addition to
`bytes`. This is Python-specific (PIL doesn't exist in other bindings), so the conversion lives in
the Python wrapper only.

In `__init__.py`, widen `gen_image_code_v0` to accept
`bytes | bytearray | memoryview | Sequence[int]`. If `pixels` is not bytes-like, convert with
`pixels = bytes(pixels)`. Rust stays `&[u8]`.

**Verified when:**

- [ ] `gen_image_code_v0(Image.open(fp).convert("L").resize((32,32)).getdata())` works
- [ ] `gen_image_code_v0(bytes(pixels))` still works
- [ ] `gen_image_code_v0(list(pixels))` works
- [ ] Type stubs updated in `_lowlevel.pyi`

### Dict for gen_meta_code_v0 meta Parameter

GitHub: https://github.com/iscc/iscc-lib/issues/5

`gen_meta_code_v0` must accept `dict` for the `meta` parameter, matching iscc-core behavior. The
dict→JSON serialization is Python-specific; the JSON→data URL encoding uses the Rust core
`json_to_data_url()` utility.

In `__init__.py`, accept `meta: str | dict | None`. If `meta` is a `dict`, serialize to compact JSON
with `json.dumps(meta, separators=(',', ':'), ensure_ascii=False)`, then call `json_to_data_url()`
to produce the data URL string. Pass the resulting string to Rust.

**Verified when:**

- [ ] `gen_meta_code_v0(name="Test", meta={"key": "value"})` works
- [ ] `gen_meta_code_v0(name="Test", meta="data:application/json;base64,...")` still works
- [ ] Output matches iscc-core for identical dict input
- [ ] Type stubs updated

### encode_component, iscc_decode, and Type Enums

GitHub: https://github.com/iscc/iscc-lib/issues/6, https://github.com/iscc/iscc-lib/issues/7

The Rust core exposes `encode_component` and `iscc_decode` as Tier 1 functions with `u8` enum
fields. The Python binding provides idiomatic enum wrappers.

**Python-side additions:**

- `MT` — `enum.IntEnum` mapping MainType values (`META=0`, `SEMANTIC=1`, `CONTENT=2`, `DATA=3`,
    `INSTANCE=4`, `ISCC=5`, `ID=6`, `FLAKE=7`)
- `ST` — `enum.IntEnum` mapping SubType values (`NONE=0`, `TEXT=0`, `IMAGE=1`, `AUDIO=2`, `VIDEO=3`,
    `MIXED=4`)
- `VS` — `enum.IntEnum` mapping Version values (`V0=0`)
- `encode_component(mtype, stype, version, bit_length, digest)` — thin wrapper calling lowlevel
- `iscc_decode(iscc_unit)` — thin wrapper, returns `(MT, ST, VS, length_index, bytes)`

**Verified when:**

- [ ] `encode_component(MT.DATA, ST.NONE, VS.V0, 64, digest)` returns valid ISCC unit
- [ ] `iscc_decode("GABTQLB6CQ6ILWLO")` returns `(MT.DATA, ST.NONE, VS.V0, 1, digest_bytes)`
- [ ] Round-trip: decode(encode(...)) recovers original fields
- [ ] `MT`, `ST`, `VS` are `IntEnum` instances (support int comparison)
- [ ] Output matches iscc-core for all test cases
- [ ] All symbols exported in `__all__`

### core_opts Algorithm Constants

GitHub: https://github.com/iscc/iscc-lib/issues/8

Expose Rust core constants via Python for iscc-core API parity. Provide both module-level constants
and a `core_opts` namespace object.

**Python-side additions in `__init__.py`:**

```python
from types import SimpleNamespace

# Module-level constants (from Rust core)
META_TRIM_NAME = _lowlevel.META_TRIM_NAME  # 128
META_TRIM_DESCRIPTION = _lowlevel.META_TRIM_DESCRIPTION  # 4096
IO_READ_SIZE = _lowlevel.IO_READ_SIZE  # 4_194_304
TEXT_NGRAM_SIZE = _lowlevel.TEXT_NGRAM_SIZE  # 13

# iscc-core compatible namespace
core_opts = SimpleNamespace(
    meta_trim_name=META_TRIM_NAME,
    meta_trim_description=META_TRIM_DESCRIPTION,
    io_read_size=IO_READ_SIZE,
    text_ngram_size=TEXT_NGRAM_SIZE,
)
```

**Verified when:**

- [ ] `iscc_lib.META_TRIM_NAME == 128`
- [ ] `iscc_lib.core_opts.meta_trim_name == 128`
- [ ] `iscc_lib.core_opts.meta_trim_description == 4096`
- [ ] `iscc_lib.core_opts.io_read_size == 4_194_304`
- [ ] `iscc_lib.core_opts.text_ngram_size == 13`
- [ ] All constants and `core_opts` exported in `__all__`

## Type Stubs

The `_lowlevel.pyi` type stubs reflect the internal return types (`dict[str, Any]`) and input types
(`Union[bytes, BinaryIO]` for streaming functions). The public API types are defined inline in
`__init__.py` — no separate `.pyi` needed since it is pure Python.

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
- [ ] All `gen_*_v0` return `IsccResult` subclass instances (not plain dicts)
- [ ] `result.iscc` attribute access works for all result types
- [ ] `result['iscc']` dict access still works (backward compatible)
- [ ] `isinstance(result, dict)` is `True`
- [ ] IDE code completion works via class-level type annotations (pyright/mypy clean)
- [ ] `json.dumps(result)` works without custom serializer
- [ ] `IsccResult` and all 9 typed result classes exported in `__all__`
