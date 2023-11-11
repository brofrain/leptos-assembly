use exports::client::prelude::*;
use leptos_router::Outlet;

#[component]
pub fn Blank() -> impl IntoView {
    view! {
        <main class=uno!["p-(x2 y12)"]>
            <Outlet/>
        </main>
    }
}
