use common::utils::id;
use leptos::{document, IntoView};
use wasm_bindgen::JsCast;

// FIXME: This function allows `dev::mock_browser` to clean up component
// instances from other tests. However, global Leptos runtime is still preserved
// across tests.
pub fn mount<V, F>(f: F)
where
    F: FnOnce() -> V + 'static,
    V: IntoView,
{
    let document = document();
    let target = document.create_element("div").unwrap();
    target.set_id(&id::usize().to_string());
    document.body().unwrap().append_child(&target).unwrap();
    leptos::mount_to(target.dyn_into().unwrap(), f);
}
