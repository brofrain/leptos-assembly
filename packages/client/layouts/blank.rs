use leptos_router::Outlet;

use crate::app::prelude::*;

#[component]
pub fn Blank() -> impl IntoView {
    view! {
        <main class=uno!["p-(x2 y12)"]>
            <Outlet/>
        </main>
    }
}
