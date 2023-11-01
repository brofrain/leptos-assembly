use leptos_router::{use_params, IntoParam, NavigateOptions, Params};

use crate::app::{
    components::base::Button,
    prelude::*,
    router::{use_navigate, Route},
};

#[derive(Params, PartialEq)]
pub struct NotFoundParams {
    pub path: String,
}

#[component]
pub fn NotFound() -> impl IntoView {
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
                <Button on:click=go_home>{t!(i18n, common.button.home)}</Button>
            </div>
        </div>
    }
}
