pub use common_macros as macros;
pub use common_macros::{flatten_mod, flatten_pub_mod};
pub use common_utils as utils;
pub use common_vendor as vendor;

#[macro_export]
macro_rules! use_macros {
    () => {
        #[macro_use]
        extern crate common;
    };
}
