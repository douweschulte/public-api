use rustdoc_types::Crate;

use crate::Result;

mod item_iterator;

use item_iterator::ItemIterator;

pub use item_iterator::PublicItem;

pub fn sorted_public_items_from_rustdoc_json_str(
    rustdoc_json_str: &str,
) -> Result<Vec<PublicItem>> {
    let crate_: Crate = serde_json::from_str(rustdoc_json_str)?;

    let mut result: Vec<PublicItem> = ItemIterator::new(&crate_).collect();

    result.sort();

    Ok(result)
}
