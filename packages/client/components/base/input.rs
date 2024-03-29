use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::prelude::*;

#[component]
pub fn BaseInput(
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeTextProp,
    #[prop(optional, into)] class: MaybeTextProp,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <input
            {..attrs}
            class=uno![
                "w72", "py1", "text-center", "bg-primary", "rounded", "border-(1 secondary/30)",
                "placeholder:(pl1 italic text-sm text-secondary/75)",
                "outline-(~ 2 offset-0 transparent)",
                "hover:outline-accent !focus:outline-accent-focus", "transition-all", class
                .get_string()
            ]

            type="text"
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| {
                value.set(event_target_value(&ev));
            }
        />
    }
}
