use client_macros::inject_test_selector;
use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::prelude::*;

use crate::BaseIcon;

#[component]
pub fn TheLogo(#[prop(optional, into)] class: MaybeTextProp) -> impl IntoView {
    view! {
        <BaseIcon
            icon=icon::SiLeptos
            class=TextProp::from(move || {
                uno![
                    "text-secondary hover:text-accent", "transition-colors", "cursor-pointer", class
                    .get_string()
                ]
            })

            attr:test=inject_test_selector!()
        />
    }
}
