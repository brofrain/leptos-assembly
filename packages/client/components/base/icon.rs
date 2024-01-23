use exports::client::{
    icondata::Icon as IconType,
    leptos::TextProp,
    prelude::*,
};
use leptos_icons::Icon;

#[component]
pub fn BaseIcon(
    #[prop(into)] icon: MaybeSignal<IconType>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let class = TextProp::from(move || uno!["text-2xl", class()]);
    view! { <Icon icon=icon class=class/> }
}
