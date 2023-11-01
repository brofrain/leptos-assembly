#![feature(fn_traits)]
#![feature(unboxed_closures)]

#[macro_use]
extern crate client_globals;

pub mod confirm;
pub mod i18n;
pub mod id;
pub mod nprogress;
pub mod overlay;
pub mod panic_handler;
pub mod throttle;
pub mod toast;

flatten_pub_mod!(callback);
