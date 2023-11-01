use crate::app::prelude::*;

#[component]
pub fn BaseButton(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <button
            {..attrs}
            class=move || {
                uno![
                    "inline-block", "rounded", "p-(x4 y.5)",
                    "fw-bold text-accent-interactive-contrast", "bg-accent-interactive",
                    "transition-colors", "cursor-pointer", "select-none",
                    "disabled:pointer-events-none", class()
                ]
            }

            disabled=disabled
        >
            {children()}
        </button>
    }
}
