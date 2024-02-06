use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::prelude::*;
use leptos_icons::Icon;

const SIZE: &str = "1.2em";

#[component]
pub fn BaseIcon(
    #[prop(into)] icon: MaybeSignal<icon::Icon>,
    #[prop(optional, into)] class: MaybeTextProp,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <div {..attrs} class=move || uno!["inline-block", class.get_string()]>
            <Icon icon=icon class="flex-center" width=SIZE height=SIZE/>
        </div>
    }
}
