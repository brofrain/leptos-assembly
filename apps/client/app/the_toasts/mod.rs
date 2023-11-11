use client_components::BaseAnimatedFor;
use client_composables::toast::use_queue;
use exports::client::prelude::*;

flatten_mod!(toast);

#[component]
pub fn TheToasts() -> impl IntoView {
    let toast_queue = use_queue();

    view! {
        <div class="cover pointer-events-none flex flex-col-reverse items-center gap2 pb8">
            <BaseAnimatedFor
                each=toast_queue
                key=|toast| *toast.id()
                children=|toast| {
                    view! { <Toast severity=*toast.severity()>{*toast.body()}</Toast> }
                }
            />

        </div>
    }
}
