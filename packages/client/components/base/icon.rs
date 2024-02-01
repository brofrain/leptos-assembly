use common::vendor::client::{icondata::Icon as IconType, prelude::*};
use leptos_icons::Icon;

const SIZE: &str = "1.2em";

#[component]
pub fn BaseIcon(
    #[prop(into)] icon: MaybeSignal<IconType>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || uno!["inline-block", class()]>
            <Icon icon=icon class="flex-center" width=SIZE height=SIZE/>
        </div>
    }
}
