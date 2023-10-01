/// Contents will be compiled only for client-side of the application -
/// shorthand for `cfg_if::cfg_if! { if #[cfg(feature = "client")] { ... } }`.
///
/// # Example
///
/// ```
/// leptos_assembly::cfg_client! {
///     leptos::logging::log!("Hello from browser!");
/// }
/// ```
#[macro_export]
macro_rules! cfg_client {
    ($($t:tt)*) => {
        cfg_if::cfg_if! { if #[cfg(feature = "client")] {
            $($t)*
        }}
    };
}

/// Contents will be compiled only for client-side of the application -
/// shorthand for `cfg_if::cfg_if! { if #[cfg(feature = "server")] { ... } }`.
///
/// # Example
///
/// ```
/// leptos_assembly::cfg_server! {
///     leptos::logging::log!("Hello from server!");
/// }
/// ```
#[macro_export]
macro_rules! cfg_server {
    ($($t:tt)*) => {
        cfg_if::cfg_if! { if #[cfg(feature = "server")] {
            $($t)*
        }}
    };
}

/// Shorthand for `cfg!(feature = "client")`.
///
/// # Example
///
/// ```
/// assert_eq!(leptos_assembly::is_client!(), cfg!(feature = "client"));
/// ```
#[macro_export]
macro_rules! is_client {
    () => {
        cfg!(feature = "client")
    };
}

/// Shorthand for `cfg!(feature = "server")`.
///
/// # Example
///
/// ```
/// assert_eq!(leptos_assembly::is_server!(), cfg!(feature = "server"));
/// ```
#[macro_export]
macro_rules! is_server {
    () => {
        cfg!(feature = "server")
    };
}
