use common::prelude::*;
use dev::wasm_test::mock_browser;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::BaseButton;

#[wasm_bindgen_test]
async fn render_children() {
    mock_browser();

    leptos::mount_to_body(
        || view! { <BaseButton>{"Hello, World!"}</BaseButton> },
    );

    let el = document().query_selector("button").unwrap().unwrap();

    let msg = el.text_content().unwrap();
    assert_eq!(&msg, "Hello, World!");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn match_snapshot() {
    use dev::vendor::insta::assert_yaml_snapshot;
    use leptos::ssr::render_to_string;

    let html = render_to_string(|| {
        view! { <BaseButton>{"Hello, World!"}</BaseButton> }.into_view()
    });

    assert_yaml_snapshot!(html);
}
