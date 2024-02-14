use client_hooks::confirm;
use client_i18n::provide_i18n_context;
use client_utils::{future::next_tick, reactivity::provide_global_context};
use common::{
    prelude::*,
    vendor::{wasm_bindgen::JsCast, web_sys},
};
use dev::{mock_browser, mount};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::TheConfirms;

#[wasm_bindgen_test]
async fn can_be_accepted() {
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

    document()
        .query_selector("[test='16321067049399601648']")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .click();

    next_tick().await;

    assert!(accepted());
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

    document()
        .query_selector("[test='8190099354128144357']")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap()
        .click();

    next_tick().await;

    assert!(!accepted());
}
