//! Magnus bridge for iscc-lib Ruby bindings.
//!
//! Exposes a subset of Tier 1 ISCC functions as Ruby module functions
//! under the `IsccLib` module. The pure Ruby wrapper in `lib/iscc_lib.rb`
//! provides idiomatic result classes and keyword arguments.
//!
//! Initial symbols (~10 of 32):
//! - `gen_meta_code_v0` (flagship gen function with Hash return + optional params)
//! - `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
//! - Constants: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META,
//!   IO_READ_SIZE, TEXT_NGRAM_SIZE

use magnus::{Error, RHash, Ruby, function, prelude::*};

/// Map an `IsccError` to a Magnus `RuntimeError`.
fn to_magnus_err(e: iscc_lib::IsccError) -> Error {
    Error::new(magnus::exception::runtime_error(), e.to_string())
}

/// Generate a Meta-Code from name and optional metadata.
///
/// Returns a Ruby Hash with keys: `iscc`, `name`, `metahash`, and optionally
/// `description` and `meta`.
fn gen_meta_code_v0(
    name: String,
    description: Option<String>,
    meta: Option<String>,
    bits: u32,
) -> Result<RHash, Error> {
    let r = iscc_lib::gen_meta_code_v0(&name, description.as_deref(), meta.as_deref(), bits)
        .map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("name", r.name)?;
    hash.aset("metahash", r.metahash)?;
    if let Some(desc) = r.description {
        hash.aset("description", desc)?;
    }
    if let Some(meta) = r.meta {
        hash.aset("meta", meta)?;
    }
    Ok(hash)
}

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
fn text_clean(text: String) -> String {
    iscc_lib::text_clean(&text)
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
fn text_remove_newlines(text: String) -> String {
    iscc_lib::text_remove_newlines(&text)
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
fn text_trim(text: String, nbytes: usize) -> String {
    iscc_lib::text_trim(&text, nbytes)
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and certain
/// Unicode categories, then recombines with NFKC normalization.
fn text_collapse(text: String) -> String {
    iscc_lib::text_collapse(&text)
}

/// Initialize the IsccLib Ruby module with all bridge functions and constants.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("IsccLib")?;

    // Gen functions (prefixed with _ for Ruby wrapper layer)
    module.define_module_function("_gen_meta_code_v0", function!(gen_meta_code_v0, 4))?;

    // Text utility functions
    module.define_module_function("text_clean", function!(text_clean, 1))?;
    module.define_module_function("text_remove_newlines", function!(text_remove_newlines, 1))?;
    module.define_module_function("text_trim", function!(text_trim, 2))?;
    module.define_module_function("text_collapse", function!(text_collapse, 1))?;

    // Constants
    module.const_set("META_TRIM_NAME", iscc_lib::META_TRIM_NAME)?;
    module.const_set("META_TRIM_DESCRIPTION", iscc_lib::META_TRIM_DESCRIPTION)?;
    module.const_set("META_TRIM_META", iscc_lib::META_TRIM_META)?;
    module.const_set("IO_READ_SIZE", iscc_lib::IO_READ_SIZE)?;
    module.const_set("TEXT_NGRAM_SIZE", iscc_lib::TEXT_NGRAM_SIZE)?;

    Ok(())
}
