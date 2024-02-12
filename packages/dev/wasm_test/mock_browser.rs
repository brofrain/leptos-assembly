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
}
