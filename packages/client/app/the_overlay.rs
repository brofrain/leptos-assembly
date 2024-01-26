use client_hooks::overlay;
use exports::client::prelude::*;

#[component]
pub fn TheOverlay() -> impl IntoView {
    let show = overlay::use_show();

    view! {
        <Show when=show>
            <div class=uno!["cover", "bg-#000/50"]></div>
        </Show>
    }
}
