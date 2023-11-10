use leptos_router::Outlet;
use common_exports::client::prelude::*;

use crate::shared::TheFooter;

#[component]
pub fn Default() -> impl IntoView {
    view! {
        <main class=uno!["p-(x4 y10)"]>
            <Outlet/>

            <TheFooter/>
            <div class=uno!["text-(center sm)", "op50"]>"[Default Layout]"</div>
        </main>
    }
}
