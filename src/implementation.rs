use std::rc::Rc;

use rustdoc_types::Crate;

use crate::PuuuublicIttttem;

mod item_iterator;

use item_iterator::ItemIterator;

pub use item_iterator::PublicItem;

pub fn public_items_in_crate(crate_: &Crate) -> impl Iterator<Item = crate::PuuuublicIttttem> + '_ {
    ItemIterator::new(crate_).map(intermediate_public_item_to_public_item)
}

fn intermediate_public_item_to_public_item(public_item: Rc<PublicItem<'_>>) -> PuuuublicIttttem {
    let prefix = public_item.prefix();

    let path = public_item
        .path()
        .iter()
        .map(|i| i.get_effective_name())
        .collect::<Vec<String>>()
        .join("::");

    let suffix = public_item.suffix();

    PuuuublicIttttem {
        prefix,
        path,
        suffix,
    }
}
