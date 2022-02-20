use std::{collections::HashMap, rc::Rc};

use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Type};

use super::intermediate_public_item::IntermediatePublicItem;

type Impls<'a> = HashMap<&'a Id, Vec<&'a Impl>>;

/// Iterates over all items in a crate. Iterating over items has the benefit of
/// behaving properly when:
/// 1. A single item is imported several times.
/// 2. An item is (publicly) imported from another crate
///
/// Note that this implementation iterates over everything (with the exception
/// of `impl`s, see relevant code for more details), so if the rustdoc JSON is
/// generated with `--document-private-items`, then private items will also be
/// included in the output.
pub struct ItemIterator<'a> {
    /// The entire rustdoc JSON data
    crate_: &'a Crate,

    /// What items left to visit (and possibly add more items from)
    items_left: Vec<Rc<IntermediatePublicItem<'a>>>,

    /// Normally, a reference in the rustdoc JSON exists. If
    /// [Self::crate_.index] is missing an id (e.g. if it is for a dependency
    /// but the rustdoc JSON was built with `--no-deps`) then we track that in
    /// this field.
    missing_ids: Vec<&'a Id>,

    /// `impl`s are a bit special. They do not need to be reachable by the crate
    /// root in order to matter. All that matters is that the trait and type
    /// involved are both public. Since the rustdoc JSON by definition only
    /// includes public items, all `impl`s we see are relevant. Whenever we
    /// encounter a type that has an `impl`, we inject the associated items of
    /// the `impl` as children of the type.
    impls: Impls<'a>,
}

impl<'a> ItemIterator<'a> {
    pub fn new(crate_: &'a Crate) -> Self {
        let mut s = ItemIterator {
            crate_,
            items_left: vec![],
            missing_ids: vec![],
            impls: find_all_impls(crate_),
        };

        // Bootstrap with the root item
        s.try_add_item_to_visit(&crate_.root, &None, None);

        s
    }

    fn add_children_for_item(&mut self, public_item: &Rc<IntermediatePublicItem<'a>>) {
        // Handle any impls. See [`ItemIterator::impls`] docs for more info.
        let mut add_after_borrow = vec![];
        if let Some(impls) = self.impls.get(&public_item.item.id) {
            for impl_ in impls {
                for id in &impl_.items {
                    add_after_borrow.push((id, &impl_.trait_));
                }
            }
        }
        for id_and_trait in add_after_borrow {
            self.try_add_item_to_visit(id_and_trait.0, id_and_trait.1, Some(public_item.clone()));
        }

        // Handle regular children of the item
        for child in items_in_container(public_item.item).into_iter().flatten() {
            self.try_add_item_to_visit(child, &None, Some(public_item.clone()));
        }
    }

    fn try_add_item_to_visit(
        &mut self,
        id: &'a Id,
        as_trait: &'a Option<Type>,
        parent: Option<Rc<IntermediatePublicItem<'a>>>,
    ) {
        match self.crate_.index.get(id) {
            // We handle `impl`s specially, and we don't want to process `impl`
            // items directly. See [`ItemIterator::impls`] docs for more info.
            Some(Item {
                inner: ItemEnum::Impl { .. },
                ..
            }) => (),

            Some(item) => self
                .items_left
                .push(Rc::new(IntermediatePublicItem::new(item, as_trait, parent))),

            None => self.missing_ids.push(id),
        }
    }
}

impl<'a> Iterator for ItemIterator<'a> {
    type Item = Rc<IntermediatePublicItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        if let Some(public_item) = self.items_left.pop() {
            self.add_children_for_item(&public_item.clone());

            result = Some(public_item);
        }

        result
    }
}

/// `impl`s are special. This helper finds all `impl`s. See
/// [`ItemIterator::impls`] docs for more info.
fn find_all_impls(crate_: &Crate) -> Impls {
    let mut impls = HashMap::new();

    for item in crate_.index.values() {
        if let ItemEnum::Impl(impl_) = &item.inner {
            if let Impl {
                for_: Type::ResolvedPath { id, .. },
                ..
            } = impl_
            {
                impls.entry(id).or_insert_with(Vec::new).push(impl_);
            }
        }
    }

    impls
}

/// Some items contain other items, which is relevant for analysis. Keep track
/// of such relationships.
fn items_in_container(item: &Item) -> Option<&Vec<Id>> {
    match &item.inner {
        ItemEnum::Module(m) => Some(&m.items),
        ItemEnum::Union(u) => Some(&u.fields),
        ItemEnum::Struct(s) => Some(&s.fields),
        ItemEnum::Enum(e) => Some(&e.variants),
        ItemEnum::Trait(t) => Some(&t.items),
        ItemEnum::Impl(i) => Some(&i.items),
        ItemEnum::Variant(rustdoc_types::Variant::Struct(ids)) => Some(ids),
        // TODO: `ItemEnum::Variant(rustdoc_types::Variant::Tuple(ids)) => Some(ids),` when https://github.com/rust-lang/rust/issues/92945 is fixed
        _ => None,
    }
}
