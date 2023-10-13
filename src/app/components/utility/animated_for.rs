#![allow(clippy::disallowed_macros)]

use std::{collections::HashMap, fmt, hash::Hash, rc::Rc};

use leptos::{
    html::ElementDescriptor,
    leptos_dom::{tracing, Each},
    logging::log,
    update,
    IntoView,
    MaybeProp,
    NodeRef,
    StoredValue,
    View,
};
use wasm_bindgen::JsCast;

// @kw refactor
fn sync_el_per_key<Item, ChildFn, ChildEl, Child, KeyFn, Key>(
    el_per_key: StoredValue<HashMap<Key, web_sys::HtmlElement>>,
    key_fn: KeyFn,
    children_fn: ChildFn,
) -> (
    impl Fn(&Item) -> Key + 'static,
    impl Fn(Item) -> View + 'static,
)
where
    ChildFn: Fn(Item) -> (NodeRef<ChildEl>, Child) + 'static,
    ChildEl: ElementDescriptor + Clone + 'static,
    Child: IntoView + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + 'static,
    Item: 'static,
{
    let key_fn = Rc::new(key_fn);

    (
        {
            let key_fn = Rc::clone(&key_fn);
            move |item| key_fn(item)
        },
        move |item| {
            let key = key_fn(&item);
            let (node_ref, child) = children_fn(item);

            node_ref.on_load(move |_| {
                let el = node_ref
                    .get()
                    .unwrap()
                    .into_any()
                    .dyn_ref::<web_sys::HtmlElement>()
                    .unwrap()
                    .clone();

                update!(|el_per_key| {
                    el_per_key.insert(key, el);
                });
            });

            child.into_view()
        },
    )
}

#[leptos::component]
pub fn AnimatedFor<Items, ItemIter, Item, ChildFn, Child, ChildEl, KeyFn, Key>(
    each: Items,
    key: KeyFn,
    children: ChildFn,
    #[prop(optional, into)] appear: MaybeProp<bool>,
    #[prop(optional, into)] move_class: MaybeProp<String>,
    #[prop(optional, into)] enter_class: MaybeProp<String>,
    #[prop(optional, into)] enter_from_class: MaybeProp<String>,
    #[prop(optional, into)] leave_class: MaybeProp<String>,
) -> impl IntoView
where
    Items: Fn() -> ItemIter + 'static,
    ItemIter: IntoIterator<Item = Item> + 'static,
    ChildFn: Fn(Item) -> (NodeRef<ChildEl>, Child) + 'static,
    Child: IntoView + 'static,
    ChildEl: ElementDescriptor + Clone + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + fmt::Debug + Clone + 'static,
    Item: 'static,
{
    let el_per_key =
        StoredValue::new(HashMap::<Key, web_sys::HtmlElement>::new());

    let (key_fn, children_fn) = sync_el_per_key(el_per_key, key, children);

    let each_fn = move || {
        for (key, el) in el_per_key.get_value() {
            let rect = el.get_bounding_client_rect();
            let x = rect.x();
            let y = rect.y();
            log!("{key:?}: {x}, {y}");
        }

        each()
    };

    Each::new(each_fn, key_fn, children_fn).into_view()
}
