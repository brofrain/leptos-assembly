use client_hooks::overlay;
use common::prelude::*;

#[component]
pub fn Overlay() -> impl IntoView {
    let show = overlay::use_show();

    view! {
        <Show when=show>
            <div class=uno!["cover", "bg-#000/50"]></div>
        </Show>
    }
}
