#![allow(clippy::disallowed_macros)]

use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    rc::Rc,
};

use leptos::{
    document,
    leptos_dom::{tracing, Each},
    logging::log,
    spawn_local,
    update,
    with,
    IntoView,
    MaybeProp,
    Memo,
    RwSignal,
    SignalGetUntracked,
    SignalUpdateUntracked,
    SignalWithUntracked,
    StoredValue,
    View,
};
use leptos_use::{watch_debounced_with_options, WatchDebouncedOptions};
use wasm_bindgen::JsCast;
use web_sys::DomRect;

use crate::utils::{
    animation::{
        clear_cb_on_transition_end,
        force_reflow,
        set_cb_once_on_transition_end,
        AnimatedEl,
        Classes,
    },
    future::next_tick,
};

const MOVE_ATTRIBUTE: &str = "animated-for-move";
const ENTER_ATTRIBUTE: &str = "animated-for-enter";
const LEAVE_ATTRIBUTE: &str = "animated-for-leave";

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

fn build_clear_transition(
    active_transition_classes: &Classes,
) -> impl Fn(&web_sys::HtmlElement) {
    let classes_to_remove = active_transition_classes.clone();
    move |el| {
        el.remove_classes(&classes_to_remove);
        el.remove_attribute(MOVE_ATTRIBUTE).unwrap();
        el.remove_attribute(ENTER_ATTRIBUTE).unwrap();
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
    enter_from_class: Classes,
    enter_class: Classes,
) -> impl Fn(&web_sys::HtmlElement) {
    let release_transition = build_release_transition(&enter_class);

    move |el: &web_sys::HtmlElement| {
        el.remove_classes(&enter_from_class);
        el.add_classes(&enter_class);
        el.set_empty_attribute(ENTER_ATTRIBUTE);
        release_transition(el);
    }
}

fn extract_el_from_view(view: &View) -> Option<web_sys::HtmlElement> {
    match view {
        View::Component(component) => {
            let node_view = component.children.get(0)?.clone();

            let el = node_view
                .into_html_element()
                .ok()?
                .dyn_ref::<web_sys::HtmlElement>()?
                .clone();

            Some(el)
        }
        view => {
            let el = view
                .clone()
                .into_html_element()
                .ok()?
                .dyn_ref::<web_sys::HtmlElement>()?
                .clone();

            Some(el)
        }
    }
}

fn use_keyed_elements<Item, ChildFn, Child, KeyFn, Key>(
    key_fn: StoredValue<KeyFn>,
    children_fn: ChildFn,
    appear: bool,
    enter_from_class: Memo<Classes>,
    enter_class: Memo<Classes>,
) -> (
    StoredValue<HashMap<Key, web_sys::HtmlElement>>,
    impl Fn(Item) -> View + 'static,
)
where
    ChildFn: Fn(Item) -> Child + 'static,
    Child: IntoView + 'static,
    KeyFn: Fn(&Item) -> Key + 'static,
    Key: Eq + Hash + Clone + 'static + std::fmt::Debug, // @kw
    Item: 'static,
{
    let el_per_key =
        StoredValue::new(HashMap::<Key, web_sys::HtmlElement>::new());

    let entering_children_keys = RwSignal::new(Vec::<Key>::new());

    // @kw use normal timeout instead?
    // don't run multiple times when a few children are added at once
    _ = watch_debounced_with_options(
        entering_children_keys,
        move |_, _, _| {
            spawn_local(async move {
                next_tick().await;

                let start_enter = build_start_enter(
                    enter_from_class.get_untracked(),
                    enter_class.get_untracked(),
                );

                with!(|el_per_key| {
                    entering_children_keys.with_untracked(|keys| {
                        for key in keys {
                            let el = el_per_key.get(key).unwrap();
                            start_enter(el);
                        }
                    });
                });

                entering_children_keys.update_untracked(
                    |entering_children_keys| {
                        entering_children_keys.clear();
                    },
                );
            });
        },
        1.0,
        WatchDebouncedOptions::default().max_wait(Some(1.0)),
    );

    let initial_children_mounted = StoredValue::new(false);
    spawn_local(async move {
        next_tick().await;
        initial_children_mounted.set_value(true);
    });

    (el_per_key, move |item| {
        let key = with!(|key_fn| key_fn(&item));
        let child = children_fn(item);

        let view = child.into_view();

        let el = extract_el_from_view(&view);

        if let Some(el) = el {
            update!(|el_per_key| {
                el_per_key.insert(key.clone(), el.clone());
            });

            if initial_children_mounted() || appear {
                el.add_classes(&enter_from_class());
                el.enable_instant_transition();

                update!(|entering_children_keys| {
                    entering_children_keys.push(key);
                });
            }
        }

        view
    })
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

#[allow(clippy::too_many_lines)] // @kw
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
    Key: Eq + Hash + Clone + fmt::Debug + 'static, // @kw
    KeyFn: Fn(&Item) -> Key + 'static,
    Item: 'static,
{
    let appear = appear.unwrap_or_default();

    let move_class = use_class_memo(move_class);
    let enter_class = use_class_memo(enter_class);
    let enter_from_class = use_class_memo(enter_from_class);
    let leave_class = use_class_memo(leave_class);

    let key_fn = StoredValue::new(key);

    let (el_per_key, children_fn) = use_keyed_elements(
        key_fn,
        children,
        appear,
        enter_from_class,
        enter_class,
    );

    let each_fn = move || {
        let item_iter = each().into_iter();

        let mut keys = HashSet::<Key>::new();

        let mut items = Vec::new();

        with!(|key_fn| {
            for item in item_iter {
                let key = key_fn(&item);
                keys.insert(key);
                items.push(item);
            }
        });

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
                    leaving_els_parent = Some(el.parent_element().unwrap());
                }

                let rect = el.get_bounding_client_rect();
                leaving_els_with_rects.push((el, rect));
            }
        });

        spawn_local(async move {
            next_tick().await;

            if let Some(parent) = leaving_els_parent {
                let document_pos = document()
                    .document_element()
                    .expect("document to be Element")
                    .get_bounding_client_rect();

                for (el, rect) in &leaving_els_with_rects {
                    el.set_empty_attribute(LEAVE_ATTRIBUTE);
                    lock_fixed_position(el, rect, &document_pos);
                    parent.append_child(el).unwrap();
                }
            }

            let mut moved_el_keys = Vec::new();

            let clear_move_transition =
                build_clear_transition(&move_class.get_untracked());

            with!(|el_per_key| {
                for (key, old_pos) in &before_render_el_rect_per_key {
                    let el = el_per_key.get(key).unwrap();

                    clear_move_transition(el);

                    if check_if_moved_and_lock_previous_position(el, old_pos) {
                        moved_el_keys.push(key.clone());
                    }
                }
            });

            force_reflow();

            if !leaving_els_with_rects.is_empty() {
                with!(|leave_class| {
                    for (el, ..) in leaving_els_with_rects {
                        el.add_classes(leave_class);
                        el.disable_instant_transition();

                        set_cb_once_on_transition_end(&el, |el| {
                            el.remove();
                        });
                    }
                });
            }

            if moved_el_keys.is_empty() {
                return;
            }

            move_class.with_untracked(|move_class| {
                let release_move_transition =
                    build_release_transition(move_class);

                let mut release_move_and_enter_transition = None;

                with!(|el_per_key| {
                    for key in moved_el_keys {
                        let el = el_per_key.get(&key).unwrap();

                        el.add_classes(move_class);
                        el.set_empty_attribute(MOVE_ATTRIBUTE);

                        if el.has_attribute(ENTER_ATTRIBUTE) {
                            if release_move_and_enter_transition.is_none() {
                                release_move_and_enter_transition =
                                    Some(build_release_transition(
                                        &[
                                            move_class.clone(),
                                            enter_class.get_untracked(),
                                        ]
                                        .concat(),
                                    ));
                            }

                            release_move_and_enter_transition.as_ref().unwrap()(
                                el,
                            );
                        } else {
                            release_move_transition(el);
                        }
                    }
                });
            });
        });

        items
    };

    Each::new(
        each_fn,
        move |item| with!(|key_fn| key_fn(item)),
        children_fn,
    )
    .into_view()
}
