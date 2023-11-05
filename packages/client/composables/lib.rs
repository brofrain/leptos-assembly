#![feature(fn_traits)]
#![feature(unboxed_closures)]

common::use_macros!();

pub mod confirm;
pub mod i18n;
pub mod id;
pub mod nprogress;
pub mod overlay;
pub mod panic_handler;
pub mod throttle;
pub mod toast;

flatten_pub_mod!(callback);
