"""Pin the single pointer form for vector inputs in the C++ wrapper.

``std::vector::data()`` may be NULL for an empty vector. ``iscc.hpp`` passes every input
vector through ``detail::safe_data``, so the C FFI always gets a non-null pointer. The
FFI reads a NULL pointer with a count of 0 as an empty input, so every wrapper behaves
the same with either pointer form and only a check on the source can hold the rule.
Pure Python, no compiler.
"""

from pathlib import Path

_HEADER = (
    Path(__file__).resolve().parents[1]
    / "packages"
    / "cpp"
    / "include"
    / "iscc"
    / "iscc.hpp"
)
_HELPER_LINE = "return v.empty() ? &sentinel : v.data();"


def data_call_lines(header_text: str) -> list[str]:
    """Return the stripped source lines of the header that call ``.data()``."""
    return [line.strip() for line in header_text.splitlines() if ".data()" in line]


def test_only_safe_data_calls_vector_data():
    """The one ``.data()`` call in ``iscc.hpp`` is the one inside ``detail::safe_data``."""
    assert data_call_lines(_HEADER.read_text(encoding="utf-8")) == [_HELPER_LINE]


def test_bare_data_call_is_reported():
    """A wrapper that passes ``ptrs.data()`` to the FFI shows up next to the helper line."""
    bare = "    detail::UniqueString s(iscc_gen_mixed_code_v0(ptrs.data(), codes.size(), bits));"
    assert data_call_lines(f"    {_HELPER_LINE}\n{bare}\n") == [
        _HELPER_LINE,
        bare.strip(),
    ]
