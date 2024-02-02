leptos_i18n::load_locales!();
pub use i18n::{provide_i18n_context, use_i18n, Locale};

pub mod __exports {
    pub use paste;
}

/// Shorthand for `td!(i18n.get_locale_untracked(), ...)`.
#[macro_export]
macro_rules! t_untracked {
    ($i18n:ident, $($t:tt)*) => {
        leptos_i18n::td!($i18n.get_locale_untracked(), $($t)*)
    }
}

/// Shorthand for `td_string!(i18n.get_locale_untracked(), ...)`.
#[macro_export]
macro_rules! t_string_untracked {
    ($i18n:ident, $($t:tt)*) => {
        leptos_i18n::td_string!($i18n.get_locale_untracked(), $($t)*)
    };
}
