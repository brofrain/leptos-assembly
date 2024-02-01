use std::hash::Hash;

use common::vendor::client::prelude::*;
use leptos_animated_for::AnimatedFor as RawAnimatedFor;

#[component]
pub fn BaseAnimatedFor<Items, ItemIter, Item, Child, ChildFn, Key, KeyFn>(
    each: Items,
    key: KeyFn,
    children: ChildFn,
) -> impl IntoView
where
    Items: Fn() -> ItemIter + 'static,
    ItemIter: IntoIterator<Item = Item> + 'static,
    Child: IntoView + 'static,
    ChildFn: Fn(Item) -> Child + 'static,
    Key: Eq + Hash + Clone + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Item: 'static,
{
    view! {
        <RawAnimatedFor
            each=each
            key=key
            children=children
            appear=true
            move_class="transition-400"
            enter_class="transition-opacity"
            enter_from_class="op0"
            leave_class="transition-opacity op0 pointer-events-none"
        />
    }
}
