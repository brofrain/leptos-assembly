#[macro_export]
macro_rules! __t_get {
    ($method:ident; $i18n:ident, $($keys:ident).+) => {
        paste::item! {
            $i18n.[< $method >]().$($keys).+
        }
    };

    ($method:ident; $i18n:ident, $($keys:ident).+, $($var_name:ident = $var_val:expr),+) => {
        paste::item! {
            $crate::__t_get!($method; $i18n, $($keys).+)
            .$(
                [< var_ $var_name >]($var_val)
            ).+
        }
    };

    ($method:ident; $i18n:ident, $($keys:ident).+, $($var:ident),+) => {
        paste::item! {
            $crate::__t_get!($method; $i18n, $($keys).+)
            .$(
                [< var_ $var >]($var)
            ).+
        }
    };
}

/// Shorthand for `i18n::get_keys().*`. Unlike [`t!`] does not return a signal.
#[macro_export]
macro_rules! t_get {
    ($i18n:ident, $($t:tt)*) => {
        $crate::__t_get!(get_keys; $i18n, $($t)*)
    }
}

/// Shorthand for `i18n::get_keys_untracked().*`.
#[macro_export]
macro_rules! t_get_untracked {
    ($i18n:ident, $($t:tt)*) => {
        $crate::__t_get!(get_keys_untracked; $i18n, $($t)*)
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

/// Returns translation tranformed into [`String`]. The conversion is relatively
/// expensive and all HTML tags inside the translation will be stripped.
#[macro_export]
macro_rules! t_string {
    ($($t:tt)*) => {
        leptos::view! { <span>{
            $crate::t_get!($($t)*)
        }</span> }.text_content().unwrap()
    };
}
