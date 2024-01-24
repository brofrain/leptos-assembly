use client_components::BaseIcon;
use exports::client::{icondata::Icon, prelude::*};

// FIXME: add click transition without breaking <AnimatedFor>
#[component]
pub fn FeatureBrick(
    title: &'static str,
    icon: Icon,
    active: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                uno![
                    "flex flex-col items-center gap2 rounded w27 py2",
                    "select-none cursor-pointer ws-nowrap", if active() {
                    "bg-accent text-accent-contrast" } else { "bg-secondary/4" }
                ]
            }

            on:click=move |_| active.set(!active())
        >
            <BaseIcon icon=icon class="text-lg"/>
            <div>{title}</div>
        </div>
    }
}
