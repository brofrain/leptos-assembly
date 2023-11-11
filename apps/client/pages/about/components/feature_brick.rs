use exports::client::prelude::*;

// FIXME: add click transition without breaking <AnimatedFor>
#[component]
pub fn FeatureBrick(
    title: &'static str,
    icon_class: &'static str,
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
            <div class=uno![icon_class, "text-lg"]></div>
            <div>{title}</div>
        </div>
    }
}
