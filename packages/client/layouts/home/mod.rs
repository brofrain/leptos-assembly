use leptos_router::Outlet;

use crate::app::{components::layouts::shared::TheFooter, prelude::*};

flatten_mod!(the_leptos_tag_info);

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class=uno!["p-(x6 y10)"]>
            <Outlet/>

            <TheFooter/>
            <TheLeptosTagInfo/>
            <div class=uno!["text-(center sm)", "op50"]>"[Home Layout]"</div>
        </main>
    }
}
