use common::{prelude::*, vendor::leptos_router::Outlet};

use crate::shared::Footer;

#[component]
pub fn Default() -> impl IntoView {
    view! {
        <main class=uno!["p-(x4 y10)"]>
            <Outlet/>

            <Footer/>
            <div class=uno!["text-(center sm)", "op50"]>"[Default Layout]"</div>
        </main>
    }
}
