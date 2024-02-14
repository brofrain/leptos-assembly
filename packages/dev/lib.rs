pub use insta::assert_yaml_snapshot as assert_snapshot;

mod wasm_test {
    pub mod mock_browser;
    pub mod mount;
}
pub use wasm_test::{mock_browser::*, mount::*};
