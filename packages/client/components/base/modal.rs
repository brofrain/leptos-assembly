use client_hooks::overlay;
use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::{
    prelude::*,
    vendor::{
        wasm_bindgen::JsCast,
        web_sys::{self, MouseEvent},
    },
};

#[component]
pub fn BaseModal(
    children: Children,
    #[prop(optional, into)] class: MaybeTextProp,
    #[prop(optional, into)] on_overlay_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let on_overlay_click = move |e| {
        if let Some(on_overlay_click) = on_overlay_click.as_ref() {
            on_overlay_click(e);
        }
    };

    let overlay = overlay::use_switch();
    overlay.enable();
    on_cleanup(move || overlay.disable());

    if let Some(el) = document().active_element() {
        el.dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .blur()
            .unwrap();
    }

    view! {
        <div class="cover flex justify-center items-center pb1/4" on:click=on_overlay_click>
            <div
                class=move || {
                    uno!["min-w-86 max-w-93/100", "rounded", "p4", "bg-primary", class.get_string()]
                }

                on:click=|e| e.stop_propagation()
            >
                {children()}
            </div>
        </div>
    }
}
