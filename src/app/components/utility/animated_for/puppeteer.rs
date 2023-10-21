use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use leptos::{MaybeProp, StoredValue};
use wasm_bindgen::{prelude::Closure, JsCast};

use super::untracked_classes::UntrackedClasses;
use crate::utils::animation::AnimatedEl;

#[derive(Clone, Copy)]
pub struct Puppeteer<Key>
where
    Key: 'static,
{
    classes: UntrackedClasses,
    transition_end_cbs: StoredValue<Vec<Rc<dyn FnOnce()>>>,
    enter_from_class_per_key: StoredValue<HashMap<Key, Vec<String>>>,
}

impl<Key> Puppeteer<Key>
where
    Key: Clone + Hash + Eq + PartialEq,
{
    pub fn new(
        raw_enter_from: MaybeProp<String>,
        raw_enter: MaybeProp<String>,
        raw_move: MaybeProp<String>,
        raw_leave: MaybeProp<String>,
    ) -> Self {
        Self {
            classes: UntrackedClasses::new(
                raw_enter_from,
                raw_enter,
                raw_move,
                raw_leave,
            ),
            transition_end_cbs: StoredValue::new(Vec::new()),
            enter_from_class_per_key: StoredValue::new(HashMap::new()),
        }
    }

    pub fn prepare_enter(&self, key: &Key, el: &web_sys::HtmlElement) {
        el.enable_instant_transition();
        let added_classes = self.classes.add_enter_from(el);
        self.enter_from_class_per_key.update_value(|v| {
            v.insert(key.clone(), added_classes);
        });
    }

    fn push_class_cleanup_on_transition_end(
        &self,
        el: &web_sys::HtmlElement,
        classes_to_remove: Vec<String>,
    ) {
        let el = Rc::new(el.clone());

        let clear_classes = Rc::new({
            let el = Rc::clone(&el);
            move || {
                el.remove_classes(&classes_to_remove);
                el.set_ontransitionend(None);
                el.set_onanimationend(None);
            }
        });

        // @kw use leptos API?
        let closure =
            Closure::<dyn FnMut(&web_sys::TransitionEvent)>::wrap(Box::new({
                let clear_classes = Rc::clone(&clear_classes);
                let el = Rc::clone(&el);
                move |event| {
                    let event_el = event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlElement>()
                        .unwrap();

                    // @kw still needed?
                    if *el != event_el {
                        return;
                    }

                    clear_classes();
                }
            }));

        el.set_ontransitionend(Some(closure.as_ref().unchecked_ref()));
        el.set_onanimationend(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        self.transition_end_cbs.update_value(|v| {
            v.push(clear_classes);
        });
    }

    pub fn start_enter(&self, key: &Key, el: &web_sys::HtmlElement) {
        self.enter_from_class_per_key.update_value(|v| {
            let classes_to_remove = v.remove(key);

            if let Some(classes_to_remove) = classes_to_remove {
                el.remove_classes(&classes_to_remove);
            }
        });

        el.disable_instant_transition();

        let added_classes = self.classes.add_enter(el);
        self.push_class_cleanup_on_transition_end(el, added_classes);
    }
}
