use client_composables::overlay;
use client_globals::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

#[component]
pub fn BaseModal(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] on_overlay_click: Option<Callback<MouseEvent>>,
) -> impl IntoView {
    let on_overlay_click = move |e| {
        if let Some(on_overlay_click) = on_overlay_click.as_ref() {
            on_overlay_click(e);
        }
    };

    overlay::enable();
    on_cleanup(overlay::disable);

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
                    uno!["min-w-86 max-w-93/100", "rounded", "p4", "bg-primary", class()]
                }

                on:click=|e| e.stop_propagation()
            >
                {children()}
            </div>
        </div>
    }
}
