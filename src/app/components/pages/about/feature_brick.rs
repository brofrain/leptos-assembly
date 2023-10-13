use leptos::html::Div;

use crate::app::prelude::*;

#[component]
pub fn FeatureBrick(
    title: &'static str,
    icon_class: &'static str,
    active: RwSignal<bool>,

    // FIXME
    // the param is detected as unused, but it's actually used in the `view!`
    #[allow(unused_variables)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                uno![
                    "flex flex-col items-center gap2 rounded w27 py2",
                    "select-none cursor-pointer ws-nowrap transition", if active() {
                    "bg-accent text-accent-contrast" } else { "bg-secondary/4" }
                ]
            }

            on:click=move |_| active.set(!active())
        >
            <div class=uno![icon_class, "text-lg"]></div>
            <div>{title}</div>
        </div>
    }
}
