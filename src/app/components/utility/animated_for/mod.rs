#![allow(clippy::disallowed_macros)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use leptos::{
    document,
    leptos_dom::{tracing, Each},
    spawn_local,
    update,
    with,
    IntoView,
    MaybeProp,
    StoredValue,
    View,
};
use web_sys::DomRect;

use self::animator::Animator;
use crate::utils::{
    animation::{extract_el_from_view, force_reflow, AnimatedEl},
    future::next_tick,
};

mod animator;
mod untracked_classes;

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

fn prepare_leave(
    leaving_els_parent: &web_sys::Element,
    leaving_els_with_rects: &Vec<(web_sys::HtmlElement, DomRect)>,
) {
    let document_pos = document()
        .document_element()
        .expect("document to be Element")
        .get_bounding_client_rect();

    for (el, rect) in leaving_els_with_rects {
        lock_fixed_position(el, rect, &document_pos);
        leaving_els_parent.append_child(el).unwrap();
    }
}

fn check_if_moved_and_lock_previous_position(
    el: &web_sys::HtmlElement,
    old_pos: &DomRect,
) -> bool {
    let new_pos = el.get_bounding_client_rect();

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

#[allow(clippy::too_many_arguments)]
fn use_entering_children<Item, ChildFn, Child, KeyFn, Key>(
    key_fn: StoredValue<KeyFn>,
    children_fn: ChildFn,
    appear: Option<bool>,
    move_class: MaybeProp<String>,
    enter_class: MaybeProp<String>,
    enter_from_class: MaybeProp<String>,
    leave_class: MaybeProp<String>,
) -> (
    StoredValue<HashMap<Key, web_sys::HtmlElement>>,
    Animator<Key>,
    impl Fn(Item) -> View + 'static,
)
where
    ChildFn: Fn(Item) -> Child + 'static,
    Child: IntoView + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + Clone + 'static,
    Item: 'static,
{
    let appear = appear.unwrap_or_default();

    let el_per_key = StoredValue::new(HashMap::new());

    let animator =
        Animator::new(enter_from_class, enter_class, move_class, leave_class);

    let initial_children_mounted = StoredValue::new(false);
    spawn_local(async move {
        initial_children_mounted.set_value(true);
    });

    (el_per_key, animator, move |item| {
        let key = with!(|key_fn| key_fn(&item));
        let child = children_fn(item);

        let view = child.into_view();

        let el = extract_el_from_view(&view);

        if let Some(el) = el {
            update!(|el_per_key| {
                el_per_key.insert(key.clone(), el.clone());
            });

            if initial_children_mounted() || appear {
                animator.prepare_enter(&key, &el);

                spawn_local(async move {
                    next_tick().await;
                    animator.start_enter(&key, &el);
                });
            }
        }

        view
    })
}

#[leptos::component(transparent)]
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
    Key: Eq + Hash + Clone + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Item: 'static,
{
    let key_fn = StoredValue::new(key);

    let (el_per_key, animator, children_fn) = use_entering_children(
        key_fn,
        children,
        appear,
        move_class,
        enter_class,
        enter_from_class,
        leave_class,
    );

    let items_fn = move || {
        let items = Vec::from_iter(each());

        let keys =
            with!(|key_fn| items.iter().map(key_fn).collect::<HashSet<_>>());

        let mut leaving_els_parent = None;
        let mut leaving_els_with_rects = Vec::new();

        let mut before_render_el_rect_per_key = HashMap::<Key, DomRect>::new();

        update!(|el_per_key| {
            let mut keys_to_remove = Vec::new();

            for (key, el) in el_per_key.iter() {
                if keys.contains(key) {
                    let rect = el.get_bounding_client_rect();
                    before_render_el_rect_per_key.insert(key.clone(), rect);
                } else {
                    keys_to_remove.push(key.clone());
                }
            }

            for key in keys_to_remove {
                let el = el_per_key.remove(&key).unwrap();

                if leaving_els_parent.is_none() {
                    leaving_els_parent = Some(
                        el.parent_element()
                            .expect("children to have parent element"),
                    );
                }

                let rect = el.get_bounding_client_rect();
                leaving_els_with_rects.push((el, rect));
            }
        });

        spawn_local(async move {
            animator.clear_transitions();

            if let Some(parent) = leaving_els_parent {
                prepare_leave(&parent, &leaving_els_with_rects);
            }

            let mut moved_el_keys = Vec::new();

            with!(|el_per_key| {
                for (key, old_pos) in &before_render_el_rect_per_key {
                    let el = el_per_key.get(key).unwrap();

                    if check_if_moved_and_lock_previous_position(el, old_pos) {
                        moved_el_keys.push(key.clone());
                    }
                }
            });

            force_reflow();

            if !leaving_els_with_rects.is_empty() {
                for (el, ..) in leaving_els_with_rects {
                    animator.start_leave(&el);
                }
            }

            if moved_el_keys.is_empty() {
                return;
            }

            with!(|el_per_key| {
                for key in moved_el_keys {
                    let el = el_per_key.get(&key).unwrap();
                    animator.start_move(el);
                }
            });
        });

        items
    };

    Each::new(
        items_fn,
        move |item| with!(|key_fn| key_fn(item)),
        children_fn,
    )
}