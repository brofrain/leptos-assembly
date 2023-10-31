use crate::app::{composables::overlay, prelude::*};

#[component]
pub fn TheOverlay() -> impl IntoView {
    let show = overlay::use_show();

    view! {
        <Show when=show fallback=|| {}>
            <div class=uno!["cover", "bg-#000/50"]></div>
        </Show>
    }
}
