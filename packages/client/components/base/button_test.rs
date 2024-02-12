use common::prelude::*;
use dev::wasm_test::mock_browser;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::BaseButton;

#[wasm_bindgen_test]
async fn match_snapshot() {
    mock_browser();

    leptos::mount_to_body(
        || view! { <BaseButton>{"Hello, World!"}</BaseButton> },
    );

    let el = document().query_selector("button").unwrap().unwrap();

    let msg = el.text_content().unwrap();
    let html = el.outer_html();

    assert_eq!(&msg, "Hello, World!");
}
