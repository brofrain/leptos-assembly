use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::vendor::client::{icondata as i, prelude::*};

use crate::BaseIcon;

#[component]
pub fn TheLogo(#[prop(optional, into)] class: MaybeTextProp) -> impl IntoView {
    view! {
        <BaseIcon
            icon=i::SiLeptos
            class=TextProp::from(move || {
                uno![
                    "text-secondary hover:text-accent", "transition-colors", "cursor-pointer", class
                    .get_string()
                ]
            })

            attr:test="logo"
        />
    }
}
