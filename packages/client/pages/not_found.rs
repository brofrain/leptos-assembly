use client_components::{BaseButton, BaseIcon};
use client_i18n::use_i18n;
use client_router::{use_navigate, NotFoundParams, Route};
use common::{prelude::*, vendor::leptos_router::use_params};

#[component]
pub fn NotFound() -> impl IntoView {
    let params = use_params::<NotFoundParams>();
    let i18n = use_i18n();

    let path =
        move || with!(|params| params.as_ref().map(|params| params.path().clone()).ok());

    let navigate = use_navigate();

    let go_home = move |_| {
        navigate(Route::Home);
    };

    view! {
        <div class=uno!["text-center"]>
            <BaseIcon icon=icon::BsExclamationCircle class=uno!["text-4xl mb3"]/>

            <div class=uno!["text-lg"]>
                <span class=uno!["op75"]>{t!(i18n, page_not_found)}</span>
                " /"
                {path}
            </div>

            <div class=uno!["mt5"]>
                <BaseButton on:click=go_home>{t!(i18n, common.button.home)}</BaseButton>
            </div>
        </div>
    }
}
