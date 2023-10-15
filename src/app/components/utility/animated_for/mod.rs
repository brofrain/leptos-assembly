#![allow(clippy::disallowed_macros)]

use std::{collections::HashMap, fmt, hash::Hash, rc::Rc};

use leptos::{
    html::ElementDescriptor,
    leptos_dom::{tracing, Each},
    logging::log,
    update,
    with,
    Callback,
    IntoView,
    MaybeProp,
    Memo,
    NodeRef,
    StoredValue,
    View,
};
use wasm_bindgen::JsCast;
use web_sys::DomRect;

use crate::utils::animation::{clear_cb_on_transition_end, AnimatedEl};

trait AnimatedForEl {
    fn clear_transform(&self);
}

impl AnimatedForEl for web_sys::HtmlElement {
    fn clear_transform(&self) {
        self.style().set_property("transform", "").unwrap();
    }
}

fn lock_fixed_position(
    el: &web_sys::HtmlElement,
    el_pos: &DomRect,
    document_pos: &DomRect,
) {
    let top = el_pos.top() - document_pos.top();
    let left = el_pos.left() - document_pos.left();
    let width = el_pos.width();
    let height = el_pos.height();

    el.set_important_style_property("position", "fixed");
    el.set_important_style_property("margin", "0px");
    el.set_important_style_property("top", &format!("{top}px"));
    el.set_important_style_property("left", &format!("{left}px"));
    el.set_important_style_property("width", &format!("{width}px"));
    el.set_important_style_property("height", &format!("{height}px"));

    el.enable_instant_transition();
}

fn check_if_moved_and_lock_previous_position(
    el: &web_sys::HtmlElement,
    new_pos: &DomRect,
    old_pos: &DomRect,
) -> bool {
    let dx = old_pos.left() - new_pos.left();
    let dy = old_pos.top() - new_pos.top();

    if dx != 0.0 || dy != 0.0 {
        el.set_important_style_property(
            "transform",
            &format!("translate({dx}px,{dy}px)"),
        );
        el.enable_instant_transition();

        return true;
    }

    false
}

fn use_keyed_elements<Item, ChildFn, ChildEl, Child, KeyFn, Key>(
    key_fn: KeyFn,
    children_fn: ChildFn,
) -> (
    StoredValue<HashMap<Key, web_sys::HtmlElement>>,
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
    let el_per_key =
        StoredValue::new(HashMap::<Key, web_sys::HtmlElement>::new());

    let key_fn = Rc::new(key_fn);

    (
        el_per_key,
        {
            let key_fn = Rc::clone(&key_fn);
            move |item| key_fn(item)
        },
        move |item| {
            let key = key_fn(&item);
            let (node_ref, child) = children_fn(item);

            // @kw add .expect(...)
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
pub fn AnimatedFor<Items, ItemIter, Item, Child, ChildEl, ChildFn, Key, KeyFn>(
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
    Child: IntoView + 'static,
    ChildEl: ElementDescriptor + Clone + 'static,
    ChildFn: Fn(Item) -> (NodeRef<ChildEl>, Child) + 'static,
    Key: Eq + Hash + fmt::Debug + 'static, // @kw
    KeyFn: Fn(&Item) -> Key + 'static,
    Item: 'static,
{
    let appear = Memo::new(move |_| appear().unwrap_or_default());

    let build_clear_transition = Callback::new({
        move |()| {
            // todo @kw build classes to remove

            move |el: &web_sys::HtmlElement| {
                // el.remove_classes(&move_class);
                // el.remove_classes(&enter_class);
                // el.remove_classes(&enter_from_class);
                clear_cb_on_transition_end(el);
            }
        }
    });

    let (el_per_key, key_fn, children_fn) = use_keyed_elements(key, children);

    let each_fn = move || {
        // @kw
        with!(|el_per_key| {
            for (key, el) in el_per_key {
                let rect = el.get_bounding_client_rect();
                let x = rect.x();
                let y = rect.y();
                log!("{key:?}: {x}, {y}");
            }
        });

        each()
    };

    Each::new(each_fn, key_fn, children_fn).into_view()
}
