use client_i18n::use_i18n;
use client_stores::{use_store, Names};
use common::prelude::*;

#[component]
pub fn Hi() -> impl IntoView {
    let i18n = use_i18n();

    let names_store = use_store::<Names>();

    // TODO: add something more interesting here

    view! {
        <div class="text-center">
            <p>{t!(i18n, name.welcome, name = move || names_store.last_name())}</p>
        </div>
    }
}
