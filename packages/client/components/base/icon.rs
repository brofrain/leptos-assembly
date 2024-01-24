use exports::client::{
    icondata::Icon as IconType,
    leptos::TextProp,
    prelude::*,
};
use leptos_icons::Icon;

const SIZE: &str = "1.2em";

#[component]
pub fn BaseIcon(
    #[prop(into)] icon: MaybeSignal<IconType>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let class = TextProp::from(move || uno!["flex-center", class()]);
    view! {
        <div>
            <Icon icon=icon class=class width=SIZE height=SIZE/>
        </div>
    }
}
