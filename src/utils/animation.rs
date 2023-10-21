use leptos::{window, View};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;

// @kw make private?
pub type Classes = Vec<String>;

pub trait AnimatedEl {
    /// Returns the classes that were not previously present, and were added by
    /// the method.
    fn add_classes(&self, classes: &Classes) -> Classes;
    fn remove_classes(&self, classes: &Classes);
    fn set_important_style_property(&self, property: &str, value: &str);
    fn enable_instant_transition(&self);
    fn disable_instant_transition(&self);
    fn set_empty_attribute(&self, attr: &str);
}

impl AnimatedEl for HtmlElement {
    fn add_classes(&self, classes: &Classes) -> Classes {
        let mut added_classes = Vec::new();

        for class in classes {
            let class_list = self.class_list();

            if class_list.contains(class) {
                continue;
            }

            class_list.add_1(class).unwrap();
            added_classes.push(class.clone());
        }

        added_classes
    }

    fn remove_classes(&self, classes: &Classes) {
        for class in classes {
            self.class_list().remove_1(class).unwrap();
        }
    }

    fn set_important_style_property(&self, property: &str, value: &str) {
        self.style()
            .set_property_with_priority(property, value, "important")
            .unwrap();
    }

    fn enable_instant_transition(&self) {
        self.set_important_style_property("transition-duration", "0s");
    }

    fn disable_instant_transition(&self) {
        self.style()
            .set_property("transition-duration", "")
            .unwrap();
    }

    fn set_empty_attribute(&self, attr: &str) {
        self.set_attribute(attr, "").unwrap();
    }
}

pub fn clear_cb_on_transition_end(el: &HtmlElement) {
    el.set_ontransitionend(None);
    el.set_onanimationend(None);
}

pub fn set_cb_once_on_transition_end<F>(el: &HtmlElement, mut cb: F)
where
    F: FnMut(&HtmlElement) + 'static,
{
    let original_el = el.clone();

    // @kw use leptos API?
    let closure = Closure::<dyn FnMut(&web_sys::TransitionEvent)>::wrap(
        Box::new(move |event| {
            let el = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();

            if original_el != el {
                return;
            }

            cb(&el);
            el.set_ontransitionend(None);
            el.set_onanimationend(None);
        }),
    );

    el.set_ontransitionend(Some(closure.as_ref().unchecked_ref()));
    el.set_onanimationend(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

pub fn force_reflow() {
    _ = window().document().unwrap().body().unwrap().offset_height();
}

pub fn extract_el_from_view(view: &View) -> Option<web_sys::HtmlElement> {
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
