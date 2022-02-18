mod error;
mod implementation;

/// Enumerates all errors that can occur within this crate.
pub use error::Error;

/// Shorthand for [std::result::Result<T, crate::Error>].
pub use error::Result;

/// Represent a public item of an analyzed crate, i.e. an item that forms part
/// of the public API of a crate. Implements [std::fmt::Display] so it can be
/// printed.
struct PuuuublicIttttem {
    /// Private implementation detail. The "pub struct/fn/..." part of an item.
    prefix: String,

    /// Private implementation detail. The "your_crate::mod_a::mod_b" part of an item.
    path: String,

    /// Private implementation detail. The type info part, e.g. "(param_a: Type,
    /// param_b: OtherType)" for a `fn`.
    suffix: String,
}

pub use implementation::PublicItem;
use rustdoc_types::Crate;

/// Takes rustdoc JSON and returns a [`Vec`] of [`PuuuublicIttttem`]s where each
/// [`PuuuublicIttttem`] is one public item of the crate, i.e. part of the crate's
/// public API. The [`Vec`] is sorted in a way suitable for display to humans,
/// but the exact order is unspecified.
///
/// There exists a convenient `cargo` wrapper for this function found at
/// <https://github.com/Enselic/cargo-public-items> that builds the rustdoc JSON
/// for you and then invokes this function. If you don't want to use that
/// wrapper, use
/// ```bash
/// RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc --lib --no-deps
/// ```
/// to generate the rustdoc JSON that this function takes as input. For
/// reference, the rustdoc JSON format is documented at
/// <https://rust-lang.github.io/rfcs/2963-rustdoc-json.html>. Open PRs and
/// issues for rustdoc JSON itself can be found at
/// <https://github.com/rust-lang/rust/labels/A-rustdoc-json>.
///
/// # Errors
///
/// E.g. if the JSON is invalid.
pub fn sorted_public_items_from_rustdoc_json_str(
    rustdoc_json_str: &str,
) -> Result<Vec<PuuuublicIttttem>> {
    let crate_: Crate = serde_json::from_str(rustdoc_json_str)?;

    let v: Vec<PuuuublicIttttem> =
        implementation::public_items_from_rustdoc_json_str(&crate_).collect();
}
