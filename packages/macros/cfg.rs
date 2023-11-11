#![allow(clippy::module_name_repetitions)]

/// Contents will be compiled only for client-side of the application -
/// shorthand for `cfg_if::cfg_if! { if #[cfg(feature = "csr")] { ... } }`.
///
/// # Example
///
/// ```
/// macros::cfg_csr! {
///     leptos::logging::log!("Hello from browser!");
/// }
/// ```
#[macro_export]
macro_rules! cfg_csr {
    ($($t:tt)*) => {
        $crate::__exports::cfg_if::cfg_if! { if #[cfg(feature = "csr")] {
            $($t)*
        }}
    };
}

/// Contents will be compiled only for client-side of the application -
/// shorthand for `cfg_if::cfg_if! { if #[cfg(feature = "ssr")] { ... } }`.
///
/// # Example
///
/// ```
/// macros::cfg_ssr! {
///     leptos::logging::log!("Hello from server!");
/// }
/// ```
#[macro_export]
macro_rules! cfg_ssr {
    ($($t:tt)*) => {
        $crate::__exports::cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
            $($t)*
        }}
    };
}

/// Shorthand for `cfg!(feature = "csr")`.
///
/// # Example
///
/// ```
/// assert_eq!(macros::is_csr!(), cfg!(feature = "csr"));
/// ```
#[macro_export]
macro_rules! is_csr {
    () => {
        cfg!(feature = "csr")
    };
}

/// Shorthand for `cfg!(feature = "ssr")`.
///
/// # Example
///
/// ```
/// assert_eq!(macros::is_ssr!(), cfg!(feature = "ssr"));
/// ```
#[macro_export]
macro_rules! is_ssr {
    () => {
        cfg!(feature = "ssr")
    };
}
