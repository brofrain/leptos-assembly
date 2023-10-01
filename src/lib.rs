#[macro_use]
mod macros;

mod env;
mod prelude;

mod app;
mod utils;

cfg_client! {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        use app::App;
        use utils::logger;

        logger::init();

        leptos::mount_to_body(
            || leptos::view! { <App/> },
        );
    }
}

cfg_server! {
    pub use crate::{app::App, utils::logger};
}
