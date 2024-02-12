pub mod vendor {
    pub use insta;
}

pub mod wasm_test {
    mod mock_browser;
    pub use mock_browser::*;
}
