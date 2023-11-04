// @kw common #[macro_use] leptos_i18n ?

leptos_i18n::load_locales!();
pub use i18n::{provide_i18n_context, use_i18n, Locale};

pub mod __exports {
    pub use paste;
}

#[macro_export]
macro_rules! __t_get {
    ($macro:ident; $i18n_method:ident; $i18n:ident, $($t:tt)*) => {
        $crate::__exports::paste::item! {
            leptos_i18n::$macro!($i18n.[< $i18n_method >](), $($t)*)
        }
    };
}

/// Shorthand for `i18n::get_keys().*`. Unlike [`t!`] does not return a signal.
#[macro_export]
macro_rules! t_get {
    ($i18n:ident, $($t:tt)*) => {
        $crate::__t_get!(td; get_locale; $i18n, $($t)*)
    }
}

/// Shorthand for `i18n::get_keys_untracked().*`.
#[macro_export]
macro_rules! t_get_untracked {
    ($i18n:ident, $($t:tt)*) => {
        $crate::__t_get!(td; get_locale_untracked; $i18n, $($t)*)
    }
}

/// Returns translation tranformed into [`leptos::View`]. Shorthand for
/// `i18n::get_keys().*.into_view()`.
#[macro_export]
macro_rules! t_view {
    ($($t:tt)*) => {
        $crate::t_get!($($t)*).into_view()
    };
}

/// Returns untracked translation tranformed into [`leptos::View`]. Shorthand
/// for `i18n::get_keys_untracked().*.into_view()`.
#[macro_export]
macro_rules! t_view_untracked {
    ($($t:tt)*) => {
        $crate::t_get_untracked!($($t)*).into_view()
    };
}

/// Returns translation tranformed into [`String`]. Shorthand
/// for `td_string!(i18n::get_locale(), ...)`.
#[macro_export]
macro_rules! t_string {
    ($i18n:ident, $($t:tt)*) => {
        {
            #[allow(clippy::str_to_string)]
            $crate::__t_get!(td_string; get_locale; $i18n, $($t)*).to_string()
        }
    };
}

/// Returns translation tranformed into [`String`]. Shorthand
/// for `td_string!(i18n::get_locale_untracked(), ...)`.
#[macro_export]
macro_rules! t_string_untracked {
    ($i18n:ident, $($t:tt)*) => {
        {
            #[allow(clippy::str_to_string)]
            $crate::__t_get!(
                td_string; get_locale_untracked; $i18n, $($t)*
            ).to_string()
        }
    };
}
