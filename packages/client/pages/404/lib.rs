use client_components::BaseButton;
use client_i18n::use_i18n;
use client_router::{use_navigate, NotFoundParams, Route};
use common_exports::client::prelude::*;
use leptos_router::{use_params, NavigateOptions};

#[component]
pub fn Index() -> impl IntoView {
    let params = use_params::<NotFoundParams>();
    let i18n = use_i18n();

    let path = move || with!(|params| params.as_ref().unwrap().path.clone());

    let navigate = use_navigate();

    let go_home = move |_| {
        navigate(&Route::Home, NavigateOptions::default());
    };

    view! {
        <div class=uno!["text-center"]>
            <div class=uno!["inline-block text-4xl mb1 icon-carbon-warning"]></div>

            <div class=uno!["text-lg"]>
                <span class=uno!["op75"]>{t!(i18n, page_not_found)}</span>
                " "
                {path}
            </div>

            <div class=uno!["mt5"]>
                <BaseButton on:click=go_home>{t!(i18n, common.button.home)}</BaseButton>
            </div>
        </div>
    }
}
