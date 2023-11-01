#![allow(clippy::module_name_repetitions)] // @kw still needed?

pub use common_proc_macros::*;

#[macro_use]
pub mod cfg;

#[macro_use]
pub mod js;

#[macro_use]
pub mod i18n;

/// Shorthand for `mod module; use module::*;` with hushed
/// `clippy::module_name_repetitions` lint.
#[macro_export]
macro_rules! flatten_mod {
    ($($module:ident),*) => {
        $(
            #[allow(clippy::module_name_repetitions)]
            mod $module;
            use $module::*;
        )*
    };
}

/// Shorthand for `mod module; pub use module::*;` with hushed
/// `clippy::module_name_repetitions` lint.
#[macro_export]
macro_rules! flatten_pub_mod {
    ($($module:ident),*) => {
        $(
            #[allow(clippy::module_name_repetitions)]
            mod $module;
            pub use $module::*;
        )*
    };
}

pub mod exports {
    pub use cfg_if;
}