use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::vendor::client::prelude::*;

#[component]
pub fn BaseButton(
    children: Children,
    #[prop(optional, into)] class: MaybeTextProp,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <button
            {..attrs}
            class=move || {
                uno![
                    "inline-block", "rounded", "p-(x4 t1 b.5)",
                    "fw-bold text-accent-interactive-contrast", "bg-accent-interactive",
                    "transition-colors", "cursor-pointer", "select-none",
                    "disabled:pointer-events-none", class.get_string()
                ]
            }

            disabled=disabled
        >
            {children()}
        </button>
    }
}
