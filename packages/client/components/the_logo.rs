use common::vendor::client::{icondata as i, prelude::*};

use crate::BaseIcon;

#[component]
pub fn TheLogo(
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <BaseIcon
            icon=i::SiLeptos
            class=Signal::derive(move || {
                uno![
                    "text-secondary hover:text-accent", "transition-colors", "cursor-pointer",
                    class()
                ]
            })

            attr:test="logo"
        />
    }
}
