use common::{prelude::*, vendor::leptos_router::Outlet};

use crate::shared::Footer;

flatten_mod!(leptos_tag_info);

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class=uno!["p-(x6 y10)"]>
            <Outlet/>

            <Footer/>
            <LeptosTagInfo/>
            <div class=uno!["text-(center sm)", "op50"]>"[Home Layout]"</div>
        </main>
    }
}
