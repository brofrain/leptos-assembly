#![allow(clippy::disallowed_macros)]

use std::{collections::HashMap, hash::Hash, rc::Rc};

use leptos::{
    create_node_ref,
    html::{AnyElement, Div},
    leptos_dom::{tracing, Each},
    update,
    view,
    with,
    Effect,
    IntoView,
    MaybeProp,
    NodeRef,
    StoredValue,
    View,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};

// @kw refactor
fn sync_el_per_key<Item, ChildFn, Child, KeyFn, Key>(
    el_per_key: StoredValue<HashMap<Key, HtmlElement>>,
    key_fn: KeyFn,
    children_fn: ChildFn,
) -> (
    impl Fn(&Item) -> Key + 'static,
    impl Fn(Item) -> View + 'static,
)
where
    ChildFn: Fn(Item) -> Child + 'static,
    Child: IntoView + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + 'static,
    Item: 'static,
{
    let key_fn = Rc::new(key_fn);

    (
        move |item| key_fn(item),
        move |item| {
            let child_view = children_fn(item);
            let node_ref = create_node_ref::<Div>();
            let wrapped = view! { <div _ref=node_ref>{child_view}</div> };

            node_ref.on_load(move |el| {
                el.on_mount(move |el| {
                    let parent =
                        el.parent_node().expect("<AnimatedFor> has no parent");
                    let unwrapped_el = el
                        .first_element_child()
                        .expect("<AnimatedFor> has invalid children");

                    parent
                        .append_child(
                            unwrapped_el
                                .dyn_ref::<Node>()
                                .expect("<AnimatedFor> has invalid children"),
                        )
                        .expect("Failed to unwrap <AnimatedFor> children");

                    let wrapper_node = el
                        .dyn_ref::<Node>()
                        .expect("<AnimatedFor> has invalid children");

                    parent.remove_child(wrapper_node).unwrap();
                });
            });

            wrapped.into_view()
        },
    )
}

#[leptos::component]
pub fn AnimatedFor<Items, ItemIter, Item, ChildFn, Child, KeyFn, Key>(
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
    ItemIter: IntoIterator<Item = Item>,
    ChildFn: Fn(Item) -> Child + 'static,
    Child: IntoView + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + 'static,
    Item: 'static,
{
    let el_per_key = StoredValue::new(HashMap::<Key, HtmlElement>::new());

    let (key_fn, children_fn) = sync_el_per_key(el_per_key, key, children);

    Each::new(each, key_fn, children_fn).into_view()
}
