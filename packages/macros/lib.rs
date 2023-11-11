pub use proc_macros::*;

#[macro_use]
pub mod cfg;

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

pub mod __exports {
    pub use cfg_if;
}
