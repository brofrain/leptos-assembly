// FIXME: There is nothing here currently, but it's preferred not to
// distinguish declarative macros from procedural ones in the target codebase.
#[allow(unused_imports)]
pub use proc_macros::*;

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
