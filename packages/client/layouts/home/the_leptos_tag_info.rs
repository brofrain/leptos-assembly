use client_hooks::nprogress;
use client_i18n::use_i18n;
use common::vendor::client::prelude::*;
use server::get_leptos_tag;

#[component]
pub fn TheLeptosTagInfo() -> impl IntoView {
    let i18n = use_i18n();
    let nprogress = nprogress::use_switch();

    let tag = Resource::local(
        || (),
        move |()| async move {
            nprogress.enable();
            let result = get_leptos_tag().await;
            nprogress.disable();
            result
        },
    );

    let msg = move || {
        tag.with(|tag| {
            tag.as_ref().map(|tag| {
                tag.as_ref().map_or_else(
                    move |_| t_string!(i18n, home.leptos_tag_error),
                    |tag| t_string!(i18n, home.leptos_tag, tag),
                )
            })
        })
    };

    view! {
        <div class=uno!["text-(sm center) italic", "op70", "mb2"]>
            <Suspense fallback=t!(i18n, home.leptos_tag_loading)>{msg}</Suspense>
        </div>
    }
}
