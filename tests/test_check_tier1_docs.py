"""Tests for the Tier 1 doc-parity gate in ``scripts/check_tier1_docs.py``.

The anchor test runs the check against the real tree and asserts zero errors — this
carries the gate into CI (CI runs pytest, never prek) and catches deletions the
``files:``-scoped hook never sees. The fire tests copy ``lib.rs`` and every inspected
tracked Markdown file into ``tmp_path``, inject exactly one defect, and assert the
complete error list.
"""

import importlib.util
import re
import shutil
from pathlib import Path

# Load scripts/check_tier1_docs.py by path — it is a repo gate script, not a package.
_SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "check_tier1_docs.py"
_spec = importlib.util.spec_from_file_location("check_tier1_docs", _SCRIPT_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
ct = importlib.util.module_from_spec(_spec)
_loader.exec_module(ct)

TRACKED = ct.tracked_markdown(ct.ROOT)

SYNTHETIC_LIB = """\
pub mod codec;
pub(crate) mod cdc;
pub use cdc::alg_cdc_chunks;
pub use simhash::{alg_simhash, sliding_window};
pub use streaming::{
    DataHasher,
};
pub use types::*;
#[cfg(feature = "meta-code")]
pub const META_TRIM_NAME: usize = 128;
pub const fn const_helper() -> u32 { 1 }
pub enum IsccError { InvalidInput(String) }
pub type IsccResult<T> = Result<T, IsccError>;
pub fn gen_meta_code_v0(
    name: &str,
) -> IsccResult<String> {
    fn inner_helper() {}
}
fn private_fn() {}
mod tests {
    pub fn indented_fn() {}
}
"""


def _copy_tree(tmp_path):
    """Copy lib.rs and every inspected tracked Markdown file into tmp_path."""
    for rel in [ct.LIB_RS, *(p for p in TRACKED if not ct.is_skipped(p))]:
        dest = tmp_path / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ct.ROOT / rel, dest)
    return tmp_path


def _edit(root, rel, old, new):
    """Replace the single occurrence of `old` with `new` in root/rel."""
    path = root / rel
    text = path.read_text(encoding="utf-8")
    assert text.count(old) == 1
    path.write_text(text.replace(old, new), encoding="utf-8")


def _symbols():
    """Return the Tier 1 symbols derived from the real lib.rs."""
    return ct.derive_symbols((ct.ROOT / ct.LIB_RS).read_text(encoding="utf-8"))


def test_real_repo_passes():
    # The gate passes on the repository as committed, and main() (the prek hook
    # entry point) exits 0.
    assert ct.run_checks(ct.ROOT, TRACKED) == []
    assert ct.main() == 0


def test_real_lib_rs_derives_every_item_class():
    # Functions, constants, and both `pub use` forms all reach the derived set;
    # the result structs from the `types::*` glob and `IsccError` do not.
    symbols = _symbols()
    assert {"gen_meta_code_v0", "gen_sum_code_v0", "iscc_decode"} <= symbols
    assert {"META_TRIM_META", "IO_READ_SIZE", "TEXT_NGRAM_SIZE"} <= symbols
    assert {"alg_cdc_chunks", "sliding_window", "DataHasher", "text_trim"} <= symbols
    assert not {"IsccError", "IsccResult", "DataCodeResult", "codec"} & symbols


def test_registry_paths_are_tracked_and_disjoint():
    # Every registered doc is tracked, and no path sits in two registry sets.
    registered = [*ct.FULL, *ct.EXACT, *ct.EXEMPT]
    assert len(registered) == len(set(registered))
    assert set(registered) <= set(TRACKED)


def test_derive_symbols_synthetic():
    assert ct.derive_symbols(SYNTHETIC_LIB) == {
        "alg_cdc_chunks",
        "alg_simhash",
        "sliding_window",
        "DataHasher",
        "META_TRIM_NAME",
        "const_helper",
        "gen_meta_code_v0",
    }


def test_symbol_matches_every_binding_casing():
    for spelling in (
        "gen_sum_code_v0",
        "genSumCodeV0",
        "GenSumCodeV0",
        "iscc_gen_sum_code_v0",
    ):
        assert ct.symbol_in("gen_sum_code_v0", ct.doc_tokens(f"call {spelling}()"))
    for spelling in (
        "META_TRIM_META",
        "MetaTrimMeta",
        "metaTrimMeta",
        "iscc_meta_trim_meta",
    ):
        assert ct.symbol_in("META_TRIM_META", ct.doc_tokens(f"`{spelling}`"))
    for spelling in (
        "NewDataHasher",
        "iscc_data_hasher_new",
        "dataHasherNew",
        "DataHasher",
    ):
        assert ct.symbol_in("DataHasher", ct.doc_tokens(spelling))


def test_symbol_match_is_never_substring():
    # `isccdecode` occurs inside `IsccDecodeResult`, which must not satisfy it;
    # only types get prefix/suffix matching.
    assert not ct.symbol_in("iscc_decode", ct.doc_tokens("IsccDecodeResult"))
    assert not ct.symbol_in("text_trim", ct.doc_tokens("text_trimmed"))
    assert not ct.symbol_in("DataHasher", ct.doc_tokens("MyDataHashers"))


def test_fire_pub_fn_added_to_lib_rs(tmp_path):
    # A Tier 1 function added at the crate root is missing from every FULL and
    # EXACT doc, and from nothing else.
    root = _copy_tree(tmp_path)
    _edit(
        root,
        ct.LIB_RS,
        "/// Max UTF-8 byte length for name metadata trimming.",
        "pub fn gen_ghost_v0() {}\n\n/// Max UTF-8 byte length for name metadata trimming.",
    )
    expected = [
        f"{p}: missing Tier 1 symbols: gen_ghost_v0"
        if p in ct.FULL
        else f"{p}: tier1 block missing: gen_ghost_v0"
        for p in TRACKED
        if p in ct.FULL or p in ct.EXACT
    ]
    assert len(expected) == len(ct.FULL) + len(ct.EXACT)
    assert ct.run_checks(root, TRACKED) == expected


def test_fire_symbol_deleted_from_full_doc(tmp_path):
    # Every spelling of META_TRIM_META is removed from one FULL doc.
    root = _copy_tree(tmp_path)
    rel = "docs/howto/python.md"
    path = root / rel
    keys = {"metatrimmeta", "isccmetatrimmeta"}
    text = path.read_text(encoding="utf-8")
    stripped = re.sub(
        r"[A-Za-z_]\w*", lambda m: "removed" if ct.norm(m[0]) in keys else m[0], text
    )
    assert stripped != text
    path.write_text(stripped, encoding="utf-8")
    assert ct.run_checks(root, TRACKED) == [
        f"{rel}: missing Tier 1 symbols: META_TRIM_META"
    ]


def test_fire_extra_name_in_exact_block(tmp_path):
    root = _copy_tree(tmp_path)
    rel = "crates/iscc-lib/CLAUDE.md"
    _edit(root, rel, ct.BEGIN_MARKER, f"{ct.BEGIN_MARKER}\n\n- ghost: `gen_ghost_v0`")
    assert ct.run_checks(root, TRACKED) == [f"{rel}: tier1 block extra: gen_ghost_v0"]


def test_fire_repeated_name_in_exact_block(tmp_path):
    # Set equality alone would pass a duplicated row.
    root = _copy_tree(tmp_path)
    rel = "crates/iscc-lib/CLAUDE.md"
    _edit(root, rel, ct.BEGIN_MARKER, f"{ct.BEGIN_MARKER}\n\n- again: `text_trim`")
    assert ct.run_checks(root, TRACKED) == [f"{rel}: tier1 block repeats: text_trim"]


def test_fire_missing_exact_markers(tmp_path):
    root = _copy_tree(tmp_path)
    rel = ".claude/context/specs/rust-core.md"
    _edit(root, rel, ct.END_MARKER, "")
    assert ct.run_checks(root, TRACKED) == [
        f"{rel}: needs exactly one {ct.BEGIN_MARKER} ... {ct.END_MARKER} block"
    ]


def test_fire_unregistered_doc_over_threshold(tmp_path):
    # A doc naming DISCOVERY_THRESHOLD symbols must be registered; one fewer passes.
    root = _copy_tree(tmp_path)
    names = sorted(_symbols())[: ct.DISCOVERY_THRESHOLD]
    rel = "docs/extra.md"
    (root / rel).write_text("".join(f"- `{n}`\n" for n in names), encoding="utf-8")
    assert ct.run_checks(root, [*TRACKED, rel]) == [
        (
            f"{rel}: names {ct.DISCOVERY_THRESHOLD} Tier 1 symbols but is not registered "
            "(add it to FULL, EXACT or EXEMPT in scripts/check_tier1_docs.py)"
        )
    ]
    (root / rel).write_text("".join(f"- `{n}`\n" for n in names[1:]), encoding="utf-8")
    assert ct.run_checks(root, [*TRACKED, rel]) == []


def test_fire_registered_doc_not_tracked(tmp_path):
    root = _copy_tree(tmp_path)
    rel = "docs/howto/go.md"
    tracked = [p for p in TRACKED if p != rel]
    assert ct.run_checks(root, tracked) == [f"{rel}: registered but not tracked"]


def test_floor_rejects_collapsed_parse(tmp_path):
    # A lib.rs whose items no longer parse must not read as a pass.
    root = _copy_tree(tmp_path)
    lib = root / ct.LIB_RS
    lib.write_text(lib.read_text(encoding="utf-8").replace("\npub ", "\npub(crate) "))
    errors = ct.run_checks(root, TRACKED)
    assert len(errors) == 1
    assert errors[0].startswith(f"floor: {ct.LIB_RS} parse lost ")


def test_main_exits_nonzero_on_finding(tmp_path, monkeypatch, capsys):
    root = _copy_tree(tmp_path)
    rel = "crates/iscc-lib/CLAUDE.md"
    _edit(root, rel, ct.BEGIN_MARKER, f"{ct.BEGIN_MARKER}\n\n- ghost: `gen_ghost_v0`")
    monkeypatch.setattr(ct, "ROOT", root)
    monkeypatch.setattr(ct, "tracked_markdown", lambda _root: TRACKED)
    assert ct.main() == 1
    assert f"error: {rel}: tier1 block extra: gen_ghost_v0" in capsys.readouterr().out
