#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(lazy_cell)]

#[macro_use]
extern crate common_macros;

mod env;
mod prelude;

mod app;
mod utils;

cfg_csr! {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        use app::App;
        use utils::logger;

        logger::init();

        leptos::mount_to_body(App);
    }
}

cfg_ssr! {
    pub use crate::{app::App, utils::logger};
}
