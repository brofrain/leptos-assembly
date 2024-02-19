use std::panic;

use common::vendor::log::{self, Level};

#[allow(unsafe_code)]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/wasm_test/mock_browser.cjs")]
    extern "C" {
        pub fn mock_browser();
    }
}

pub fn mock_browser() {
    js::mock_browser();
    _ = console_log::init_with_level(Level::Debug);
    panic::set_hook(Box::new(move |panic_info| {
        let msg = panic_info.to_string();
        log::error!("{msg}");
    }));
}
