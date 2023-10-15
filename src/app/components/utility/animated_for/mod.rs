#![allow(clippy::disallowed_macros)]

use std::{collections::HashMap, fmt, hash::Hash, rc::Rc};

use leptos::{
    leptos_dom::{tracing, Each},
    logging::log,
    spawn_local,
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

use crate::utils::{
    animation::{
        clear_cb_on_transition_end,
        set_cb_once_on_transition_end,
        AnimatedEl,
        Classes,
    },
    future::next_tick,
};

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

fn build_clear_transition(
    active_transition_classes: &Classes,
) -> impl Fn(&web_sys::HtmlElement) {
    let classes_to_remove = active_transition_classes.clone();
    move |el| {
        el.remove_classes(&classes_to_remove);
        clear_cb_on_transition_end(el);
    }
}

fn build_clear_transition_on_transition_end(
    active_transition_classes: &Classes,
) -> impl Fn(&web_sys::HtmlElement) {
    let clear_transition =
        Rc::new(build_clear_transition(active_transition_classes));
    move |el| {
        let clear_transition = Rc::clone(&clear_transition);
        set_cb_once_on_transition_end(el, move |el| {
            clear_transition(el);
        });
    }
}

fn build_release_transition(
    active_transition_classes: &Classes,
) -> impl Fn(&web_sys::HtmlElement) {
    let clear_transition_on_transition_end =
        build_clear_transition_on_transition_end(active_transition_classes);

    move |el: &web_sys::HtmlElement| {
        el.clear_transform();
        el.disable_instant_transition();
        clear_transition_on_transition_end(el);
    }
}

fn build_start_enter(
    enter_from_class: Memo<Classes>,
    enter_class: Memo<Classes>,
) -> impl Fn(&web_sys::HtmlElement) {
    let enter_from_class = enter_from_class();
    let enter_class = enter_class();
    let release_transition = build_release_transition(&enter_class);

    move |el: &web_sys::HtmlElement| {
        el.remove_classes(&enter_from_class);
        el.add_classes(&enter_class);
        release_transition(el);
    }
}

fn use_keyed_elements<Item, ChildFn, Child, KeyFn, Key>(
    key_fn: KeyFn,
    children_fn: ChildFn,
    appear: bool,
    enter_class: Memo<Classes>,
    enter_from_class: Memo<Classes>,
) -> (
    StoredValue<HashMap<Key, web_sys::HtmlElement>>,
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
    let el_per_key =
        StoredValue::new(HashMap::<Key, web_sys::HtmlElement>::new());

    let key_fn = Rc::new(key_fn);

    let mounted = StoredValue::new(false);
    spawn_local(async move {
        next_tick().await;
        mounted.set_value(true);
    });

    (
        el_per_key,
        {
            let key_fn = Rc::clone(&key_fn);
            move |item| key_fn(item)
        },
        move |item| {
            let key = key_fn(&item);
            let child = children_fn(item);

            // @kw add .expect(...)

            let child_view = child.into_view();

            match child_view {
                View::Component(component) => {
                    let node_view = component.children[0].clone();

                    let el = node_view
                        .into_html_element()
                        .unwrap()
                        .dyn_ref::<web_sys::HtmlElement>()
                        .unwrap()
                        .clone();

                    update!(|el_per_key| {
                        el_per_key.insert(key, el);
                    });

                    component.into_view()
                }
                view => {
                    let el = view
                        .clone()
                        .into_html_element()
                        .unwrap()
                        .dyn_ref::<web_sys::HtmlElement>()
                        .unwrap()
                        .clone();

                    update!(|el_per_key| {
                        el_per_key.insert(key, el);
                    });

                    view
                }
            }
        },
    )
}

fn use_class_memo(class: MaybeProp<String>) -> Memo<Classes> {
    Memo::new(move |_| {
        class()
            .map(|class| {
                class.split_whitespace().map(ToOwned::to_owned).collect()
            })
            .unwrap_or_default()
    })
}

#[leptos::component]
pub fn AnimatedFor<Items, ItemIter, Item, Child, ChildFn, Key, KeyFn>(
    each: Items,
    key: KeyFn,
    children: ChildFn,
    #[prop(optional, into)] appear: Option<bool>,
    #[prop(optional, into)] move_class: MaybeProp<String>,
    #[prop(optional, into)] enter_class: MaybeProp<String>,
    #[prop(optional, into)] enter_from_class: MaybeProp<String>,
    #[prop(optional, into)] leave_class: MaybeProp<String>,
) -> impl IntoView
where
    Items: Fn() -> ItemIter + 'static,
    ItemIter: IntoIterator<Item = Item> + 'static,
    Child: IntoView + 'static,
    ChildFn: Fn(Item) -> Child + 'static,
    Key: Eq + Hash + fmt::Debug + 'static, // @kw
    KeyFn: Fn(&Item) -> Key + 'static,
    Item: 'static,
{
    let appear = appear.unwrap_or_default();

    let move_class = use_class_memo(move_class);
    let enter_class = use_class_memo(enter_class);
    let enter_from_class = use_class_memo(enter_from_class);
    let leave_class = use_class_memo(leave_class);

    let (el_per_key, key_fn, children_fn) = use_keyed_elements(
        key,
        children,
        appear,
        enter_class,
        enter_from_class,
    );

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
