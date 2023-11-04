use client_composables::nprogress;
use client_globals::prelude::*;
use client_i18n::{t_view, use_i18n};
use leptos_i18n::t;
use server::get_leptos_tag;

#[component]
pub fn TheLeptosTagInfo() -> impl IntoView {
    let i18n = use_i18n();

    let tag = Resource::local(
        || (),
        |()| async {
            nprogress::enable();
            let result = get_leptos_tag().await;
            nprogress::disable();
            result
        },
    );

    let msg = Memo::new(move |_| {
        tag().map_or_else(
            move || t_view!(i18n, home.leptos_tag_error),
            |tag| t_view!(i18n, home.leptos_tag, tag),
        )
    });

    view! {
        <div class=uno!["text-(sm center) italic", "op70", "mb2"]>
            <Suspense fallback=t!(i18n, home.leptos_tag_loading)>{msg}</Suspense>
        </div>
    }
}
