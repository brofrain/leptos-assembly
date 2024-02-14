use common::prelude::*;
use dev::{mock_browser, mount};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::BaseButton;

#[wasm_bindgen_test]
fn render_children() {
    mock_browser();

    mount(|| view! { <BaseButton>{"Hello, World!"}</BaseButton> });

    let el = document().query_selector("button").unwrap().unwrap();

    let msg = el.text_content().unwrap();
    assert_eq!(&msg, "Hello, World!");
}

#[wasm_bindgen_test]
async fn have_reactive_disabled_attribute() {
    mock_browser();

    let disabled = RwSignal::new(false);

    mount(
        move || view! { <BaseButton disabled=disabled>{"Hello, World!"}</BaseButton> },
    );

    let el = document().query_selector("button").unwrap().unwrap();
    let is_disabled = || el.has_attribute("disabled");

    assert!(!is_disabled());

    disabled.set(true);
    assert!(is_disabled());

    disabled.set(false);
    assert!(!is_disabled());
}

#[cfg(not(target_arch = "wasm32"))]
mod server {
    use common::prelude::*;
    use dev::assert_snapshot;
    use leptos::ssr::render_to_string;

    use crate::BaseButton;

    #[test]
    fn match_snapshot() {
        let html = render_to_string(|| {
            view! { <BaseButton>{"Hello, World!"}</BaseButton> }.into_view()
        });

        assert_snapshot!(html);
    }
}
