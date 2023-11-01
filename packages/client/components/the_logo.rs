use crate::app::prelude::*;

#[component]
pub fn TheLogo(
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div class=move || {
            uno![
                "icon-simple-icons-leptos", "text-secondary hover:text-accent", "transition-colors",
                "cursor-pointer", class()
            ]
        }></div>
    }
}
