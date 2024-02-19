use client_hooks::confirm;
use client_i18n::provide_i18n_context;
use client_macros::generate_test_selectors;
use client_utils::{future::next_tick, reactivity::provide_global_context};
use common::{
    prelude::*,
    vendor::{wasm_bindgen::JsCast, web_sys},
};
use dev::{mock_browser, mount};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::TheConfirms;

#[wasm_bindgen_test]
async fn can_be_confirmed() {
    mock_browser();
    provide_i18n_context();
    provide_global_context();

    let show_confirm = confirm::use_show();
    let confirmed = StoredValue::new(false);

    mount(TheConfirms);

    spawn_local(async move {
        if show_confirm(confirm::Options::default())
            .await
            .is_accepted()
        {
            confirmed.set_value(true);
        }
    });

    next_tick().await;

    let selectors = generate_test_selectors!();

    document()
        .query_selector(selectors.the_confirms.confirm)
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .click();

    next_tick().await;

    assert!(confirmed());
}

#[wasm_bindgen_test]
async fn can_be_canceled() {
    mock_browser();
    provide_i18n_context();
    provide_global_context();

    let show_confirm = confirm::use_show();
    let accepted = StoredValue::new(false);

    mount(TheConfirms);

    spawn_local(async move {
        if show_confirm(confirm::Options::default())
            .await
            .is_accepted()
        {
            accepted.set_value(true);
        }
    });

    next_tick().await;

    let selectors = generate_test_selectors!();

    document()
        .query_selector(selectors.the_confirms.cancel)
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .click();

    next_tick().await;

    assert!(!accepted());
}
