use client_hooks::nprogress;
use client_i18n::use_i18n;
use client_macros::pin_test_selector;
use common::prelude::*;
use server::leptos_tag;

#[component]
pub fn LeptosTagInfo() -> impl IntoView {
    let i18n = use_i18n();
    let nprogress = nprogress::use_switch();

    let tag = Resource::local(
        || (),
        move |()| async move {
            nprogress.enable();
            let result = leptos_tag::get().await;
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
        <div class=uno!["text-(sm center) italic", "op70", "mb2"] test=pin_test_selector!()>
            <Suspense fallback=t!(i18n, home.leptos_tag_loading)>{msg}</Suspense>
        </div>
    }
}
